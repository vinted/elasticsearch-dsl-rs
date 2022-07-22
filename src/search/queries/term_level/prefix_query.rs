use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// Returns documents that contain a specific prefix in a provided field.
///
/// To create a prefix query with numeric values:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::prefix("test", 123);
/// ```
/// To create a prefix query with string values and optional fields:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::prefix("test", "username")
///     .boost(2)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-prefix-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct PrefixQuery {
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
    /// Creates an instance of [`PrefixQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `value` - Term you wish to find in the provided field.
    /// To return a document, the term must exactly match the field value, including whitespace and capitalization.
    pub fn prefix<T, U>(field: T, value: U) -> PrefixQuery
    where
        T: ToString,
        U: Serialize,
    {
        PrefixQuery {
            field: field.to_string(),
            value: Term::new(value),
            rewrite: None,
            case_insensitive: None,
            boost: None,
            _name: None,
        }
    }
}

impl PrefixQuery {
    /// Method used to rewrite the query. For valid values and more information, see the
    /// [rewrite](Rewrite) parameter.
    pub fn rewrite(mut self, rewrite: Rewrite) -> Self {
        self.rewrite = Some(rewrite);
        self
    }

    /// Allows ASCII case insensitive matching of the value with the indexed field values when set
    /// to true. Default is false which means the case sensitivity of matching depends on the
    /// underlying field’s mapping.
    pub fn case_insensitive(mut self, case_insensitive: bool) -> Self {
        self.case_insensitive = Some(case_insensitive);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for PrefixQuery {
    fn should_skip(&self) -> bool {
        self.value.should_skip()
    }
}

serialize_with_root_keyed!("prefix": PrefixQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::prefix("test", 123),
            json!({
                "prefix": {
                    "test": {
                        "value": 123
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::prefix("test", 123)
                .rewrite(Rewrite::ConstantScore)
                .case_insensitive(true)
                .boost(2)
                .name("test"),
            json!({
                "prefix": {
                    "test": {
                        "value": 123,
                        "rewrite": "constant_score",
                        "case_insensitive": true,
                        "boost": 2.0,
                        "_name": "test"
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::bool().filter(Query::prefix("test", None::<String>)),
            json!({ "bool": {} }),
        )
    }
}
