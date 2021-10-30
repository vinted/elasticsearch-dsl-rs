use crate::search::*;
use crate::util::*;
use std::collections::BTreeSet;

/// Returns documents that contain one or more **exact** terms in a provided field.
/// The terms query is the same as the term query, except you can search for multiple values.
///
/// To create a terms query with numeric values:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::terms("test", vec![123]);
/// ```
/// To create a terms query with string values and optional fields:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::terms("test", vec!["username"])
///     .boost(2)
///     .name("test");
/// ```
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
pub struct TermsQuery<T: Terms> {
    #[serde(rename = "terms")]
    inner: Inner<T>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner<T: Terms> {
    #[serde(flatten)]
    pair: KeyValuePair<String, T>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`TermsQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `values` - An array of terms you wish to find in the provided field. To return a
    /// document, one or more terms must exactly match a field value,
    /// including whitespace and capitalization.<br>
    /// By default, Elasticsearch limits the `terms` query to a maximum of
    /// 65,536 terms. You can change this limit using the
    /// [`index.max_terms_count setting`](https://www.elastic.co/guide/en/elasticsearch/reference/current/index-modules.html#index-max-terms-count).<br>
    /// > To use the field values of an existing document as search terms,
    /// use the [terms lookup](crate::TermsLookup) parameters.
    pub fn terms<S, I>(field: S, values: I) -> TermsQuery<BTreeSet<Scalar>>
    where
        S: Into<String>,
        I: IntoIterator,
        I::Item: Into<Scalar>,
    {
        TermsQuery {
            inner: Inner {
                pair: KeyValuePair::new(field.into(), values.into_iter().map(Into::into).collect()),
                boost: None,
                _name: None,
            },
        }
    }

    /// Creates an instance of [`TermsQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `index` - Name of the index from which to fetch field values.
    /// - `id` - [ID](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-id-field.html)
    /// of the document from which to fetch field values.
    /// - `path` - Name of the field from which to fetch field values. Elasticsearch uses
    /// these values as search terms for the query. If the field values
    /// include an array of nested inner objects, you can access those objects
    /// using dot notation syntax.
    pub fn terms_lookup<S: Into<String>>(
        field: S,
        index: S,
        id: S,
        path: S,
    ) -> TermsQuery<TermsLookup> {
        TermsQuery {
            inner: Inner {
                pair: KeyValuePair::new(
                    field.into(),
                    TermsLookup {
                        index: index.into(),
                        id: id.into(),
                        path: path.into(),
                        routing: None,
                    },
                ),
                boost: None,
                _name: None,
            },
        }
    }
}

impl<T: Terms> TermsQuery<T> {
    add_boost_and_name!();
}

impl TermsQuery<TermsLookup> {
    /// Custom [routing value](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-routing-field.html)
    /// of the document from which to fetch term values. If a custom routing
    /// value was provided when the document was indexed, this parameter is
    /// required.
    pub fn routing(mut self, routing: impl Into<String>) -> Self {
        self.inner.pair.value.routing = Some(routing.into());
        self
    }
}

impl ShouldSkip for TermsQuery<BTreeSet<Scalar>> {
    fn should_skip(&self) -> bool {
        self.inner.pair.value.should_skip()
    }
}

impl ShouldSkip for TermsQuery<TermsLookup> {}

#[cfg(test)]
mod tests {
    use super::*;

    mod scalar {
        use super::*;

        test_serialization! {
            with_required_fields(
                Query::terms("test", vec![123, 12, 13]),
                json!({"terms": { "test": [12, 13, 123] } })
            );

            with_all_fields(
                Query::terms("test", vec![123]).boost(2).name("test"),
                json!({
                    "terms": {
                        "test": [123],
                        "boost": 2.0,
                        "_name": "test",
                    }
                })
            );
        }

        #[test]
        fn should_skip_when_there_are_no_values() {
            let values: Vec<Scalar> = Vec::new();
            let query = Query::terms("test", values);

            assert!(query.should_skip())
        }

        #[test]
        fn should_not_skip_when_there_are_no_values() {
            let query = Query::terms("test", vec![123]);

            assert!(!query.should_skip())
        }
    }

    mod lookup {
        use super::*;

        test_serialization! {
            with_required_fields(
                Query::terms_lookup("test", "index_value", "id_value", "path_value"),
                json!({
                    "terms": {
                        "test": {
                            "index": "index_value",
                            "id": "id_value",
                            "path": "path_value",
                        }
                    }
                })
            );

            with_all_fields(
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
                        "boost": 2.0,
                        "_name": "test",
                    }
                })
            );
        }
    }
}
