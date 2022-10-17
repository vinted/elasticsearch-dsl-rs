/// Indicates how scores for matching child objects affect the root parent
/// document’s
/// [relevance score](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum NestedQueryScoreMode {
    /// Use the mean relevance score of all matching child objects.
    #[serde(rename = "avg")]
    Average,

    /// Uses the highest relevance score of all matching child objects.
    #[serde(rename = "max")]
    Maximum,

    /// Uses the lowest relevance score of all matching child objects.
    #[serde(rename = "min")]
    Minimum,

    /// Do not use the relevance scores of matching child objects. The query
    /// assigns parent documents a score of `0`.
    #[serde(rename = "none")]
    None,

    /// Add together the relevance scores of all matching child objects.
    #[serde(rename = "sum")]
    Sum,
}
