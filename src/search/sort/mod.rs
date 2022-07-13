//! Allows you to add one or more sorts on specific fields.
//! Each sort can be reversed as well.
//! The sort is defined on a per field level, with special field name for `_score` to sort by score, and `_doc` to sort by index order.
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/master/search-your-data.html>

mod sort_field;
mod sort_missing;
mod sort_mode;
mod sort_order;
mod sort_special_field;

pub use self::sort_field::*;
pub use self::sort_missing::*;
pub use self::sort_mode::*;
pub use self::sort_order::*;
pub use self::sort_special_field::*;

use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// Sorts search hits by other field values
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#sort-search-results>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct Sort {
    #[serde(skip)]
    field: SortField,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    order: Option<SortOrder>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    mode: Option<SortMode>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    unmapped_type: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing: Option<Term>,
}

impl Sort {
    /// Creates an instance of [`Sort`]
    pub fn new<T>(field: T) -> Self
    where
        T: Into<SortField>,
    {
        Self {
            field: field.into(),
            order: None,
            mode: None,
            unmapped_type: None,
            missing: None,
        }
    }

    /// Explicit order
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_sort_order>
    pub fn order(mut self, order: SortOrder) -> Self {
        self.order = Some(order);
        self
    }

    /// Sort mode for numeric fields
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_sort_mode_option>
    pub fn mode(mut self, mode: SortMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Fallback type if mapping is not defined
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_ignoring_unmapped_fields>
    pub fn unmapped_type<T>(mut self, unmapped_type: T) -> Self
    where
        T: Into<String>,
    {
        self.unmapped_type = Some(unmapped_type.into());
        self
    }

    /// The missing parameter specifies how docs which are missing the sort field should be treated
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_missing_values>
    pub fn missing<T>(mut self, missing: T) -> Self
    where
        T: Serialize,
    {
        self.missing = Term::new(missing);
        self
    }
}

impl From<Sort> for Vec<Sort> {
    fn from(sort: Sort) -> Self {
        vec![sort]
    }
}

serialize_keyed!(Sort: field);

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
