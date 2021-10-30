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
/// Query::simple_query_string("\"fried eggs\" +(eggplant | potato) -frittata")
///     .boost(2)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html>
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct SimpleQueryStringQuery {
    #[serde(rename = "simple_query_string")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
struct Inner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    query: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fields: Vec<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    default_operator: Option<Operator>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    analyze_wildcard: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    analyzer: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    auto_generate_synonyms_phrase_query: Option<bool>,

    #[serde(
        skip_serializing_if = "ShouldSkip::should_skip",
        serialize_with = "join_with_pipe"
    )]
    flags: Vec<SimpleQueryStringQueryFlags>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fuzzy_max_expansions: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fuzzy_prefix_length: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fuzzy_transpositions: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    lenient: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    minimum_should_match: Option<MinimumShouldMatch>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    quote_field_suffix: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`SimpleQueryStringQuery`]
    ///
    /// - `query` - Query string you wish to parse and use for search. See
    /// [Simple query string syntax](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-syntax).
    pub fn simple_query_string<S>(query: S) -> SimpleQueryStringQuery
    where
        S: Into<String>,
    {
        SimpleQueryStringQuery {
            inner: Inner {
                query: query.into(),
                fields: vec![],
                default_operator: None,
                analyze_wildcard: None,
                analyzer: None,
                auto_generate_synonyms_phrase_query: None,
                fuzzy_transpositions: None,
                fuzzy_max_expansions: None,
                flags: vec![],
                fuzzy_prefix_length: None,
                quote_field_suffix: None,
                lenient: None,
                minimum_should_match: None,
                boost: None,
                _name: None,
            },
        }
    }
}

