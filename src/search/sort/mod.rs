//! Allows you to add one or more sorts on specific fields.
//! Each sort can be reversed as well.
//! The sort is defined on a per field level, with special field name for `_score` to sort by score, and `_doc` to sort by index order.
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/master/search-your-data.html>
use crate::search::*;
use crate::util::*;
use serde::ser::{Serialize, Serializer};

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

/// The `missing` parameter specifies how docs which are missing the sort field should be treated:
///
/// The `missing` value can be set to `_last`, `_first`, or a custom value (that will be used for missing docs as the sort value). The default is `_last`.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_missing_values>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortMissing {
    /// Sorts missing fields first
    First,

    /// Sorts missing field last
    Last,

    /// Provide a custom term for missing fields
    Custom(Term),
}

impl Serialize for SortMissing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::First => "_first".serialize(serializer),
            Self::Last => "_last".serialize(serializer),
            Self::Custom(field) => field.serialize(serializer),
        }
    }
}

impl<T> From<T> for SortMissing
where
    T: Into<Term>,
{
    fn from(value: T) -> Self {
        Self::Custom(value.into())
    }
}

/// Allows you to add one or more sorts on specific fields. Each sort can be reversed as well. The sort is defined on a per field level.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#sort-search-results>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortField {
    /// Sort by `_id` field
    Id,

    /// Sort by `_score`
    Score,

    /// Sort by key within aggregations
    Key,

    /// Sort by count within aggregations,
    Count,

    /// Sort by index order
    Doc,

    /// Sorts by a given field name
    Field(String),
}

impl Serialize for SortField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Id => "_id".serialize(serializer),
            Self::Score => "_score".serialize(serializer),
            Self::Key => "_key".serialize(serializer),
            Self::Count => "_count".serialize(serializer),
            Self::Doc => "_doc".serialize(serializer),
            Self::Field(field) => field.serialize(serializer),
        }
    }
}

impl<T> From<T> for SortField
where
    T: ToString,
{
    fn from(value: T) -> Self {
        Self::Field(value.to_string())
    }
}

/// Sorts search hits by other field values
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#sort-search-results>
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Sort(KeyValuePair<SortField, SortInner>);

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
struct SortInner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    order: Option<SortOrder>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    mode: Option<SortMode>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    unmapped_type: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing: Option<SortMissing>,
}

impl Sort {
    /// Creates an instance of [`Sort`]
    pub fn new<T>(field: T) -> Self
    where
        T: Into<SortField>,
    {
        Self(KeyValuePair::new(field.into(), Default::default()))
    }

    /// Explicit order
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_sort_order>
    pub fn order(mut self, order: SortOrder) -> Self {
        self.0.value.order = Some(order);
        self
    }

    /// Sort mode for numeric fields
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_sort_mode_option>
    pub fn mode(mut self, mode: SortMode) -> Self {
        self.0.value.mode = Some(mode);
        self
    }

    /// Fallback type if mapping is not defined
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_ignoring_unmapped_fields>
    pub fn unmapped_type<T>(mut self, unmapped_type: T) -> Self
    where
        T: Into<String>,
    {
        self.0.value.unmapped_type = Some(unmapped_type.into());
        self
    }

    /// The missing parameter specifies how docs which are missing the sort field should be treated
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_missing_values>
    pub fn missing<T>(mut self, missing: T) -> Self
    where
        T: Into<SortMissing>,
    {
        self.0.value.missing = Some(missing.into());
        self
    }
}

impl From<Sort> for Vec<Sort> {
    fn from(sort: Sort) -> Self {
        vec![sort]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(Sort::new("test"), json!({"test": {}}));
        assert_serialize(Sort::new(SortField::Id), json!({"_id": {}}));
        assert_serialize(
            Sort::new("test")
                .order(SortOrder::Asc)
                .mode(SortMode::Max)
                .unmapped_type("long")
                .missing("miss"),
            json!({
                "test": {
                    "order": "asc",
                    "mode": "max",
                    "unmapped_type": "long",
                    "missing": "miss",
                }
            }),
        );
    }
}
