use crate::{util::ShouldSkip, Query};

/// Sorts search hits by fields that are inside one or more nested objects.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#nested-sorting>
#[derive(Default, Clone, PartialEq, Debug, Serialize)]
pub struct NestedFieldSort {
    path: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    filter: Option<Query>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    max_children: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    nested: Option<Box<NestedFieldSort>>,
}

impl NestedFieldSort {
    /// Creates an instance of [NestedFieldSort]
    pub fn path<T>(path: T) -> Self
    where
        T: ToString,
    {
        Self {
            path: path.to_string(),
            filter: None,
            max_children: None,
            nested: None,
        }
    }

    /// A filter that the inner objects inside the nested path should match with in order for its field values to be taken into account by sorting.
    pub fn filter<T>(mut self, filter: T) -> Self
    where
        T: Into<Option<Query>>,
    {
        self.filter = filter.into();
        self
    }

    /// The maximum number of children to consider per root document when picking the sort value.
    pub fn max_children(mut self, max_children: u32) -> Self {
        self.max_children = Some(max_children);
        self
    }

    /// Same as top-level nested but applies to another nested path within the current nested object.
    pub fn nested(mut self, nested: NestedFieldSort) -> Self {
        self.nested = Some(Box::new(nested));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{util::assert_serialize, Query};

    #[test]
    fn serialization() {
        // custom tests
        assert_serialize(NestedFieldSort::path("offer"), json!({  "path": "offer" }));

        assert_serialize(
            NestedFieldSort::path("offer").max_children(2),
            json!({  "path": "offer", "max_children": 2 }),
        );

        // based on examples from Elasticsearch documentation
        assert_serialize(
            NestedFieldSort::path("offer").filter(Query::term("offer.color", "blue")),
            json!({
               "path": "offer",
               "filter": {
                  "term" : { "offer.color" : {"value": "blue"} }
               }
            }),
        );

        assert_serialize(
            NestedFieldSort::path("parent")
                .filter(Query::range("parent.age").gte(21))
                .nested(
                    NestedFieldSort::path("parent.child")
                        .filter(Query::r#match("parent.child.name", "matt")),
                ),
            json!({
                "path": "parent",
                "filter": {
                   "range": {"parent.age": {"gte": 21}}
                },
                "nested": {
                   "path": "parent.child",
                   "filter": {
                      "match": {"parent.child.name": {"query": "matt"}}
                   }
                }
            }),
        );
    }
}
