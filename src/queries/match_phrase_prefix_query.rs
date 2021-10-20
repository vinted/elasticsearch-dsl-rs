use super::params::*;
use super::Query;
use crate::ShouldSkip;
use serde::ser::{Serialize, SerializeStruct, Serializer};

/// Returns documents that contain the words of a provided text, in the **same order** as provided.
/// The last term of the provided text is treated as a [prefix](super::PrefixQuery), matching any
/// words that begin with that term.
///
/// To create a MatchPhrasePrefix query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// MatchPhrasePrefixQuery::new("test", "search text")
///     .boost(2)
///     .name("test");
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::match_phrase_prefix("test", "search text")
///     .boost(2)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query-phrase-prefix.html>
#[derive(Debug, Clone, PartialEq)]
pub struct MatchPhrasePrefixQuery {
    field: String,
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    query: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    analyzer: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    max_expansions: Option<u8>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    slop: Option<u8>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    zero_terms_query: Option<ZeroTermsQuery>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`MatchPhrasePrefixQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `query` - Text you wish to find in the provided <field>. <br> The `match_phrase_prefix`
    /// query analyzes any provided text into tokens before performing a search. The last term of
    /// this text is treated as a [prefix](super::PrefixQuery), matching any words that begin with
    /// that term.
    pub fn match_phrase_prefix(
        field: impl Into<String>,
        query: impl Into<String>,
    ) -> MatchPhrasePrefixQuery {
        MatchPhrasePrefixQuery::new(field, query)
    }
}

impl MatchPhrasePrefixQuery {
    /// Creates an instance of [`MatchPhrasePrefixQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `query` - Text you wish to find in the provided <field>. <br> The `match_phrase_prefix`
    /// query analyzes any provided text into tokens before performing a search. The last term of
    /// this text is treated as a [prefix](super::PrefixQuery), matching any words that begin with
    /// that term.
    pub fn new(field: impl Into<String>, query: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            inner: Inner {
                query: query.into(),
                analyzer: None,
                max_expansions: None,
                slop: None,
                zero_terms_query: None,
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

    /// Maximum number of terms to which the query will expand.
    /// Defaults to `50`.
    pub fn max_expansions(mut self, max_expansions: u8) -> Self {
        self.inner.max_expansions = Some(max_expansions);
        self
    }

    /// The maximum number of intervening unmatched positions, as well as
    /// whether matches are required to be in-order.
    pub fn slop(mut self, slop: u8) -> Self {
        self.inner.slop = Some(slop);
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

impl ShouldSkip for MatchPhrasePrefixQuery {
    fn should_skip(&self) -> bool {
        self.inner.query.should_skip()
    }
}

impl Serialize for MatchPhrasePrefixQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hash = std::collections::HashMap::new();
        let _ = hash.insert(&self.field, &self.inner);

        let mut map = serializer.serialize_struct("MatchPhrasePrefixQuery", 1)?;
        map.serialize_field("match_phrase_prefix", &hash)?;
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            MatchPhrasePrefixQuery::new("test", "search text"),
            json!({
                "match_phrase_prefix": {
                    "test": {
                        "query": "search text"
                    }
                }
            })
        );

        with_all_fields(
            MatchPhrasePrefixQuery::new("test", "search text")
                .analyzer("search_time_analyzer")
                .max_expansions(20)
                .slop(5)
                .zero_terms_query(ZeroTermsQuery::None)
                .boost(2)
                .name("test"),
            json!({
                "match_phrase_prefix": {
                    "test": {
                        "query": "search text",
                        "analyzer": "search_time_analyzer",
                        "max_expansions": 20,
                        "slop": 5,
                        "zero_terms_query": "none",
                        "boost": 2.0,
                        "_name": "test"
                    }
                }
            })
        );
    }
}
