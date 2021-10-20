//! Allows you to add one or more sorts on specific fields.
//! Each sort can be reversed as well.
//! The sort is defined on a per field level, with special field name for `_score` to sort by score, and `_doc` to sort by index order.
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/master/search-your-data.html>
use crate::ShouldSkip;
use serde::ser::{Serialize, SerializeMap, Serializer};

/// The order defaults to `desc` when sorting on the `_score`, and defaults to `asc` when sorting on anything else.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_sort_order>
#[derive(Debug, Clone, Serialize, PartialEq, PartialOrd)]
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
#[derive(Serialize, Clone, Debug, PartialEq, PartialOrd)]
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
#[derive(Serialize, Clone, Debug, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum SortMissing {
    /// Sorts missing fields first
    #[serde(rename = "_first")]
    First,

    /// Sorts missing field last
    #[serde(rename = "_last")]
    Last,

    /// Provide a custom scalar value for missing fields
    Custom(String),
}

impl From<String> for SortMissing {
    fn from(value: String) -> Self {
        SortMissing::Custom(value)
    }
}

impl From<&str> for SortMissing {
    fn from(value: &str) -> Self {
        SortMissing::Custom(value.into())
    }
}

/// Allows you to add one or more sorts on specific fields. Each sort can be reversed as well. The sort is defined on a per field level.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#sort-search-results>
#[derive(Serialize, Clone, Debug, PartialEq, PartialOrd)]
#[serde(untagged)]
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

impl From<String> for SortField {
    fn from(field: String) -> Self {
        SortField::Field(field)
    }
}

impl From<&str> for SortField {
    fn from(field: &str) -> Self {
        SortField::Field(field.into())
    }
}

/// Sorts search hits by other field values
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#sort-search-results>
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sort {
    /// Field to sort by
    pub field: SortField,

    /// Explicit order
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_sort_order>
    pub order: Option<SortOrder>,

    /// Sort mode for numeric fields
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_sort_mode_option>
    pub mode: Option<SortMode>,

    /// Fallback type if mapping is not defined
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_ignoring_unmapped_fields>
    pub unmapped_type: Option<String>,

    /// The missing parameter specifies how docs which are missing the sort field should be treated
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_missing_values>
    pub missing: Option<SortMissing>,
}

impl Serialize for Sort {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct InnerSort {
            #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
            order: Option<SortOrder>,

            #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
            mode: Option<SortMode>,

            #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
            unmapped_type: Option<String>,

            #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
            missing: Option<SortMissing>,
        }

        let mut map = serializer.serialize_map(Some(1))?;

        let field = match &self.field {
            SortField::Id => "_id",
            SortField::Score => "_score",
            SortField::Key => "_key",
            SortField::Count => "_count",
            SortField::Doc => "_doc",
            SortField::Field(field) => field.as_str(),
        };

        let value = InnerSort {
            order: self.order.clone(),
            mode: self.mode.clone(),
            unmapped_type: self.unmapped_type.clone(),
            missing: self.missing.clone(),
        };

        map.serialize_entry(&field, &value)?;
        map.end()
    }
}

impl Sort {
    /// Creates a new sort instance
    pub fn new(field: impl Into<SortField>) -> Self {
        Self {
            field: field.into(),
            order: None,
            mode: None,
            unmapped_type: None,
            missing: None,
        }
    }

    /// Sets sort order
    pub fn order(mut self, order: SortOrder) -> Self {
        self.order = Some(order);
        self
    }

    /// Sets sort mode
    pub fn mode(mut self, mode: SortMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Sets unmapped type
    pub fn unmapped_type(mut self, unmapped_type: impl Into<String>) -> Self {
        self.unmapped_type = Some(unmapped_type.into());
        self
    }

    /// Sets missing value
    pub fn missing(mut self, missing: impl Into<SortMissing>) -> Self {
        self.missing = Some(missing.into());
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
    fn sort_with_field_only_serializes_successfully() {
        let sort = Sort::new("test");

        let result = serde_json::to_string(&sort).unwrap();

        let expectation = r#"{"test":{}}"#;

        assert_eq!(result, expectation);
    }

    #[test]
    fn sort_with_all_attributes_serializes_successfully() {
        let sort = Sort::new("test")
            .order(SortOrder::Asc)
            .mode(SortMode::Max)
            .unmapped_type("long")
            .missing("miss");

        let result = serde_json::to_string(&sort).unwrap();

        let expectation =
            r#"{"test":{"order":"asc","mode":"max","unmapped_type":"long","missing":"miss"}}"#;

        assert_eq!(result, expectation);
    }
}
