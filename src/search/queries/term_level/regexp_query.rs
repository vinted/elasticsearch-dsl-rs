use crate::search::*;
use crate::util::*;
use serde::Serialize;

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
/// Query::regexp("test", "username");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-regexp-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct RegexpQuery {
    #[serde(skip)]
    field: String,

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
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`RegexpQuery`]
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
        S: ToString,
    {
        RegexpQuery {
            field: field.to_string(),
            value: value.to_string(),
            flags: vec![],
            case_insensitive: None,
            max_determinized_states: None,
            rewrite: None,
            boost: None,
            _name: None,
        }
    }
}

impl RegexpQuery {
    /// Enables optional operators for the regular expression. For valid values and more
    /// information, see
    /// [Regular expression syntax](https://www.elastic.co/guide/en/elasticsearch/reference/current/regexp-syntax.html#regexp-optional-operators).
    pub fn flags<I>(mut self, flags: I) -> Self
    where
        I: IntoIterator<Item = RegexpFlag>,
    {
        self.flags.extend(flags.into_iter());
        self
    }

    /// Allows case insensitive matching of the regular expression value with the indexed field
    /// values when set to `true`. Default is `false` which means the case sensitivity of matching
    /// depends on the underlying field’s mapping.
    pub fn case_insensitive(mut self, case_insensitive: bool) -> Self {
        self.case_insensitive = Some(case_insensitive);
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
        self.max_determinized_states = Some(max_determinized_states);
        self
    }

    /// Method used to rewrite the query. For valid values and more information, see the
    /// [rewrite](Rewrite) parameter.
    pub fn rewrite(mut self, rewrite: Rewrite) -> Self {
        self.rewrite = Some(rewrite);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for RegexpQuery {
    fn should_skip(&self) -> bool {
        self.value.should_skip()
    }
}

serialize_with_root_keyed!("regexp": RegexpQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::regexp("test", "regexp"),
            json!({
                "regexp": {
                    "test": {
                        "value": "regexp"
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::regexp("test", "regexp")
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
            }),
        );
    }
}
