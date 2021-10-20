/// Indicates how the range query matches values for range fields.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Relation {
    /// Matches documents with a range field value that intersects the query’s range.
    Intersects,

    /// Matches documents with a range field value that entirely contains the query’s range.
    Contains,

    /// Matches documents with a range field value entirely within the query’s range.
    Within,
}
