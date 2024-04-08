//! Collapse search results
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/current/collapse-search-results.html>

use crate::util::*;
use crate::*;

/// Internal representation for collapse
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Collapse {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    inner_hits: Vec<InnerHits>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    max_concurrent_group_searches: Option<u64>,
}

impl Collapse {
    /// Creates an instance of [`Collapse`]
    ///
    /// - `field` - Field you wish to collapse on
    pub fn new<T>(field: T) -> Self
    where
        T: ToString,
    {
        Self {
            field: field.to_string(),
            max_concurrent_group_searches: None,
            inner_hits: Default::default(),
        }
    }

    /// Add `inner_hits` to Collapse
    pub fn inner_hits<T>(mut self, inner_hits: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<InnerHits>,
    {
        self.inner_hits
            .extend(inner_hits.into_iter().map(Into::into));
        self
    }

    /// Add `max_concurrent_group_searches` to Collapse
    pub fn max_concurrent_group_searches(mut self, max_concurrent_group_searches: u64) -> Self {
        self.max_concurrent_group_searches = Some(max_concurrent_group_searches);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(Collapse::new("field"), json!({ "field": "field" }));

        assert_serialize(
            Collapse::new("field")
                .max_concurrent_group_searches(10)
                .inner_hits([InnerHits::new()
                    .name("inner_hits")
                    .size(10)
                    .sort([FieldSort::descending("sort_field")])
                    .collapse("field_inner_collapse")]),
            json!({
                "field": "field",
                "inner_hits": [
                    {
                        "name": "inner_hits",
                        "size": 10,
                        "sort": [{"sort_field": {"order": "desc"}}],
                        "collapse": {
                            "field": "field_inner_collapse"
                        }
                    }
                ],
                "max_concurrent_group_searches": 10
            }),
        );

        assert_serialize(
            Collapse::new("field").inner_hits([
                InnerHits::new()
                    .name("inner_hits_0")
                    .size(10)
                    .sort([FieldSort::descending("sort_field")]),
                InnerHits::new()
                    .name("inner_hits_1")
                    .size(10)
                    .sort([FieldSort::descending("sort_field")]),
            ]),
            json!({
                "field": "field",
                "inner_hits": [
                    {
                        "name": "inner_hits_0",
                        "size": 10,
                        "sort": [{"sort_field": {"order": "desc"}}]
                    },
                    {
                        "name": "inner_hits_1",
                        "size": 10,
                        "sort": [{"sort_field": {"order": "desc"}}]
                    }
                ]
            }),
        );
    }
}
