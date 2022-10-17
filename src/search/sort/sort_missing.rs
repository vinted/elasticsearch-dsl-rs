/// The `missing` parameter specifies how docs which are missing the sort field should be treated:
///
/// The `missing` value can be set to `_last`, `_first`, or a custom value (that will be used for missing docs as the sort value). The default is `_last`.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_missing_values>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SortMissing {
    /// Sorts missing fields first
    #[serde(rename = "_first")]
    First,

    /// Sorts missing field last
    #[serde(rename = "_last")]
    Last,
}
