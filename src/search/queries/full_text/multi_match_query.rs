use crate::search::*;
use crate::util::*;

/// Returns documents that match a provided text, number, date or boolean value.
/// The provided text is analyzed before matching.
///
/// The `match` query is the standard query for performing a full-text search,
/// including options for fuzzy matching.
///
/// To create a Match query with numeric values:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::multi_match(vec!["test"], "search text")
///     .boost(2)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-multi-match-query.html>
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct MultiMatchQuery {
    #[serde(rename = "multi_match")]
    inner: Inner,
}
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
struct Inner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fields: Vec<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    r#type: Option<MultiMatchQueryType>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    tie_breaker: Option<TieBreaker>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    query: Text,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    analyzer: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    auto_generate_synonyms_phrase_query: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fuzziness: Option<Fuzziness>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    max_expansions: Option<u8>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    prefix_length: Option<u8>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fuzzy_transpositions: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fuzzy_rewrite: Option<Rewrite>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    lenient: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    operator: Option<Operator>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    minimum_should_match: Option<MinimumShouldMatch>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    zero_terms_query: Option<ZeroTermsQuery>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`MultiMatchQuery`]
    ///
    /// - `fields` - Fields you wish to search.
    /// - `query` - Text, number, boolean value or date you wish to find in the provided
    /// `<field>`. The `match` query
    /// [analyzes](https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis.html)
    /// any provided text before performing a search. This means the `match`
    /// query can search
    /// [`text`](https://www.elastic.co/guide/en/elasticsearch/reference/current/text.html)
    /// fields for analyzed tokens rather than an exact term.
    pub fn multi_match<F, S>(fields: F, query: S) -> MultiMatchQuery
    where
        F: IntoIterator,
        F::Item: ToString,
        S: Into<Text>,
    {
        MultiMatchQuery {
            inner: Inner {
                fields: fields.into_iter().map(|s| s.to_string()).collect(),
                r#type: None,
                tie_breaker: None,
                query: query.into(),
                analyzer: None,
                auto_generate_synonyms_phrase_query: None,
                fuzziness: None,
                max_expansions: None,
                prefix_length: None,
                fuzzy_transpositions: None,
                fuzzy_rewrite: None,
                lenient: None,
                operator: None,
                minimum_should_match: None,
                zero_terms_query: None,
                boost: None,
                _name: None,
            },
        }
    }
}

impl MultiMatchQuery {
    /// The way the multi_match query is executed internally depends on the
    /// type parameter
    pub fn r#type(mut self, r#type: MultiMatchQueryType) -> Self {
        self.inner.r#type = Some(r#type);

