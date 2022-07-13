use crate::search::*;
use crate::util::*;
use serde::Serialize;

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
#[serde(remote = "Self")]
pub struct PercolateQuery {
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
        T: Serialize,
    {
        let source = serde_json::to_value(source).unwrap_or_default();
        let source = if let Some(array) = source.as_array() {
            PercolateSource::Documents(array.to_vec())
        } else {
            PercolateSource::Document(source)
        };

        PercolateQuery {
            field: field.to_string(),
            source,
            name: None,
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
        self.name = Some(name.to_string());
        self
    }
}

impl ShouldSkip for PercolateQuery {
    fn should_skip(&self) -> bool {
        self.source.should_skip()
    }
}

serialize_query!("percolate": PercolateQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        #[derive(Serialize)]
        struct Source {
            id: i32,
            message: &'static str,
        }

        assert_serialize_query(
            Query::percolate(
                "field_name",
                Source {
                    id: 1,
                    message: "search text",
                },
            ),
            json!({
                "percolate": {
                    "field": "field_name",
                    "document": {
                        "id": 1,
                        "message": "search text",
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::percolate(
                "field_name",
                [Source {
                    id: 1,
                    message: "search text",
                }],
            ),
            json!({
                "percolate": {
                    "field": "field_name",
                    "documents": [
                        {
                            "id": 1,
                            "message": "search text",
                        }
                    ]
                }
            }),
        );

        assert_serialize_query(
            Query::percolate("field_name", json!({"message": "lol"})),
            json!({
                "percolate": {
                    "field": "field_name",
                    "document": {
                        "message": "lol"
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::percolate("field_name", json!({"message": "lol"})).name("toast"),
            json!({
                "percolate": {
                    "field": "field_name",
                    "name": "toast",
                    "document": {
                        "message": "lol"
                    }
                }
            }),
        );

        assert_serialize_query(
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
            }),
        );

        assert_serialize_query(
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
            }),
        );
    }
}