impl SimpleQueryStringQuery {
    /// Array of fields you wish to search.
    ///
    /// This field accepts wildcard expressions. You also can boost relevance
    /// scores for matches to particular fields using a caret (`^`) notation.
    /// See
    /// [Wildcards and per-field boosts in the fields parameter](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-boost)
    /// for examples.
    ///
    /// Defaults to the `index.query.default_field` index setting, which has a
    /// default value of `*`. The `*` value extracts all fields that are
    /// eligible to term queries and filters the metadata fields. All extracted
    /// fields are then combined to build a query if no `prefix` is specified.
    pub fn fields<I>(mut self, fields: I) -> Self
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        self.inner.fields = fields.into_iter().map(|x| x.to_string()).collect();
        self
    }

    /// Default boolean logic used to interpret text in the query string if no
    /// operators are specified.
    pub fn default_operator(mut self, default_operator: Operator) -> Self {
        self.inner.default_operator = Some(default_operator);
        self
    }

    /// If `true`, the query attempts to analyze wildcard terms in the query
    /// string.
    ///
    /// Defaults to `false`.
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.inner.analyze_wildcard = Some(analyze_wildcard);
        self
    }

    /// [Analyzer](https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis.html)
    /// used to convert text in the query string into tokens. Defaults to the
    /// [index-time analyzer](https://www.elastic.co/guide/en/elasticsearch/reference/current/specify-analyzer.html#specify-index-time-analyzer)
    /// mapped for the `default_field`. If no analyzer is mapped, the index’s
    /// default analyzer is used.
    pub fn analyzer(mut self, analyzer: impl Into<String>) -> Self {
        self.inner.analyzer = Some(analyzer.into());
        self
    }

    /// If `true`, the parser creates a
    /// [`match_phrase`](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query-phrase.html)
    /// query for each
    /// [multi-position token](https://www.elastic.co/guide/en/elasticsearch/reference/current/token-graphs.html#token-graphs-multi-position-tokens).
    ///
    /// Defaults to `true`. For examples, see
    /// [Multi-position tokens](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-synonyms).
    pub fn auto_generate_synonyms_phrase_query(
        mut self,
        auto_generate_synonyms_phrase_query: bool,
    ) -> Self {
        self.inner.auto_generate_synonyms_phrase_query = Some(auto_generate_synonyms_phrase_query);
        self
    }

    /// List of enabled operators for the simple query string syntax.
    ///
    /// Defaults to [ALL](SimpleQueryStringQueryFlags::All) (all operators).
    /// See [Limit operators](SimpleQueryStringQueryFlags) for valid values.
    pub fn flags<I>(mut self, flags: I) -> Self
    where
        I: IntoIterator<Item = SimpleQueryStringQueryFlags>,
    {
        self.inner.flags.extend(flags.into_iter());
        self
    }

    /// Maximum number of terms to which the query expands for fuzzy matching.
    ///
    /// Defaults to `50`.
    pub fn fuzzy_max_expansions(mut self, fuzzy_max_expansions: u32) -> Self {
        self.inner.fuzzy_max_expansions = Some(fuzzy_max_expansions);
        self
    }

    /// Number of beginning characters left unchanged for fuzzy matching.
    ///
    /// Defaults to `0`.
    pub fn fuzzy_prefix_length(mut self, fuzzy_prefix_length: u32) -> Self {
        self.inner.fuzzy_prefix_length = Some(fuzzy_prefix_length);
        self
    }

    /// If `true`, edits for fuzzy matching include transpositions of two
    /// adjacent characters (ab → ba).
    ///
    /// Defaults to `true`.
    pub fn fuzzy_transpositions(mut self, fuzzy_transpositions: bool) -> Self {
        self.inner.fuzzy_transpositions = Some(fuzzy_transpositions);
        self
    }

    /// If `true`, format-based errors, such as providing a text `query`
    /// value for a
    /// [numeric](https://www.elastic.co/guide/en/elasticsearch/reference/current/number.html)
    /// field, are ignored.
    ///
    /// Defaults to `false`.
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.inner.lenient = Some(lenient);
        self
    }

    /// Minimum number of clauses that must match for a document to be returned.
    /// See the
    /// [`minimum_should_match` parameter](crate::MinimumShouldMatch)
    /// for valid values and more information.
    pub fn minimum_should_match(
        mut self,
        minimum_should_match: impl Into<MinimumShouldMatch>,
    ) -> Self {
        self.inner.minimum_should_match = Some(minimum_should_match.into());
        self
    }

    /// Suffix appended to quoted text in the query string.
    ///
    /// You can use this suffix to use a different analysis method for exact
    /// matches. See
    /// [Mixing exact search with stemming](https://www.elastic.co/guide/en/elasticsearch/reference/current/mixing-exact-search-with-stemming.html).
    pub fn quote_field_suffix<S>(mut self, quote_field_suffix: S) -> Self
    where
        S: ToString,
    {
        self.inner.quote_field_suffix = Some(quote_field_suffix.to_string());
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for SimpleQueryStringQuery {
    fn should_skip(&self) -> bool {
        self.inner.query.should_skip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            Query::simple_query_string("search text"),
            json!({
                "simple_query_string": {
                    "query": "search text",
                }
            })
        );

        with_all_fields(
            Query::simple_query_string("search text")
                .fields(["database"])
                .default_operator(Operator::And)
                .analyze_wildcard(true)
                .analyzer("search_time_analyzer")
                .auto_generate_synonyms_phrase_query(true)
                .flags([SimpleQueryStringQueryFlags::And, SimpleQueryStringQueryFlags::Escape])
                .fuzzy_max_expansions(20)
                .fuzzy_prefix_length(3)
                .fuzzy_transpositions(false)
                .lenient(true)
                .minimum_should_match("22")
                .quote_field_suffix("s")
                .boost(2)
                .name("test"),
            json!({
                "simple_query_string": {
                    "query": "search text",
                    "fields": ["database"],
                    "default_operator": "AND",
                    "analyze_wildcard": true,
                    "analyzer": "search_time_analyzer",
                    "auto_generate_synonyms_phrase_query": true,
                    "flags": "AND|ESCAPE",
                    "fuzzy_max_expansions": 20,
                    "fuzzy_prefix_length": 3,
                    "fuzzy_transpositions": false,
                    "lenient": true,
                    "minimum_should_match": "22",
                    "quote_field_suffix": "s",
                    "boost": 2.0,
                    "_name": "test",
                }
            })
        );
    }
}
