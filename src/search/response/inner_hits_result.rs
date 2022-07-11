use super::HitsMetadata;

/// Represents inner hits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InnerHitsResult {
    /// The actual inner hits
    pub hits: HitsMetadata,
}
