use crate::util::*;

/// Values that can be percolated
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PercolateSource {
    /// A document
    Document(serde_json::Value),

    /// A collection of documents
    Documents(Vec<serde_json::Value>),
}

impl From<serde_json::Value> for PercolateSource {
    fn from(document: serde_json::Value) -> Self {
        Self::Document(document)
    }
}

impl From<Vec<serde_json::Value>> for PercolateSource {
    fn from(documents: Vec<serde_json::Value>) -> Self {
        Self::Documents(documents)
    }
}

impl<const N: usize> From<[serde_json::Value; N]> for PercolateSource {
    fn from(value: [serde_json::Value; N]) -> Self {
        Self::Documents(value.to_vec())
    }
}

impl ShouldSkip for PercolateSource {
    fn should_skip(&self) -> bool {
        match self {
            PercolateSource::Document(document) => !document.is_object(),
            PercolateSource::Documents(documents) => {
                documents.is_empty() || documents.iter().any(|document| !document.is_object())
            }
        }
    }
}
