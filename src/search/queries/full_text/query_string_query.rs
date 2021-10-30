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
/// Query::query_string("(new york city) OR (big apple)")
///     .boost(2)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html>
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct QueryStringQuery {
    #[serde(rename = "query_string")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
struct Inner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    query: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    default_field: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    allow_leading_wildcard: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    analyze_wildcard: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    analyzer: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    auto_generate_synonyms_phrase_query: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    default_operator: Option<Operator>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    enable_position_increments: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fields: Vec<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fuzziness: Option<Fuzziness>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fuzzy_max_expansions: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fuzzy_prefix_length: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fuzzy_transpositions: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    lenient: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    max_determinized_states: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    minimum_should_match: Option<MinimumShouldMatch>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    quote_analyzer: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    phrase_slop: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    quote_field_suffix: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    rewrite: Option<Rewrite>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    time_zone: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`QueryStringQuery`]
    ///
    /// - `query` - Query string you wish to parse and use for search. See
    /// [Simple query string syntax](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html#simple-query-string-syntax).
    pub fn query_string<S>(query: S) -> QueryStringQuery
    where
        S: Into<String>,
    {
        QueryStringQuery {
            inner: Inner {
                query: query.into(),
                fields: vec![],
                default_operator: None,
                analyze_wildcard: None,
                analyzer: None,
                auto_generate_synonyms_phrase_query: None,
                fuzzy_transpositions: None,
                fuzzy_max_expansions: None,
                fuzzy_prefix_length: None,
                quote_field_suffix: None,
                lenient: None,
                minimum_should_match: None,
                allow_leading_wildcard: None,
                default_field: None,
                enable_position_increments: None,
                fuzziness: None,
                max_determinized_states: None,
                phrase_slop: None,
                quote_analyzer: None,
                rewrite: None,
                time_zone: None,
                boost: None,
                _name: None,
            },
        }
    }
}

impl QueryStringQuery {
    /// Default field you wish to search if no field is provided in the query
    /// string.
    ///
    /// Defaults to the `index.query.default_field` index setting, which has a
    /// default value of `*`. The `*` value extracts all fields that are
    /// eligible for term queries and filters the metadata fields. All
    /// extracted fields are then combined to build a query if no `prefix`
    /// is specified.
    ///
    /// Searching across all eligible fields does not include
    /// [nested documents](https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html).
    /// Use a [`nested` query](crate::NestedQuery) to search those documents.
    ///
    /// For mappings with a large number of fields, searching across all
    /// eligible fields could be expensive.
    ///
    /// There is a limit on the number of fields that can be queried at once.
    /// It is defined by the `indices.query.bool.max_clause_count`
    /// [search setting](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-settings.html),
    /// which defaults to 1024.
    pub fn default_field<S>(mut self, default_field: S) -> Self
    where
        S: ToString,
    {
        self.inner.default_field = Some(default_field.to_string());
        self
    }

    /// If `true`, the wildcard characters `*` and `?` are allowed as the first
    /// character of the query string.
    ///
    /// Defaults to `true`.
    pub fn allow_leading_wildcard(mut self, allow_leading_wildcard: bool) -> Self {
        self.inner.allow_leading_wildcard = Some(allow_leading_wildcard);
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

    /// Default boolean logic used to interpret text in the query string if no
    /// operators are specified.
    pub fn default_operator(mut self, default_operator: Operator) -> Self {
        self.inner.default_operator = Some(default_operator);
        self
    }

    /// If `true`, enable position increments in queries constructed from a
    /// `query_string` search.
    ///
    /// Defaults to `true`.
    pub fn enable_position_increments(mut self, enable_position_increments: bool) -> Self {
        self.inner.enable_position_increments = Some(enable_position_increments);
        self
    }

    /// Array of fields you wish to search.
    ///
    /// You can use this parameter query to search across multiple fields. See
    /// [Search multiple fields](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html#query-string-multi-field).
    pub fn fields<I>(mut self, fields: I) -> Self
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        self.inner.fields = fields.into_iter().map(|x| x.to_string()).collect();
        self
    }

