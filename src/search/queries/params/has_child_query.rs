/// Indicates how scores for matching child documents affect the root parent document’s relevance
/// score.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HasChildScoreMode {
    /// Do not use the relevance scores of matching child documents. The query assigns parent
    /// documents a score of 0.
    None,

    /// Use the mean relevance score of all matching child documents.
    Avg,

    /// Uses the highest relevance score of all matching child documents.
    Max,

    /// Uses the lowest relevance score of all matching child documents.
    Min,

    ///Add together the relevance scores of all matching child documents.
    Sum,
}
