use crate::search::*;
use crate::util::*;
use std::convert::TryInto;

/// In order to percolate a newly indexed document, the [percolate](PercolateLookupQuery) query can
/// be used. Based on the response from an index request, the `_id` and other meta information can
/// be used to immediately percolate the newly added document.
///
/// To percolate indexed document:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # use serde_json::json;
/// # let query =
/// Query::percolate_lookup("field", "index_name", "document_id");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-percolate-query.html#_percolating_an_existing_document>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PercolateLookupQuery {
    #[serde(rename = "percolate")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    field: String,

    index: String,

    id: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    routing: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    preference: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    version: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    name: Option<String>,
}

impl Query {
    /// Creates an instance of [`PercolateLookupQuery`]
    ///
    /// - `field` - The field of type `percolator` that holds the indexed queries
    /// - `index` - The index the document resides in
    /// - `id` - The id of the document to fetch
    pub fn percolate_lookup<S, T, U>(field: S, index: T, id: U) -> PercolateLookupQuery
    where
        S: ToString,
        T: ToString,
        U: ToString,
    {
        PercolateLookupQuery {
            inner: Inner {
                field: field.to_string(),
                index: index.to_string(),
                id: id.to_string(),
                routing: None,
                preference: None,
                version: None,
                name: None,
            },
        }
    }
}

impl PercolateLookupQuery {
    /// Routing to be used to fetch document to percolate
    pub fn routing<S>(mut self, routing: S) -> Self
    where
        S: ToString,
    {
        self.inner.routing = Some(routing.to_string());
        self
    }

    /// Preference to be used to fetch document to percolate
    pub fn preference<S>(mut self, preference: S) -> Self
    where
        S: ToString,
    {
        self.inner.preference = Some(preference.to_string());
        self
    }

    /// The expected version of the document to be fetched
    pub fn version<S>(mut self, version: S) -> Self
    where
        S: TryInto<u64>,
    {
        if let Ok(version) = version.try_into() {
            self.inner.version = Some(version);
        }
        self
    }

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

impl ShouldSkip for PercolateLookupQuery {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::percolate_lookup("field_name", "index_name", "document_id"),
            json!({
                "percolate": {
                    "field": "field_name",
                    "index": "index_name",
                    "id": "document_id"
                }
            }),
        );

        assert_serialize_query(
            Query::percolate_lookup("field_name", "index_name", "document_id")
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
            }),
        );
    }
}
