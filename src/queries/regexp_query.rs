use super::params::*;
use super::Query;
use crate::ShouldSkip;
use serde::ser::{Serialize, SerializeMap, Serializer};

/// Returns documents that contain terms matching a
/// [regular expression](https://en.wikipedia.org/wiki/Regular_expression).
///
/// A regular expression is a way to match patterns in data using placeholder characters, called
/// operators. For a list of operators supported by the `regexp` query, see
/// [Regular expression syntax](https://www.elastic.co/guide/en/elasticsearch/reference/current/regexp-syntax.html).
///
/// To create a regexp query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// RegexpQuery::new("test", "username");
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::regexp("test", "username");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-regexp-query.html>
#[derive(Debug, Clone, PartialEq)]
pub struct RegexpQuery {
    field: String,
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    value: String,

    #[serde(
        skip_serializing_if = "ShouldSkip::should_skip",
        serialize_with = "join_with_pipe"
    )]
    flags: Vec<RegexpFlag>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    case_insensitive: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    max_determinized_states: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    rewrite: Option<Rewrite>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [RegexpQuery](RegexpQuery)
    ///
    /// - `field` - Field you wish to search.
    /// - `value` - Regular expression for terms you wish to find in the provided field. For a list
    /// of supported operators, see
    /// [Regular expression syntax](https://www.elastic.co/guide/en/elasticsearch/reference/current/regexp-syntax.html). <br>
    ///
    /// By default, regular expressions are limited to 1,000 characters. You can change this limit
    /// using the
    /// [`index.max_regex_length`](https://www.elastic.co/guide/en/elasticsearch/reference/current/index-modules.html#index-max-regex-length) setting.
    pub fn regexp<S>(field: S, value: S) -> RegexpQuery
    where
        S: Into<String>,
    {
        RegexpQuery::new(field, value)
    }
}

impl RegexpQuery {
    /// Creates an instance of [RegexpQuery](RegexpQuery)
    ///
    /// - `field` - Field you wish to search.
    /// - `value` - Regular expression for terms you wish to find in the provided field. For a list
    /// of supported operators, see
    /// [Regular expression syntax](https://www.elastic.co/guide/en/elasticsearch/reference/current/regexp-syntax.html). <br>
    ///
    /// By default, regular expressions are limited to 1,000 characters. You can change this limit
    /// using the
    /// [`index.max_regex_length`](https://www.elastic.co/guide/en/elasticsearch/reference/current/index-modules.html#index-max-regex-length) setting.
    pub fn new<S>(field: S, value: S) -> RegexpQuery
    where
        S: Into<String>,
    {
        Self {
            field: field.into(),
            inner: Inner {
                value: value.into(),
                flags: vec![],
                case_insensitive: None,
                max_determinized_states: None,
                rewrite: None,
                boost: None,
                _name: None,
            },
        }
    }

    /// Enables optional operators for the regular expression. For valid values and more
    /// information, see
    /// [Regular expression syntax](https://www.elastic.co/guide/en/elasticsearch/reference/current/regexp-syntax.html#regexp-optional-operators).
    pub fn flags<I>(mut self, flags: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<RegexpFlag>,
    {
        self.inner.flags.extend(flags.into_iter().map(Into::into));
        self
    }

    /// Allows case insensitive matching of the regular expression value with the indexed field
    /// values when set to `true`. Default is `false` which means the case sensitivity of matching
    /// depends on the underlying field’s mapping.
    pub fn case_insensitive(mut self, case_insensitive: bool) -> Self {
        self.inner.case_insensitive = Some(case_insensitive);
        self
    }

    /// Maximum number of
    /// [automaton states](https://en.wikipedia.org/wiki/Deterministic_finite_automaton)
    /// required for the query. Default is 10000.
    ///
    /// Elasticsearch uses [Apache Lucene](https://lucene.apache.org/core/) internally to parse
    /// regular expressions. Lucene converts each regular expression to a finite automaton
    /// containing a number of determinized states.
    ///
    /// You can use this parameter to prevent that conversion from unintentionally consuming too
    /// many resources. You may need to increase this limit to run complex regular expressions.
    pub fn max_determinized_states(mut self, max_determinized_states: u64) -> Self {
        self.inner.max_determinized_states = Some(max_determinized_states);
        self
    }

    /// Method used to rewrite the query. For valid values and more information, see the
    /// [rewrite](Rewrite) parameter.
    pub fn rewrite(mut self, rewrite: Rewrite) -> Self {
        self.inner.rewrite = Some(rewrite);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for RegexpQuery {
    fn should_skip(&self) -> bool {
        self.inner.value.should_skip()
    }
}

impl Serialize for RegexpQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hash = std::collections::HashMap::new();
        let _ = hash.insert(&self.field, &self.inner);

        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("regexp", &hash)?;
        map.end()
    }
}

fn join_with_pipe<S>(value: &[RegexpFlag], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("|")
        .serialize(serializer)
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            RegexpQuery::new("test", "regexp"),
            json!({
                "regexp": {
                    "test": {
                        "value": "regexp"
                    }
                }
            })
        );

        with_all_fields(
            RegexpQuery::new("test", "regexp")
                .flags([RegexpFlag::Complement, RegexpFlag::Interval])
                .case_insensitive(false)
                .max_determinized_states(2)
                .rewrite(Rewrite::ConstantScore)
                .boost(2)
                .name("test"),
            json!({
                "regexp": {
                    "test": {
                        "value": "regexp",
                        "flags": "COMPLEMENT|INTERVAL",
                        "case_insensitive": false,
                        "max_determinized_states": 2,
                        "rewrite": "constant_score",
                        "boost": 2.0,
                        "_name": "test"
                    }
                }
            })
        );
    }
}