        match r#type {
            MultiMatchQueryType::BestFields(tie_breaker) => self.inner.tie_breaker = tie_breaker,
            _ => self.inner.tie_breaker = None,
        }
        self
    }

    /// [Analyzer](https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis.html)
    /// used to convert the text in the `query` value into tokens. Defaults to the
    /// [index-time analyzer](https://www.elastic.co/guide/en/elasticsearch/reference/current/specify-analyzer.html#specify-index-time-analyzer)
    /// mapped for the `<field>`. If no analyzer is mapped, the index’s default analyzer is used.
    pub fn analyzer<T>(mut self, analyzer: T) -> Self
    where
        T: Into<String>,
    {
        self.inner.analyzer = Some(analyzer.into());
        self
    }

    /// If `true`,
    /// [match phrase](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query-phrase.html)
    /// queries are automatically created for multi-term synonyms. Defaults to `true`.
    ///
    /// See [Use synonyms with match query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query.html#query-dsl-match-query-synonyms)
    /// for an example.
    pub fn auto_generate_synonyms_phrase_query(
        mut self,
        auto_generate_synonyms_phrase_query: bool,
    ) -> Self {
        self.inner.auto_generate_synonyms_phrase_query = Some(auto_generate_synonyms_phrase_query);
        self
    }

    /// Maximum edit distance allowed for matching.
    /// See [Fuzziness](Fuzziness)
    /// for valid values and more information. See
    /// [Fuzziness in the match query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query.html#query-dsl-match-query-fuzziness)
    /// for an example.
    pub fn fuzziness<T>(mut self, fuzziness: T) -> Self
    where
        T: Into<Fuzziness>,
    {
        self.inner.fuzziness = Some(fuzziness.into());
        self
    }

    /// Maximum number of terms to which the query will expand.
    /// Defaults to `50`.
    pub fn max_expansions(mut self, max_expansions: u8) -> Self {
        self.inner.max_expansions = Some(max_expansions);
        self
    }

    /// Number of beginning characters left unchanged for fuzzy matching.
    /// Defaults to `0`.
    pub fn prefix_length(mut self, prefix_length: u8) -> Self {
        self.inner.prefix_length = Some(prefix_length);
        self
    }

    /// If `true`, edits for fuzzy matching include transpositions of two
    /// adjacent characters (ab → ba). Defaults to `true`.
    pub fn fuzzy_transpositions(mut self, fuzzy_transpositions: bool) -> Self {
        self.inner.fuzzy_transpositions = Some(fuzzy_transpositions);
        self
    }

    /// Method used to rewrite the query. See the
    /// [`rewrite` parameter](Rewrite) for valid values and
    /// more information.
    ///
    /// If the `fuzziness` parameter is not `0`, the match query uses a
    /// `fuzzy_rewrite` method of `top_terms_blended_freqs_${max_expansions}`
    /// by default.
    pub fn fuzzy_rewrite(mut self, fuzzy_rewrite: Rewrite) -> Self {
        self.inner.fuzzy_rewrite = Some(fuzzy_rewrite);
        self
    }

    /// If `true`, format-based errors, such as providing a text `query`
    /// value for a
    /// [numeric](https://www.elastic.co/guide/en/elasticsearch/reference/current/number.html)
    /// field, are ignored. Defaults to `false`.
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.inner.lenient = Some(lenient);
        self
    }

    /// Boolean logic used to interpret text in the `query` value
    pub fn operator(mut self, operator: Operator) -> Self {
        self.inner.operator = Some(operator);
        self
    }

    /// Minimum number of clauses that must match for a document to be returned.
    /// See the
    /// [`minimum_should_match` parameter](crate::MinimumShouldMatch)
    /// for valid values and more information.
    pub fn minimum_should_match<T>(mut self, minimum_should_match: T) -> Self
    where
        T: Into<MinimumShouldMatch>,
    {
        self.inner.minimum_should_match = Some(minimum_should_match.into());
        self
    }

    /// Indicates whether no documents are returned if the `analyzer` removes
    /// all tokens, such as when using a `stop` filter.
    pub fn zero_terms_query(mut self, zero_terms_query: ZeroTermsQuery) -> Self {
        self.inner.zero_terms_query = Some(zero_terms_query);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for MultiMatchQuery {
    fn should_skip(&self) -> bool {
        self.inner.query.should_skip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::multi_match(["test"], "search text"),
            json!({
                "multi_match": {
                    "query": "search text",
                    "fields": ["test"],
                }
            }),
        );

        assert_serialize_query(
            Query::multi_match(["test"], "search text")
                .r#type(MultiMatchQueryType::BestFields(
                    TieBreaker::try_from(0.2).ok(),
                ))
                .analyzer("search_time_analyzer")
                .auto_generate_synonyms_phrase_query(true)
                .fuzziness(23)
                .max_expansions(2)
                .prefix_length(3)
                .fuzzy_transpositions(false)
                .fuzzy_rewrite(Rewrite::ConstantScoreBoolean)
                .lenient(true)
                .operator(Operator::And)
                .minimum_should_match("22")
                .zero_terms_query(ZeroTermsQuery::None)
                .boost(2)
                .name("test"),
            json!({
                "multi_match": {
                    "query": "search text",
                    "fields": ["test"],
                    "type": "best_fields",
                    "tie_breaker": 0.2,
                    "analyzer": "search_time_analyzer",
                    "auto_generate_synonyms_phrase_query": true,
                    "fuzziness": 23,
                    "max_expansions": 2,
                    "prefix_length": 3,
                    "fuzzy_transpositions": false,
                    "fuzzy_rewrite": "constant_score_boolean",
                    "lenient": true,
                    "operator": "AND",
                    "minimum_should_match": "22",
                    "zero_terms_query": "none",
                    "boost": 2,
                    "_name": "test",
                }
            }),
        );
    }
}
