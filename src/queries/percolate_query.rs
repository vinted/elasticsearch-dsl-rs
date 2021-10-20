use super::params::*;
use super::Query;
use crate::util::ShouldSkip;
use std::marker::PhantomData;

/// The `percolate` query can be used to match queries stored in an index. The percolate query
/// itself contains the document that will be used as query to match with the stored queries.
///
/// To percolate single document:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # use serde_json::json;
/// # let query =
/// PercolateQuery::new("field", json!({ "message": "search text" }));
/// ```
/// or
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
/// PercolateQuery::new("field", vec![json!({ "message": "search text" }), json!({ "message": "another search text" })]);
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # use serde_json::json;
/// # let query =
/// Query::percolate("field", vec![json!({ "message": "search text" }), json!({ "message": "another search text" })]);
/// ```
/// To percolate indexed document:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # use serde_json::json;
/// # let query1 =
/// PercolateQuery::new("field", PercolateLookup::new("index_name", "document_id"));
/// # let query2 =
/// PercolateQuery::new_lookup("field", "index_name", "document_id");
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # use serde_json::json;
/// # let query1 =
/// Query::percolate("field", PercolateLookup::new("index_name", "document_id"));
/// # let query2 =
/// Query::percolate_lookup("field", "index_name", "document_id");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-percolate-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PercolateQuery<T: PercolateMarker> {
    #[serde(skip)]
    phantom: PhantomData<T>,
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
    /// Creates an instance of [PercolateQuery](PercolateQuery)
    ///
    /// - `field` - The field of type `percolator` that holds the indexed queries
    /// - `source` - [Source](PercolateSource) to percolate
    pub fn percolate<T: PercolateMarker>(field: impl Into<String>, source: T) -> PercolateQuery<T> {
        PercolateQuery::new(field, source)
    }

    /// Creates an instance of [PercolateQuery](PercolateQuery)
    ///
    /// - `field` - The field of type `percolator` that holds the indexed queries
    /// - `index` - The index the document resides in
    /// - `id` - The id of the document to fetch
    pub fn percolate_lookup<S>(field: S, index: S, id: S) -> PercolateQuery<PercolateLookup>
    where
        S: Into<String>,
    {
        PercolateQuery::new_lookup(field, index, id)
    }
}

impl<T: PercolateMarker> PercolateQuery<T> {
    /// Creates an instance of [PercolateQuery](PercolateQuery)
    ///
    /// - `field` - The field of type `percolator` that holds the indexed queries
    /// - `source` - [Source](PercolateSource) to percolate
    pub fn new(field: impl Into<String>, source: T) -> Self {
        Self {
            phantom: PhantomData,
            inner: Inner {
                field: field.into(),
                source: source.into(),
                name: None,
            },
        }
    }

    /// The suffix to be used for the `_percolator_document_slot` field in case multiple `percolate`
    /// queries have been specified. This is an optional parameter
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
}

impl<T: PercolateMarker> ShouldSkip for PercolateQuery<T> {
    fn should_skip(&self) -> bool {
        false
    }
}

impl PercolateQuery<PercolateLookup> {
    /// Creates an instance of [PercolateQuery](PercolateQuery)
    ///
    /// - `field` - The field of type `percolator` that holds the indexed queries
    /// - `index` - The index the document resides in
    /// - `id` - The id of the document to fetch
    pub fn new_lookup<S>(field: S, index: S, id: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            phantom: PhantomData,
            inner: Inner {
                field: field.into(),
                name: None,
                source: PercolateSource::Lookup(PercolateLookup::new(index, id)),
            },
        }
    }

    /// Routing to be used to fetch document to percolate
    pub fn routing(mut self, routing: impl Into<String>) -> Self {
        if let PercolateSource::Lookup(ref mut source) = self.inner.source {
            source.routing = Some(routing.into());
        }
        self
    }

    /// Preference to be used to fetch document to percolate
    pub fn preference(mut self, preference: impl Into<String>) -> Self {
        if let PercolateSource::Lookup(ref mut source) = self.inner.source {
            source.preference = Some(preference.into());
        }
        self
    }

    /// The expected version of the document to be fetched
    pub fn version(mut self, version: impl Into<i64>) -> Self {
        if let PercolateSource::Lookup(ref mut source) = self.inner.source {
            source.version = Some(version.into());
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        new_with_required_fields(
            PercolateQuery::new("field_name", json!({"message": "lol"})),
            json!({
                "percolate": {
                    "field": "field_name",
                    "document": {
                        "message": "lol"
                    }
                }
            })
        );

        new_with_all_fields(
            PercolateQuery::new("field_name", json!({"message": "lol"})).name("toast"),
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

        new_multiple_with_required_fields(
            PercolateQuery::new("field_name", vec![json!({"message": "lol"})]),
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

        new_multiple_with_all_fields(
            PercolateQuery::new("field_name", vec![json!({"message": "lol"})]).name("toast"),
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

        new_lookup_with_required_fields(
            PercolateQuery::new_lookup("field_name", "index_name", "document_id"),
            json!({
                "percolate": {
                    "field": "field_name",
                    "index": "index_name",
                    "id": "document_id"
                }
            })
        );

        new_lookup_with_all_fields(
            PercolateQuery::new_lookup("field_name", "index_name", "document_id")
                .name("toast")
                .routing("routing_value")
                .preference("preference_value")
                .version(123),
            json!({
                "percolate": {
                    "field": "field_name",
                    "name": "toast",
                    "index": "index_name",
                    "id": "document_id",
                    "routing": "routing_value",
                    "preference": "preference_value",
                    "version": 123,
                }
            })
        );
    }
}
