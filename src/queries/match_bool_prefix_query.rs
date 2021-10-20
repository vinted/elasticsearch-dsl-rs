use super::params::*;
use super::Query;
use crate::ShouldSkip;
use serde::ser::{Serialize, SerializeStruct, Serializer};

/// A `match_bool_prefix` query analyzes its input and constructs a
/// [`bool` query](super::BoolQuery) from the terms. Each term except the last is used in a
/// [`term` query](super::TermQuery). The last term is used in a
/// [`prefix` query](super::PrefixQuery).
///
/// To create a MatchBoolPrefix query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// MatchBoolPrefixQuery::new("test", "search text")
///     .boost(2)
///     .name("test");
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::match_bool_prefix("test", "search text")
///     .boost(2)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-bool-prefix-query.html>
#[derive(Debug, Clone, PartialEq)]
pub struct MatchBoolPrefixQuery {
    field: String,
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    query: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    analyzer: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    minimum_should_match: Option<MinimumShouldMatch>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    operator: Option<Operator>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`MatchBoolPrefixQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `query` - Text, number, boolean value or date you wish to find in the provided `<field>`
    pub fn match_bool_prefix(
        field: impl Into<String>,
        query: impl Into<String>,
    ) -> MatchBoolPrefixQuery {
        MatchBoolPrefixQuery::new(field, query)
    }
}

impl MatchBoolPrefixQuery {
    /// Creates an instance of [`MatchBoolPrefixQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `query` - Text, number, boolean value or date you wish to find in the provided `<field>`
    pub fn new(field: impl Into<String>, query: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            inner: Inner {
                query: query.into(),
                analyzer: None,
                minimum_should_match: None,
                operator: None,
                boost: None,
                _name: None,
            },
        }
    }

    /// [Analyzer](https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis.html)
    /// used to convert the text in the `query` value into tokens. Defaults to the
    /// [index-time analyzer](https://www.elastic.co/guide/en/elasticsearch/reference/current/specify-analyzer.html#specify-index-time-analyzer)
    /// mapped for the `<field>`. If no analyzer is mapped, the index’s default analyzer is used.
    pub fn analyzer(mut self, analyzer: impl Into<String>) -> Self {
        self.inner.analyzer = Some(analyzer.into());
        self
    }

    /// Minimum number of clauses that must match for a document to be returned.
    /// See the
    /// [`minimum_should_match` parameter](MinimumShouldMatch)
    /// for valid values and more information.
    pub fn minimum_should_match(
        mut self,
        minimum_should_match: impl Into<MinimumShouldMatch>,
    ) -> Self {
        self.inner.minimum_should_match = Some(minimum_should_match.into());
        self
    }

    /// Boolean logic used to interpret text in the `query` value
    pub fn operator(mut self, operator: Operator) -> Self {
        self.inner.operator = Some(operator);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for MatchBoolPrefixQuery {
    fn should_skip(&self) -> bool {
        self.inner.query.should_skip()
    }
}

impl Serialize for MatchBoolPrefixQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hash = std::collections::HashMap::new();
        let _ = hash.insert(&self.field, &self.inner);

        let mut map = serializer.serialize_struct("MatchBoolPrefixQuery", 1)?;
        map.serialize_field("match_bool_prefix", &hash)?;
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            MatchBoolPrefixQuery::new("test", "search text"),
            json!({
                "match_bool_prefix": {
                    "test": {
                        "query": "search text"
                    }
                }
            })
        );

        with_all_fields(
            MatchBoolPrefixQuery::new("test", "search text")
                .analyzer("search_time_analyzer")
                .minimum_should_match("12")
                .operator(Operator::Or)
                .boost(2)
                .name("test"),
            json!({
                "match_bool_prefix": {
                    "test": {
                        "query": "search text",
                        "analyzer": "search_time_analyzer",
                        "minimum_should_match": "12",
                        "operator": "OR",
                        "boost": 2.0,
                        "_name": "test"
                    }
                }
            })
        );
    }
}
