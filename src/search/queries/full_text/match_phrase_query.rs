use crate::search::*;
use crate::util::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

/// The `match_phrase` query analyzes the text and creates a phrase query out
/// of the analyzed text.
///
/// To create a MatchPhrase query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::match_phrase("test", "search text")
///     .boost(2)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query-phrase.html>
#[derive(Debug, Clone, PartialEq)]
pub struct MatchPhraseQuery {
    field: String,
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    query: Text,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    analyzer: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    slop: Option<u8>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`MatchPhraseQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `query` - Text, number, boolean value or date you wish to find in the provided
    /// `<field>`.<br>
    /// The `match_phrase` query
    /// [analyzes](https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis.html)
    /// any provided text before performing a search. This means the
    /// `match_phrase` query can search
    /// [`text`](https://www.elastic.co/guide/en/elasticsearch/reference/current/text.html)
    /// fields for analyzed tokens rather than an exact term.
    pub fn match_phrase(field: impl Into<String>, query: impl Into<Text>) -> MatchPhraseQuery {
        MatchPhraseQuery {
            field: field.into(),
            inner: Inner {
                query: query.into(),
                analyzer: None,
                slop: None,
                boost: None,
                _name: None,
            },
        }
    }
}

impl MatchPhraseQuery {
    /// [Analyzer](https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis.html)
    /// used to convert the text in the `query` value into tokens. Defaults to the
    /// [index-time analyzer](https://www.elastic.co/guide/en/elasticsearch/reference/current/specify-analyzer.html#specify-index-time-analyzer)
    /// mapped for the `<field>`. If no analyzer is mapped, the index’s default analyzer is used.
    pub fn analyzer(mut self, analyzer: impl Into<String>) -> Self {
        self.inner.analyzer = Some(analyzer.into());
        self
    }

    /// The maximum number of intervening unmatched positions, as well as
    /// whether matches are required to be in-order.
    pub fn slop(mut self, slop: u8) -> Self {
        self.inner.slop = Some(slop);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for MatchPhraseQuery {
    fn should_skip(&self) -> bool {
        self.inner.query.should_skip()
    }
}

impl Serialize for MatchPhraseQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hash = std::collections::HashMap::new();
        let _ = hash.insert(&self.field, &self.inner);

        let mut map = serializer.serialize_struct("MatchPhraseQuery", 1)?;
        map.serialize_field("match_phrase", &hash)?;
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            Query::match_phrase("test", "search text"),
            json!({
                "match_phrase": {
                    "test": {
                        "query": "search text"
                    }
                }
            })
        );

        with_all_fields(
            Query::match_phrase("test", "search text")
                .analyzer("search_time_analyzer")
                .slop(1u8)
                .boost(2)
                .name("test"),
            json!({
                "match_phrase": {
                    "test": {
                        "query": "search text",
                        "analyzer": "search_time_analyzer",
                        "slop": 1,
                        "boost": 2,
                        "_name": "test"
                    }
                }
            })
        );
    }
}
