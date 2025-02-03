use super::{NestedFieldSort, SortMode, SortOrder};
use crate::util::ShouldSkip;
use crate::Term;
use serde::Serialize;

/// Sorts search hits by other field values
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#sort-search-results>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct FieldSort {
    #[serde(skip)]
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    order: Option<SortOrder>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    mode: Option<SortMode>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    unmapped_type: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    format: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing: Option<Term>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    nested: Option<NestedFieldSort>,
}

impl FieldSort {
    /// Creates an instance of [FieldSort]
    pub fn new<T>(field: T) -> Self
    where
        T: ToString,
    {
        Self {
            field: field.to_string(),
            order: None,
            mode: None,
            unmapped_type: None,
            format: None,
            missing: None,
            nested: None,
        }
    }

    /// Creates an instance of [FieldSort] by ascending order
    pub fn ascending<T>(field: T) -> Self
    where
        T: ToString,
    {
        Self::new(field).order(SortOrder::Asc)
    }

    /// Creates an instance of [FieldSort] by descending order
    pub fn descending<T>(field: T) -> Self
    where
        T: ToString,
    {
        Self::new(field).order(SortOrder::Desc)
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
        T: ToString,
    {
        self.unmapped_type = Some(unmapped_type.to_string());
        self
    }

    /// Optional format for datetime sorts
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_ignoring_unmapped_fields>
    pub fn format<T>(mut self, format: T) -> Self
    where
        T: ToString,
    {
        self.format = Some(format.to_string());
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

    /// Sorts search hits by fields that are inside one or more nested objects.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#nested-sorting>
    pub fn nested(mut self, nested: NestedFieldSort) -> Self {
        self.nested = Some(nested);
        self
    }
}

impl IntoIterator for FieldSort {
    type Item = Self;

    type IntoIter = std::option::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Some(self).into_iter()
    }
}

serialize_keyed!(FieldSort: field);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::assert_serialize;
    use crate::{Query, SortSpecialField};

    #[test]
    fn serialization() {
        assert_serialize(FieldSort::new("test"), json!({"test": {}}));

        assert_serialize(
            FieldSort::new(SortSpecialField::Score),
            json!({"_score": {}}),
        );

        assert_serialize(
            FieldSort::ascending("field"),
            json!({ "field": { "order": "asc" } }),
        );

        assert_serialize(
            FieldSort::descending("field"),
            json!({ "field": { "order": "desc" } }),
        );

        assert_serialize(
            FieldSort::ascending("test")
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

        assert_serialize(
            FieldSort::ascending("offer.price")
                .order(SortOrder::Asc)
                .mode(SortMode::Avg)
                .nested(NestedFieldSort::path("offer").filter(Query::term("offer.color", "blue"))),
            json!({
                "offer.price": {
                    "mode": "avg",
                    "order": "asc",
                    "nested": {
                        "path": "offer",
                        "filter": {
                            "term": {
                                "offer.color": {
                                    "value": "blue"
                                }
                            }
                        }
                    }
                }
            }),
        );
    }
}
