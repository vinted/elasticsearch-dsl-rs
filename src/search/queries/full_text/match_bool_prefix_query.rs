use crate::search::*;
use crate::util::*;

/// A `match_bool_prefix` query analyzes its input and constructs a
/// [`bool` query](crate::BoolQuery) from the terms. Each term except the last is used in a
/// [`term` query](crate::TermQuery). The last term is used in a
/// [`prefix` query](crate::PrefixQuery).
///
/// To create a MatchBoolPrefix query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::match_bool_prefix("test", "search text")
///     .boost(2)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-bool-prefix-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct MatchBoolPrefixQuery {
    #[serde(skip)]
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    query: Text,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    analyzer: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    minimum_should_match: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    operator: Option<Operator>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`MatchBoolPrefixQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `query` - Text, number, boolean value or date you wish to find in the provided `<field>`
    pub fn match_bool_prefix<T, U>(field: T, query: U) -> MatchBoolPrefixQuery
    where
        T: ToString,
        U: Into<Text>,
    {
        MatchBoolPrefixQuery {
            field: field.to_string(),
            query: query.into(),
            analyzer: None,
            minimum_should_match: None,
            operator: None,
            boost: None,
            _name: None,
        }
    }
}

impl MatchBoolPrefixQuery {
    /// [Analyzer](https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis.html)
    /// used to convert the text in the `query` value into tokens. Defaults to the
    /// [index-time analyzer](https://www.elastic.co/guide/en/elasticsearch/reference/current/specify-analyzer.html#specify-index-time-analyzer)
    /// mapped for the `<field>`. If no analyzer is mapped, the index’s default analyzer is used.
    pub fn analyzer<T>(mut self, analyzer: T) -> Self
    where
        T: ToString,
    {
        self.analyzer = Some(analyzer.to_string());
        self
    }

    /// Minimum number of clauses that must match for a document to be returned. See the
    /// `minimum_should_match` parameter for valid values and more information.
    pub fn minimum_should_match<T>(mut self, minimum_should_match: T) -> Self
    where
        T: ToString,
    {
        self.minimum_should_match = Some(minimum_should_match.to_string());
        self
    }

    /// Boolean logic used to interpret text in the `query` value
    pub fn operator(mut self, operator: Operator) -> Self {
        self.operator = Some(operator);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for MatchBoolPrefixQuery {
    fn should_skip(&self) -> bool {
        self.query.should_skip()
    }
}

serialize_with_root_keyed!("match_bool_prefix": MatchBoolPrefixQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::match_bool_prefix("test", "search text"),
            json!({
                "match_bool_prefix": {
                    "test": {
                        "query": "search text"
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::match_bool_prefix("test", "search text")
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
            }),
        );
    }
}