    /// Maximum edit distance allowed for fuzzy matching. For fuzzy syntax, see
    /// [`Fuzziness`].
    pub fn fuzziness(mut self, fuzziness: Fuzziness) -> Self {
        self.inner.fuzziness = Some(fuzziness);
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

    /// Maximum number of
    /// [automaton states](https://en.wikipedia.org/wiki/Deterministic_finite_automaton)
    /// required for the query.
    ///
    /// Default is `10000`.
    ///
    /// Elasticsearch uses [Apache Lucene](https://lucene.apache.org/core/)
    /// internally to parse regular expressions. Lucene converts each regular
    /// expression to a finite automaton containing a number of determinized
    /// states.
    ///
    /// You can use this parameter to prevent that conversion from
    /// unintentionally consuming too many resources. You may need to increase
    /// this limit to run complex regular expressions.
    pub fn max_determinized_states(mut self, max_determinized_states: u32) -> Self {
        self.inner.max_determinized_states = Some(max_determinized_states);
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

    /// [Analyzer](https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis.html)
    /// used to convert quoted text in the query string into tokens.
    ///
    /// Defaults to the
    /// [`search_quote_analyzer`](https://www.elastic.co/guide/en/elasticsearch/reference/current/analyzer.html#search-quote-analyzer)
    /// mapped for the `default_field`.
    ///
    /// For quoted text, this parameter overrides the analyzer specified in the
    /// `analyzer` parameter.
    pub fn quote_analyzer<S>(mut self, quote_analyzer: S) -> Self
    where
        S: ToString,
    {
        self.inner.quote_analyzer = Some(quote_analyzer.to_string());
        self
    }

    /// Maximum number of positions allowed between matching tokens for
    /// phrases.
    ///
    /// Defaults to `0`. If `0`, exact phrase matches are required. Transposed
    /// terms have a slop of `2`.
    pub fn phrase_slop(mut self, phrase_slop: u32) -> Self {
        self.inner.phrase_slop = Some(phrase_slop);
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

    /// Method used to rewrite the query. For valid values and more
    /// information, see the [`rewrite` parameter](Rewrite).
    pub fn rewrite(mut self, rewrite: Rewrite) -> Self {
        self.inner.rewrite = Some(rewrite);
        self
    }

    /// [Coordinated Universal Time (UTC) offset](https://en.wikipedia.org/wiki/List_of_UTC_time_offsets)
    /// or [IANA time zone](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)
    /// used to convert `date` values in the query string to UTC.
    ///
    /// Valid values are ISO 8601 UTC offsets, such as `+01:00` or `-08:00`,
    /// and IANA time zone IDs, such as `America/Los_Angeles`.
    pub fn time_zone<S>(mut self, time_zone: S) -> Self
    where
        S: ToString,
    {
        self.inner.time_zone = Some(time_zone.to_string());
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for QueryStringQuery {
    fn should_skip(&self) -> bool {
        self.inner.query.should_skip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            Query::query_string("search text"),
            json!({
                "query_string": {
                    "query": "search text",
                }
            })
        );

        with_all_fields(
            Query::query_string("search text")
                .fields(["database"])
                .default_operator(Operator::And)
                .analyze_wildcard(true)
                .analyzer("search_time_analyzer")
                .auto_generate_synonyms_phrase_query(true)
                .fuzzy_max_expansions(20)
                .fuzzy_prefix_length(3)
                .fuzzy_transpositions(false)
                .lenient(true)
                .minimum_should_match("22")
                .quote_field_suffix("s")
                .boost(2)
                .name("test"),
            json!({
                "query_string": {
                    "query": "search text",
                    "fields": ["database"],
                    "default_operator": "AND",
                    "analyze_wildcard": true,
                    "analyzer": "search_time_analyzer",
                    "auto_generate_synonyms_phrase_query": true,
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
