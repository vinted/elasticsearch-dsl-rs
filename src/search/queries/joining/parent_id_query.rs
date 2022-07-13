use crate::search::*;
use crate::util::*;

/// Returns child documents joined to a specific parent document. You can use a join field mapping
/// to create parent-child relationships between documents in the same index.
///
/// To create parent_id query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::parent_id("test", 1);
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-ParentId-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct ParentIdQuery {
    r#type: String,

    id: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    ignore_unmapped: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`ParentIdQuery`]
    ///
    /// - `type` - Name of the child relationship mapped for the join field
    /// - `id` - ID of the parent document. The query will return child documents of this
    /// parent document.
    pub fn parent_id<T, U>(r#type: T, id: U) -> ParentIdQuery
    where
        T: ToString,
        U: ToString,
    {
        ParentIdQuery {
            r#type: r#type.to_string(),
            id: id.to_string(),
            ignore_unmapped: None,
            boost: None,
            _name: None,
        }
    }
}

impl ParentIdQuery {
    /// Indicates whether to ignore an unmapped `type` and not return any documents instead of an
    /// error. Defaults to `false`.
    ///
    /// If `false`, Elasticsearch returns an error if the `type` is unmapped.
    ///
    /// You can use this parameter to query multiple indices that may not contain the `type`.
    pub fn ignore_unmapped(mut self, ignore_unmapped: bool) -> Self {
        self.ignore_unmapped = Some(ignore_unmapped);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for ParentIdQuery {
    fn should_skip(&self) -> bool {
        self.r#type.should_skip() || self.id.should_skip()
    }
}

serialize_query!("parent_id": ParentIdQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::parent_id("my-child", 1),
            json!({
                "parent_id": {
                    "type": "my-child",
                    "id": "1"
                }
            }),
        );

        assert_serialize_query(
            Query::parent_id("my-child", 1)
                .boost(2)
                .name("test")
                .ignore_unmapped(true),
            json!({
                "parent_id": {
                    "type": "my-child",
                    "id": "1",
                    "ignore_unmapped": true,
                    "boost": 2,
                    "_name": "test"
                }
            }),
        );
    }
}
