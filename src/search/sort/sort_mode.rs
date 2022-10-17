/// Elasticsearch supports sorting by array or multi-valued fields. The `mode` option controls what array value is picked for sorting the document it belongs to.
///
/// The default sort mode in the ascending sort order is `min` — the lowest value is picked. The default sort mode in the descending order is `max` — the highest value is picked.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_sort_mode_option>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortMode {
    /// Pick the lowest value.
    Min,

    /// Pick the highest value.
    Max,

    /// Use the sum of all values as sort value.\
    /// Only applicable for number based array fields.
    Sum,

    /// Use the average of all values as sort value.\
    /// Only applicable for number based array fields.
    Avg,

    /// Use the median of all values as sort value.\
    /// Only applicable for number based array fields.
    Median,
}
