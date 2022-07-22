use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// Returns documents that contain terms matching a wildcard pattern.
///
/// A wildcard operator is a placeholder that matches one or more characters. For example, the `*`
/// wildcard operator matches zero or more characters. You can combine wildcard operators with
/// other characters to create a wildcard pattern.
///
/// To create a wildcard query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::wildcard("test", 123);
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-wildcard-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct WildcardQuery {
    #[serde(skip_serializing)]
    field: String,

    value: Option<Term>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    rewrite: Option<Rewrite>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    case_insensitive: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`WildcardQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `value` - Wildcard you wish to find in the provided field.
    /// To return a document, the wildcard must exactly match the field value, including whitespace and capitalization.
    pub fn wildcard<T, U>(field: T, value: U) -> WildcardQuery
    where
        T: ToString,
        U: Serialize,
    {
        WildcardQuery {
            field: field.to_string(),
            value: Term::new(value),
            rewrite: None,
            case_insensitive: None,
            boost: None,
            _name: None,
        }
    }
}

impl WildcardQuery {
    /// Method used to rewrite the query. For valid values and more information, see the
    /// [rewrite](Rewrite) parameter.
    pub fn rewrite(mut self, rewrite: Rewrite) -> Self {
        self.rewrite = Some(rewrite);
        self
    }

    /// Allows case insensitive matching of the pattern with the indexed field values when set to
    /// true. Default is false which means the case sensitivity of matching depends on the
    /// underlying field’s mapping.
    pub fn case_insensitive(mut self, case_insensitive: bool) -> Self {
        self.case_insensitive = Some(case_insensitive);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for WildcardQuery {
    fn should_skip(&self) -> bool {
        self.value.should_skip()
    }
}

serialize_with_root_keyed!("wildcard": WildcardQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::wildcard("test", "value*"),
            json!({
                "wildcard": {
                    "test": {
                        "value": "value*"
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::wildcard("test", "value*")
                .rewrite(Rewrite::ConstantScore)
                .case_insensitive(true)
                .boost(2)
                .name("test"),
            json!({
                "wildcard": {
                    "test": {
                        "value": "value*",
                        "rewrite": "constant_score",
                        "case_insensitive": true,
                        "boost": 2.0,
                        "_name": "test"
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::bool().filter(Query::wildcard("test", None::<String>)),
            json!({ "bool": {} }),
        )
    }
}
