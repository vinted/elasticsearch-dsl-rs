use super::ShardFailure;
use crate::util::ShouldSkip;

/// Number of shards touched with their states
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShardStatistics {
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
    pub failures: Option<ShardFailure>,
}
