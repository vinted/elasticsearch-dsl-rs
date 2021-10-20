use crate::ShouldSkip;

/// Marker trait for [percolate query](crate::PercolateQuery) values
pub trait PercolateMarker: Into<PercolateSource> {}

impl PercolateMarker for serde_json::Value {}
impl PercolateMarker for Vec<serde_json::Value> {}
impl PercolateMarker for PercolateLookup {}

/// A document to percolate
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PercolateDocument {
    document: serde_json::Value,
}

/// A collection of documents to percolate
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PercolateDocuments {
    documents: Vec<serde_json::Value>,
}

/// Percolate document from another index
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PercolateLookup {
    pub(crate) index: String,

    pub(crate) id: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub(crate) routing: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub(crate) preference: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub(crate) version: Option<i64>,
}

impl PercolateLookup {
    /// Constructs percolate lookup instance
    pub fn new<S>(index: S, id: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            index: index.into(),
            id: id.into(),
            routing: None,
            preference: None,
            version: None,
        }
    }
}

/// Values that can be percolated
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PercolateSource {
    /// A document
    Document(PercolateDocument),

    /// A collection of documents
    Documents(PercolateDocuments),

    /// An indexed document
    Lookup(PercolateLookup),
}

impl From<serde_json::Value> for PercolateSource {
    fn from(document: serde_json::Value) -> Self {
        Self::Document(PercolateDocument { document })
    }
}

impl From<Vec<serde_json::Value>> for PercolateSource {
    fn from(documents: Vec<serde_json::Value>) -> Self {
        Self::Documents(PercolateDocuments { documents })
    }
}

impl From<PercolateLookup> for PercolateSource {
    fn from(lookup: PercolateLookup) -> Self {
        Self::Lookup(lookup)
    }
}
