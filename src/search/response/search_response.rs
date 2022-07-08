use super::{ClusterStatistics, HitsMetadata, ShardStatistics};
use crate::util::ShouldSkip;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;

/// Search response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// The time that it took Elasticsearch to process the query
    pub took: u32,

    /// Indicates whether there have been timed-out shards, if `true` - responses are partial
    pub timed_out: bool,

    /// Indicates if search has been terminated early
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub terminated_early: Option<bool>,

    /// Scroll Id
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub scroll_id: Option<String>,

    /// Dynamically fetched fields
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub fields: HashMap<String, Value>,

    /// Point in time Id
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub pit_id: Option<String>,

    /// Number of reduce phases
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub num_reduce_phases: Option<u64>,

    /// Maximum document score. [`None`] when documents are implicitly sorted
    /// by a field other than `_score`
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub max_score: Option<f32>,

    /// Number of clusters touched with their states
    #[serde(rename = "_clusters")]
    pub clusters: Option<ClusterStatistics>,

    /// Number of shards touched with their states
    #[serde(rename = "_shards")]
    pub shards: ShardStatistics,

    /// Search hits
    pub hits: HitsMetadata,

    /// Search aggregations
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub aggregations: Option<Value>,
}

impl SearchResponse {
    /// A shorthand for retrieving the _source for each hit
    pub fn documents<T>(&self) -> Result<Vec<T>, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        self.hits.hits.iter().map(|hit| hit.source()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Hit, Source, TotalHits, TotalHitsRelation};

    #[test]
    fn deserializes_successfully() {
        let json = json!({
          "took": 6,
          "timed_out": false,
          "_shards": {
            "total": 10,
            "successful": 5,
            "skipped": 3,
            "failed": 2
          },
          "hits": {
            "total": {
              "value": 10000,
              "relation": "gte"
            },
            "max_score": 1.0,
            "hits": [
              {
                "_index": "_index",
                "_type": "_doc",
                "_id": "123",
                "_score": 1.0
              }
            ]
          }
        });

        let actual: SearchResponse = serde_json::from_value(json).unwrap();

        let expected = SearchResponse {
            took: 6,
            timed_out: false,
            shards: ShardStatistics {
                total: 10,
                successful: 5,
                skipped: 3,
                failed: 2,
                failures: Default::default(),
            },
            hits: HitsMetadata {
                total: Some(TotalHits {
                    value: 10_000,
                    relation: TotalHitsRelation::GreaterThanOrEqualTo,
                }),
                max_score: Some(1.0),
                hits: vec![Hit {
                    explanation: None,
                    nested: None,
                    index: Some("_index".into()),
                    id: "123".into(),
                    score: Some(1.0),
                    source: Source::from_string("null".to_string()).unwrap(),
                    highlight: Default::default(),
                    inner_hits: Default::default(),
                    matched_queries: Default::default(),
                    sort: Default::default(),
                    fields: Default::default(),
                }],
            },
            aggregations: None,
            terminated_early: None,
            scroll_id: None,
            fields: Default::default(),
            pit_id: None,
            num_reduce_phases: None,
            max_score: None,
            clusters: None,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_documents() {
        let json = json!({
          "took": 6,
          "timed_out": false,
          "_shards": {
            "total": 10,
            "successful": 5,
            "skipped": 3,
            "failed": 2
          },
          "hits": {
            "total": {
              "value": 10000,
              "relation": "gte"
            },
            "max_score": 1.0,
            "hits": [
              {
                "_index": "_index",
                "_type": "_doc",
                "_id": "123",
                "_score": 1.0,
                "_source": {
                    "id": 123,
                    "title": "test",
                    "user_id": 456,
                }
              }
            ]
          }
        });

        #[derive(Debug, PartialEq, Deserialize)]
        struct Document {
            id: i32,
            title: String,
            user_id: Option<i32>,
        }

        let subject: SearchResponse = serde_json::from_value(json).unwrap();
        let subject = subject.documents::<Document>().unwrap();

        let expectation = [Document {
            id: 123,
            title: "test".to_string(),
            user_id: Some(456),
        }];

        assert_eq!(subject, expectation);
    }
}
