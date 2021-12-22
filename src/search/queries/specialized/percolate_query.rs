use crate::search::*;
use crate::util::*;

/// The `percolate` query can be used to match queries stored in an index. The percolate query
/// itself contains the document that will be used as query to match with the stored queries.
///
/// To percolate single document:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # use serde_json::json;
/// # let query =
/// Query::percolate("field", json!({ "message": "search text" }));
/// ```
/// To percolate multiple documents:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # use serde_json::json;
/// # let query =
/// Query::percolate("field", vec![json!({ "message": "search text" }), json!({ "message": "another search text" })]);
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-percolate-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PercolateQuery {
    #[serde(rename = "percolate")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    name: Option<String>,

    #[serde(flatten)]
    source: PercolateSource,
}

impl Query {
    /// Creates an instance of [`PercolateQuery`]
    ///
    /// - `field` - The field of type `percolator` that holds the indexed queries
    /// - `source` - [Source](PercolateSource) to percolate
    pub fn percolate<S, T>(field: S, source: T) -> PercolateQuery
    where
        S: ToString,
        T: Into<PercolateSource>,
    {
        PercolateQuery {
            inner: Inner {
                field: field.to_string(),
                source: source.into(),
                name: None,
            },
        }
    }
}

impl PercolateQuery {
    /// The suffix to be used for the `_percolator_document_slot` field in case multiple `percolate`
    /// queries have been specified. This is an optional parameter
    pub fn name<S>(mut self, name: S) -> Self
    where
        S: ToString,
    {
        self.inner.name = Some(name.to_string());
        self
    }
}

impl ShouldSkip for PercolateQuery {
    fn should_skip(&self) -> bool {
        self.inner.source.should_skip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        single_with_required_fields(
            Query::percolate("field_name", json!({"message": "lol"})),
            json!({
                "percolate": {
                    "field": "field_name",
                    "document": {
                        "message": "lol"
                    }
                }
            })
        );

        single_with_all_fields(
            Query::percolate("field_name", json!({"message": "lol"})).name("toast"),
            json!({
                "percolate": {
                    "field": "field_name",
                    "name": "toast",
                    "document": {
                        "message": "lol"
                    }
                }
            })
        );

        multiple_with_required_fields(
            Query::percolate("field_name", [json!({"message": "lol"})]),
            json!({
                "percolate": {
                    "field": "field_name",
                    "documents": [
                        {
                            "message": "lol"
                        }
                    ]
                }
            })
        );

        multiple_with_all_fields(
            Query::percolate("field_name", [json!({"message": "lol"})]).name("toast"),
            json!({
                "percolate": {
                    "field": "field_name",
                    "name": "toast",
                    "documents": [
                        {
                            "message": "lol"
                        }
                    ]
                }
            })
        );
    }
}
