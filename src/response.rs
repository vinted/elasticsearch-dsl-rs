use crate::ShouldSkip;
use serde_json::Value;
use std::collections::HashMap;

/// Search response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse<H = Value, IH = Value> {
    /// The time that it took Elasticsearch to process the query
    pub took: u32,

    /// Indicates whether there have been timed-out shards, if `true` - responses are partial
    pub timed_out: bool,

    /// Number of shards touched with their states
    #[serde(rename = "_shards")]
    pub shards: Shards,

    /// Search hits
    pub hits: Hits<H, IH>,

    /// Search aggregations
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub aggregations: Option<Value>,
}

/// Number of shards touched with their states
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Shards {
    /// Total number of touched shards
    pub total: u32,

    /// Total number of successful shards
    pub successful: u32,

    /// Total number of skipped shards
    pub skipped: u32,

    /// Total number of failed shards
    pub failed: u32,

    /// Partial response failures
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub failures: Option<Value>,
}

/// Matched hits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Hits<H, IH> {
    /// Total number of matched documents
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub total: Option<Total>,

    /// Maximum document score. [`None`](std::option::Option::None) when
    /// documents are implicitly sorted by a field other than `_score`
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub max_score: Option<f32>,

    /// Matched hits
    #[serde(default = "Vec::new")]
    pub hits: Vec<Hit<H, IH>>,
}

/// Represents a single matched document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Hit<H, IH> {
    /// Document index
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_index")]
    pub index: Option<String>,

    /// Document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// Document score. [`None`](std::option::Option::None) when documents are
    /// implicitly sorted by a field other than `_score`
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_score")]
    pub score: Option<f32>,

    /// Document source
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_source")]
    pub source: Option<H>,

    /// Highlighted matches
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub highlight: HashMap<String, Vec<String>>,

    /// Inner hits
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub inner_hits: Option<InnerHitsResponse<IH>>,

    /// Matched queries
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub matched_queries: Vec<String>,

    /// Values document was sorted by
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub sort: Vec<Value>,
}

/// Represents inner hits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InnerHitsResponse<IH> {
    /// Inner hits items
    pub items: InnerHitsItems<IH>,
}

/// Represents inner hits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InnerHitsItems<IH> {
    /// The actual inner hits
    pub hits: InnerHitsItemsHits<IH>,
}

/// Matched inner hits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InnerHitsItemsHits<IH> {
    /// Total number of matched documents
    #[serde(default)]
    pub total: Option<Total>,

    /// Maximum document score. [`None`](std::option::Option::None) when
    /// documents are implicitly sorted by a field other than `_score`
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub max_score: Option<f32>,

    /// Matched hits
    #[serde(default = "Vec::new")]
    pub hits: Vec<InnerHit<IH>>,
}

/// Represents a single matched inner hit document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InnerHit<IH> {
    /// Document index
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_index")]
    pub index: Option<String>,

    /// Document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// Document score. [`None`](std::option::Option::None) when documents are
    /// implicitly sorted by a field other than `_score`
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_score")]
    pub score: Option<f32>,

    /// Nested document metadata
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_nested")]
    pub nested: Option<Nested>,

    /// Document source
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_source")]
    pub source: Option<IH>,

    /// Matched queries
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub matched_queries: Vec<String>,

    /// Values document was sorted by
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub sort: Vec<Value>,
}

/// Total number of matched documents
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Total {
    /// Number of total documents
    pub value: u64,

    /// Relation to total number of matched documents
    pub relation: Relation,
}

impl Total {
    /// Create default Total instance
    pub fn new(value: Option<u64>) -> Self {
        Total {
            value: value.unwrap_or(0),
            relation: Relation::Equal,
        }
    }
}

/// Nested document metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Nested {
    /// Field
    pub field: String,

    /// Offset
    pub offset: u64,
}

/// Relation to total number of matched documents
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum Relation {
    /// When `track_total_hits` is `false` (default), Elasticsearch returns that
    /// there have been more than 10,000 documents
    #[serde(rename = "gte")]
    GreaterThanOrEqualTo,

    /// When there are less than 10,000 documents or `track_total_hits` is set
    /// to `true`, exact number of matched documents will be brought back
    #[serde(rename = "eq")]
    Equal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_successfully() {
        let json = serde_json::json!({
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
            shards: Shards {
                total: 10,
                successful: 5,
                skipped: 3,
                failed: 2,
                failures: Default::default(),
            },
            hits: Hits {
                total: Some(Total {
                    value: 10_000,
                    relation: Relation::GreaterThanOrEqualTo,
                }),
                max_score: Some(1.0),
                hits: vec![Hit {
                    index: Some("_index".into()),
                    id: "123".into(),
                    score: Some(1.0),
                    source: None,
                    highlight: Default::default(),
                    inner_hits: None,
                    matched_queries: Default::default(),
                    sort: Default::default(),
                }],
            },
            aggregations: None,
        };

        assert_eq!(actual, expected);
    }
}
