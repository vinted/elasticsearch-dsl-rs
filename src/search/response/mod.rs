use crate::util::*;
use serde::de::DeserializeOwned;
use serde_json::{value::RawValue, Value};
use std::collections::HashMap;

/// A source structure with delayed serde
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Source(Box<RawValue>);

impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl ShouldSkip for Source {
    fn should_skip(&self) -> bool {
        self.eq(&Source::default())
    }
}

impl Source {
    /// Parses document source into a concrete type
    pub fn parse<T>(&self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        serde_json::from_str(self.0.get())
    }

    /// Creates source from a string
    pub fn from_string(value: String) -> Result<Self, serde_json::Error> {
        RawValue::from_string(value).map(Self)
    }
}

/// Search response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// The time that it took Elasticsearch to process the query
    pub took: u32,

    /// Indicates whether there have been timed-out shards, if `true` - responses are partial
    pub timed_out: bool,

    /// Number of shards touched with their states
    #[serde(rename = "_shards")]
    pub shards: Shards,

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
pub struct HitsMetadata {
    /// Total number of matched documents
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub total: Option<TotalHits>,

    /// Maximum document score. [`None`] when documents are implicitly sorted
    /// by a field other than `_score`
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub max_score: Option<f32>,

    /// Matched hits
    #[serde(default = "Vec::new")]
    pub hits: Vec<Hit>,
}

/// Represents a single matched document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Hit {
    /// Search explanation
    #[serde(
        skip_serializing_if = "ShouldSkip::should_skip",
        rename = "_explanation"
    )]
    pub explanation: Option<Explanation>,

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
    #[serde(
        skip_serializing_if = "ShouldSkip::should_skip",
        rename = "_source",
        default
    )]
    pub source: Source,

    /// Highlighted matches
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub highlight: HashMap<String, Vec<String>>,

    /// Inner hits
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub inner_hits: HashMap<String, InnerHitsItemsHits>,

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

impl Hit {
    /// Parses document source into a concrete type
    pub fn source<T>(&self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        self.source.parse()
    }
}

/// Score explanation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Explanation {
    /// Cumulative score description
    pub description: String,

    /// Cumulative score
    pub value: f64,

    /// Score details
    #[serde(default)]
    pub details: Vec<Explanation>,
}

/// Represents inner hits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InnerHitsItems {
    /// The actual inner hits
    pub hits: InnerHitsItemsHits,
}

/// Matched inner hits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InnerHitsItemsHits {
    /// Total number of matched documents
    #[serde(default)]
    pub total: Option<TotalHits>,

    /// Maximum document score. [`None`] when documents are implicitly sorted
    /// by a field other than `_score`
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub max_score: Option<f32>,

    /// Matched hits
    #[serde(default = "Vec::new")]
    pub hits: Vec<InnerHit>,
}

/// Represents a single matched inner hit document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InnerHit {
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
    pub nested: Option<NestedIdentity>,

    /// Document source
    #[serde(
        skip_serializing_if = "ShouldSkip::should_skip",
        rename = "_source",
        default
    )]
    pub source: Source,

    /// Matched queries
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub matched_queries: Vec<String>,

    /// Values document was sorted by
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub sort: Vec<Value>,
}

impl InnerHit {
    /// Parses document source into a concrete type
    pub fn source<T>(&self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        self.source.parse()
    }
}

/// Total number of matched documents
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct TotalHits {
    /// Number of total documents
    pub value: u64,

    /// Relation to total number of matched documents
    pub relation: TotalHitsRelation,
}

impl TotalHits {
    /// Create default Total instance
    pub fn new(value: Option<u64>) -> Self {
        Self {
            value: value.unwrap_or(0),
            relation: TotalHitsRelation::Equal,
        }
    }
}

/// Nested document metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedIdentity {
    /// Field
    pub field: String,

    /// Offset
    pub offset: u64,

    /// Nested document metadata
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_nested")]
    pub nested: Option<Box<NestedIdentity>>,
}

/// Relation to total number of matched documents
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum TotalHitsRelation {
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
            shards: Shards {
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
