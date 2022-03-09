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
