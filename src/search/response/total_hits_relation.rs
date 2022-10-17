/// Relation to total number of matched documents
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
