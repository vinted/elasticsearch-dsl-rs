use crate::search::*;
use crate::util::*;

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
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-terms-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct TermsQuery {
    #[serde(skip)]
    field: String,

    #[serde(skip)]
    terms: Terms,

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
    /// use the terms lookup parameters.
    pub fn terms<S, I>(field: S, terms: I) -> TermsQuery
    where
        S: ToString,
        I: Into<Terms>,
    {
        TermsQuery {
            field: field.to_string(),
            terms: terms.into(),
            boost: None,
            _name: None,
        }
    }
}

impl TermsQuery {
    add_boost_and_name!();
}

impl ShouldSkip for TermsQuery {
    fn should_skip(&self) -> bool {
        self.terms.should_skip()
    }
}

serialize_with_root_key_value_pair!("terms": TermsQuery, field, terms);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::terms("test", [12, 13, 123]),
            json!({"terms": { "test": [12, 13, 123] } }),
        );

        assert_serialize_query(
            Query::terms("test", [123]).boost(2).name("test"),
            json!({
                "terms": {
                    "test": [123],
                    "boost": 2,
                    "_name": "test",
                }
            }),
        );
    }

    #[test]
    fn should_skip_when_there_are_no_values() {
        let values: Vec<i32> = Vec::new();
        let query = Query::terms("test", values);

        assert!(query.should_skip())
    }

    #[test]
    fn should_not_skip_when_there_are_no_values() {
        let query = Query::terms("test", [123]);

        assert!(!query.should_skip())
    }
}
