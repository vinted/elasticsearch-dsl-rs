/// Cluster statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClusterStatistics {
    /// Total number of touched clusters
    pub total: u32,

    /// Total number of successful clusters
    pub successful: u32,

    /// Total number of skipped clusters
    pub skipped: u32,
}
