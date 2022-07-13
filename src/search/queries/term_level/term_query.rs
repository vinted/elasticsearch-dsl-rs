use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// Returns documents that contain an **exact** term in a provided field.
///
/// You can use the term query to find documents based on a precise value such as a price, a product ID, or a username.
///
/// To create a term query with numeric values:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::term("test", 123);
/// ```
/// To create a term query with string values and optional fields:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::term("test", "username")
///     .boost(2)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-term-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct TermQuery {
    #[serde(skip)]
    field: String,

    value: Option<Term>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`TermQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `value` - Term you wish to find in the provided field.
    /// To return a document, the term must exactly match the field value, including whitespace and capitalization.
    pub fn term<T, U>(field: T, value: U) -> TermQuery
    where
        T: Into<String>,
        U: Serialize,
    {
        TermQuery {
            field: field.into(),
            value: Term::new(value),
            boost: None,
            _name: None,
        }
    }
}

impl TermQuery {
    add_boost_and_name!();
}

impl ShouldSkip for TermQuery {
    fn should_skip(&self) -> bool {
        self.value.should_skip()
    }
}

serialize_query!(keyed, "term": TermQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::term("test", 123),
            json!({
                "term": {
                    "test": {
                        "value": 123
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::term("test", 123).boost(2).name("test"),
            json!({
                "term": {
                    "test": {
                        "value": 123,
                        "boost": 2,
                        "_name": "test"
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::bool().filter(Query::term("test", None::<String>)),
            json!({ "bool": {} }),
        )
    }
}
