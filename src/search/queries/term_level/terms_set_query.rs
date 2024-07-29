use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// Returns documents that contain an **exact** terms_set in a provided field.
///
/// You can use the terms_set query to find documents based on a precise value such as a price, a product ID, or a username.
///
/// To create a terms_set query with numeric field:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::terms_set("test", [123], "required_matches");
/// ```
///
/// To create a terms_set query with script:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::terms_set(
///     "test",
///     [123],
///     TermsSetScript::new("Math.min(params.num_terms_sets, doc['required_matches'].value)")
///         .params(serde_json::json!({"num_terms_sets": 2}))
/// );
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-terms-set-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct TermsSetQuery {
    #[serde(skip)]
    field: String,

    terms: Terms,

    #[serde(flatten)]
    minimum_should_match: TermsSetMinimumShouldMatch,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`TermsSetQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `value` - TermsSet you wish to find in the provided field.
    ///   To return a document, the terms_set must exactly match the field value, including whitespace and capitalization.
    pub fn terms_set<S, T, U>(field: S, terms: T, minimum_should_match: U) -> TermsSetQuery
    where
        S: ToString,
        T: Into<Terms>,
        U: Into<TermsSetMinimumShouldMatch>,
    {
        TermsSetQuery {
            field: field.to_string(),
            terms: terms.into(),
            minimum_should_match: minimum_should_match.into(),
            boost: None,
            _name: None,
        }
    }
}

impl TermsSetQuery {
    add_boost_and_name!();
}

impl ShouldSkip for TermsSetQuery {
    fn should_skip(&self) -> bool {
        self.terms.should_skip()
    }
}

serialize_with_root_keyed!("terms_set": TermsSetQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::terms_set("test", [123], "required_matches"),
            json!({
                "terms_set": {
                    "test": {
                        "terms": [123],
                        "minimum_should_match_field": "required_matches"
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::terms_set(
                "programming_languages",
                ["c++", "java", "php"],
                TermsSetScript::new(
                    "Math.min(params.num_terms_sets, doc['required_matches'].value)",
                )
                .params(json!({"num_terms_sets": 2})),
            )
            .boost(2)
            .name("test"),
            json!({
                "terms_set": {
                    "programming_languages": {
                        "terms": ["c++", "java", "php"],
                        "minimum_should_match_script": {
                            "source": "Math.min(params.num_terms_sets, doc['required_matches'].value)",
                            "params": {
                                "num_terms_sets": 2
                            }
                        },
                        "boost": 2.0,
                        "_name": "test"
                    }
                }
            }),
        );
    }
}
