use crate::search::*;
use crate::util::*;
use serde::ser::{Serialize, SerializeMap, Serializer};

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
#[derive(Debug, Clone, PartialEq)]
pub struct WildcardQuery {
    field: String,
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    value: Term,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    rewrite: Option<Rewrite>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    case_insensitive: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`WildcardQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `value` - Wildcard you wish to find in the provided field.
    /// To return a document, the wildcard must exactly match the field value, including whitespace and capitalization.
    pub fn wildcard(field: impl Into<String>, value: impl Into<Term>) -> WildcardQuery {
        WildcardQuery {
            field: field.into(),
            inner: Inner {
                value: value.into(),
                rewrite: None,
                case_insensitive: None,
                boost: None,
                _name: None,
            },
        }
    }
}

impl WildcardQuery {
    /// Method used to rewrite the query. For valid values and more information, see the
    /// [rewrite](Rewrite) parameter.
    pub fn rewrite(mut self, rewrite: Rewrite) -> Self {
        self.inner.rewrite = Some(rewrite);
        self
    }

    /// Allows case insensitive matching of the pattern with the indexed field values when set to
    /// true. Default is false which means the case sensitivity of matching depends on the
    /// underlying field’s mapping.
    pub fn case_insensitive(mut self, case_insensitive: bool) -> Self {
        self.inner.case_insensitive = Some(case_insensitive);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for WildcardQuery {
    fn should_skip(&self) -> bool {
        self.inner.value.should_skip()
    }
}

impl Serialize for WildcardQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hash = std::collections::HashMap::new();
        let _ = hash.insert(&self.field, &self.inner);

        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("wildcard", &hash)?;
        map.end()
    }
}

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
                        "boost": 2,
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
