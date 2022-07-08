use crate::search::*;
use crate::util::*;

/// The combined_fields query supports searching multiple text fields as if
/// their contents had been indexed into one combined field. The query takes a
/// term-centric view of the input string: first it analyzes the query string
/// into individual terms, then looks for each term in any of the fields. This
/// query is particularly useful when a match could span multiple text fields.
///
/// To create a combined fields query with:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::combined_fields(["title", "abstract", "body"], "database systems")
///     .boost(2)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-combined-fields-query.html>
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct CombinedFieldsQuery {
    #[serde(rename = "combined_fields")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
struct Inner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fields: Vec<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    query: Text,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    auto_generate_synonyms_phrase_query: Option<bool>,

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
    /// Creates an instance of [`CombinedFieldsQuery`]
    ///
    /// - `fields` - List of fields to search. Field wildcard patterns are
    /// allowed. Only text fields are supported, and they must all have the
    /// same search analyzer.
    /// - `query` - Text to search for in the provided `<fields>`.
    /// The combined_fields query analyzes the provided text before performing a search.
    pub fn combined_fields<F, S>(fields: F, query: S) -> CombinedFieldsQuery
    where
        F: IntoIterator,
        F::Item: ToString,
        S: Into<Text>,
    {
        CombinedFieldsQuery {
            inner: Inner {
                fields: fields.into_iter().map(|s| s.to_string()).collect(),
                query: query.into(),
                auto_generate_synonyms_phrase_query: None,
                operator: None,
                minimum_should_match: None,
                zero_terms_query: None,
                boost: None,
                _name: None,
            },
        }
    }
}

impl CombinedFieldsQuery {
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

impl ShouldSkip for CombinedFieldsQuery {
    fn should_skip(&self) -> bool {
        self.inner.fields.should_skip() || self.inner.query.should_skip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::combined_fields(["test"], "search text"),
            json!({
                "combined_fields": {
                    "query": "search text",
                    "fields": ["test"],
                }
            }),
        );

        assert_serialize_query(
            Query::combined_fields(["test"], "search text")
                .auto_generate_synonyms_phrase_query(true)
                .operator(Operator::And)
                .minimum_should_match("22")
                .zero_terms_query(ZeroTermsQuery::None)
                .boost(2)
                .name("test"),
            json!({
                "combined_fields": {
                    "query": "search text",
                    "fields": ["test"],
                    "auto_generate_synonyms_phrase_query": true,
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
