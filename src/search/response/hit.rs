use super::{Explanation, NestedIdentity, Source};
use crate::{util::ShouldSkip, InnerHitsResult, Map};
use serde::de::DeserializeOwned;
use serde_json::Value;

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
    #[serde(
        default,
        skip_serializing_if = "ShouldSkip::should_skip",
        rename = "_index"
    )]
    pub index: String,

    /// Document ID
    #[serde(
        default,
        skip_serializing_if = "ShouldSkip::should_skip",
        rename = "_id"
    )]
    pub id: String,

    /// Document score. [`None`] when documents are implicitly sorted by a
    /// field other than `_score`
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_score")]
    pub score: Option<f32>,

    /// Nested document identity
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_nested")]
    pub nested: Option<NestedIdentity>,

    /// Document source
    #[serde(
        skip_serializing_if = "ShouldSkip::should_skip",
        rename = "_source",
        default
    )]
    pub source: Source,

    /// Highlighted matches
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub highlight: Map<String, Vec<String>>,

    /// Inner hits
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub inner_hits: Map<String, InnerHitsResult>,

    /// Matched queries
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub matched_queries: Vec<String>,

    /// Values document was sorted by
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub sort: Vec<Value>,

    /// Field values for the documents. Need to be specified in the request
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub fields: Map<String, Value>,
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
