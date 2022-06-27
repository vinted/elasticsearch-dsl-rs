use crate::util::*;
use serde::de::DeserializeOwned;
use serde_json::{value::RawValue, Value};
use std::collections::HashMap;

/// Boxed raw value
#[derive(Clone, Serialize, Deserialize)]
pub struct BoxedRawValue(Box<RawValue>);

impl std::fmt::Debug for BoxedRawValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for BoxedRawValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq for BoxedRawValue {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

/// Search response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse<H = Value> {
    /// The time that it took Elasticsearch to process the query
    pub took: u32,

    /// Indicates whether there have been timed-out shards, if `true` - responses are partial
    pub timed_out: bool,

    /// Number of shards touched with their states
    #[serde(rename = "_shards")]
    pub shards: Shards,

    /// Search hits
    pub hits: Hits<H>,

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
pub struct Hits<H> {
    /// Total number of matched documents
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub total: Option<Total>,

    /// Maximum document score. [`None`] when documents are implicitly sorted
    /// by a field other than `_score`
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub max_score: Option<f32>,

    /// Matched hits
    #[serde(default = "Vec::new")]
    pub hits: Vec<Hit<H>>,
}

/// Represents a single matched document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Hit<H> {
    /// Document index
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_index")]
    pub index: Option<String>,

    /// Document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// Document score. [`None`] when documents are implicitly sorted by a
    /// field other than `_score`
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_score")]
    pub score: Option<f32>,

    /// Document source
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_source")]
    pub source: Option<H>,

    /// Highlighted matches
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub highlight: HashMap<String, Vec<String>>,

    /// Inner hits
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub inner_hits: HashMap<String, BoxedRawValue>,

    /// Matched queries
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub matched_queries: Vec<String>,

    /// Values document was sorted by
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub sort: Vec<Value>,

    /// Field values for the documents. Need to be specified in the request
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub fields: std::collections::BTreeMap<String, Value>,
}

impl<H> Hit<H> {
    /// Gets inner hit by the key
    pub fn inner_hit<IH>(&self, key: &str) -> Option<InnerHitsResult<IH>>
    where
        IH: DeserializeOwned,
    {
        let value = self.inner_hits.get(key)?;

        serde_json::from_str(value.0.get()).ok()
    }
}

/// Represents inner hits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InnerHitsResult<H> {
    /// The actual inner hits
    pub hits: HitsMetadata<H>,
}

/// Matched inner hits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HitsMetadata<H> {
    /// Total number of matched documents
    #[serde(default)]
    pub total: Option<Total>,

    /// Maximum document score. [`None`] when documents are implicitly sorted
    /// by a field other than `_score`
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub max_score: Option<f32>,

    /// Matched hits
    #[serde(default = "Vec::new")]
    pub hits: Vec<InnerHit<H>>,
}

/// Represents a single matched inner hit document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InnerHit<H> {
    /// Document index
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_index")]
    pub index: Option<String>,

    /// Document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// Document score. [`None`] when documents are implicitly sorted by a
    /// field other than `_score`
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_score")]
    pub score: Option<f32>,

    /// Nested document metadata
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_nested")]
    pub nested: Option<Nested>,

    /// Document source
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_source")]
    pub source: Option<H>,

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
    fn deserializes_successfully() {
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
                    inner_hits: Default::default(),
                    matched_queries: Default::default(),
                    sort: Default::default(),
                    fields: Default::default(),
                }],
            },
            aggregations: None,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn reads_inner_hit_successfully() {
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
                    "value": 1,
                    "relation": "eq"
                },
                "max_score": 1.0,
                "hits": [
                    {
                        "_index": "test",
                        "_id": "1",
                        "_score": 1.0,
                        "inner_hits": {
                            "comments": {
                                "hits": {
                                    "total": {
                                        "value": 1,
                                        "relation": "eq"
                                    },
                                    "max_score": 1.0,
                                    "hits": [
                                        {
                                            "_index": "test",
                                            "_id": "1",
                                            "_nested": {
                                                "field": "comments",
                                                "offset": 1
                                            },
                                            "_score": 1.0,
                                            "_source": {
                                                "author": "nik9000",
                                                "number": 2
                                            }
                                        }
                                    ]
                                }
                            }
                        }
                    }
                ]
            }
        });

        let actual: SearchResponse = serde_json::from_value(json).unwrap();

        let inner_hits = actual.hits.hits[0].inner_hit("comments").unwrap();

        let expected = HitsMetadata {
            total: Some(Total {
                value: 1,
                relation: Relation::Equal,
            }),
            max_score: Some(1.0),
            hits: vec![InnerHit {
                index: Some("test".to_string()),
                id: "1".to_string(),
                score: Some(1.0),
                nested: Some(Nested {
                    field: "comments".to_string(),
                    offset: 1,
                }),
                source: Some(json!({ "author": "nik9000", "number": 2 })),
                matched_queries: Default::default(),
                sort: Default::default(),
            }],
        };

        assert_eq!(inner_hits.hits, expected);
    }
}
