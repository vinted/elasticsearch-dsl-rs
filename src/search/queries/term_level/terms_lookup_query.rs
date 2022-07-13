use crate::search::*;
use crate::util::*;

/// Terms lookup fetches the field values of an existing document.
/// Elasticsearch then uses those values as search terms. This can be
/// helpful when searching for a large set of terms.
///
/// Because terms lookup fetches values from a document, the
/// [`_source`](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-source-field.html)
/// mapping field must be enabled to use terms lookup. The `_source`
/// field is enabled by default.
///
/// > By default, Elasticsearch limits the `terms` query to a maximum of
/// 65,536 terms. This includes terms fetched using terms lookup. You can
/// change this limit using the
/// [`index.max_terms_count setting`](https://www.elastic.co/guide/en/elasticsearch/reference/current/index-modules.html#index-max-terms-count).
///
/// To create a terms lookup query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::terms_lookup("test", "index", "id", "path")
///     .routing("routing")
///     .boost(1.3)
///     .name("lookup");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-terms-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct TermsLookupQuery {
    #[serde(flatten)]
    pair: KeyValuePair<String, TermsLookup>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct TermsLookup {
    index: String,
    id: String,
    path: String,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    routing: Option<String>,
}

impl Query {
    /// Creates an instance of [`TermsLookupQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `index` - Name of the index from which to fetch field values.
    /// - `id` - [ID](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-id-field.html)
    /// of the document from which to fetch field values.
    /// - `path` - Name of the field from which to fetch field values. Elasticsearch uses
    /// these values as search terms for the query. If the field values
    /// include an array of nested inner objects, you can access those objects
    /// using dot notation syntax.
    pub fn terms_lookup<S, T, U, V>(field: S, index: T, id: U, path: V) -> TermsLookupQuery
    where
        S: ToString,
        T: ToString,
        U: ToString,
        V: ToString,
    {
        TermsLookupQuery {
            pair: KeyValuePair::new(
                field.to_string(),
                TermsLookup {
                    index: index.to_string(),
                    id: id.to_string(),
                    path: path.to_string(),
                    routing: None,
                },
            ),
            boost: None,
            _name: None,
        }
    }
}

impl TermsLookupQuery {
    add_boost_and_name!();
}

impl TermsLookupQuery {
    /// Custom [routing value](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-routing-field.html)
    /// of the document from which to fetch term values. If a custom routing
    /// value was provided when the document was indexed, this parameter is
    /// required.
    pub fn routing<S>(mut self, routing: S) -> Self
    where
        S: ToString,
    {
        self.pair.value.routing = Some(routing.to_string());
        self
    }
}

impl ShouldSkip for TermsLookupQuery {}

serialize_query!("terms": TermsLookupQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::terms_lookup("test", "index_value", "id_value", "path_value"),
            json!({
                "terms": {
                    "test": {
                        "index": "index_value",
                        "id": "id_value",
                        "path": "path_value",
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::terms_lookup("test", "index_value", "id_value", "path_value")
                .routing("routing_value")
                .boost(2)
                .name("test"),
            json!({
                "terms": {
                    "test": {
                        "index": "index_value",
                        "id": "id_value",
                        "path": "path_value",
                        "routing": "routing_value"
                    },
                    "boost": 2,
                    "_name": "test",
                }
            }),
        );
    }
}
