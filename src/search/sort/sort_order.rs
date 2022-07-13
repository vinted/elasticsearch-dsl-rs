/// The order defaults to `desc` when sorting on the `_score`, and defaults to `asc` when sorting on anything else.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_sort_order>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    /// Sort in ascending order
    Asc,

    /// Sort in descending order
    Desc,
}
