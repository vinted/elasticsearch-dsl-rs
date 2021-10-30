use crate::search::*;
use crate::util::*;
use serde::ser::{Serialize, SerializeMap, Serializer};

/// Returns documents that contain terms similar to the search term, as measured by a
/// [Levenshtein edit distance](https://en.wikipedia.org/wiki/Levenshtein_distance).
///
/// An edit distance is the number of one-character changes needed to turn one term into another.
/// These changes can include:
///
/// - Changing a character (**b**ox → **f**ox)
/// - Removing a character (**b**lack → lack)
/// - Inserting a character (sic → sic**k**)
/// - Transposing two adjacent characters (**ac**t → **ca**t)
/// To find similar terms, the fuzzy query creates a set of all possible variations, or expansions, of the search term within a specified edit distance. The query then returns exact matches for each expansion.
///
/// To create a fuzzy query with numeric values:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::fuzzy("test", 123);
/// ```
/// To create a fuzzy query with string values and optional fields:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::fuzzy("test", "username")
///     .boost(2)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-fuzzy-query.html>
#[derive(Debug, Clone, PartialEq)]
pub struct FuzzyQuery {
    field: String,
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    value: OptionalScalar,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fuzziness: Option<Fuzziness>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    max_expansions: Option<u8>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    prefix_length: Option<u8>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    transpositions: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    rewrite: Option<Rewrite>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`FuzzyQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `value` - Fuzzy you wish to find in the provided field.
    pub fn fuzzy(field: impl Into<String>, value: impl Into<OptionalScalar>) -> FuzzyQuery {
        FuzzyQuery {
            field: field.into(),
            inner: Inner {
                value: value.into(),
                fuzziness: None,
                max_expansions: None,
                prefix_length: None,
                transpositions: None,
                rewrite: None,
                boost: None,
                _name: None,
            },
        }
    }
}

impl FuzzyQuery {
    /// Maximum edit distance allowed for matching.
    /// See [Fuzziness](Fuzziness)
    /// for valid values and more information. See
    /// [Fuzziness in the match query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query.html#query-dsl-match-query-fuzziness)
    /// for an example.
    pub fn fuzziness(mut self, fuzziness: impl Into<Fuzziness>) -> Self {
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

    /// Indicates whether edits include transpositions of two adjacent characters (ab → ba).
    /// Defaults to `true`
    pub fn transpositions(mut self, transpositions: bool) -> Self {
        self.inner.transpositions = Some(transpositions);
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

impl ShouldSkip for FuzzyQuery {
    fn should_skip(&self) -> bool {
        self.inner.value.should_skip()
    }
}

impl Serialize for FuzzyQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hash = std::collections::HashMap::new();
        let _ = hash.insert(&self.field, &self.inner);

        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("fuzzy", &hash)?;
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            Query::fuzzy("test", 123),
            json!({
                "fuzzy": {
                    "test": {
                        "value": 123
                    }
                }
            })
        );

        with_all_fields(
            Query::fuzzy("test", 123)
                .fuzziness(Fuzziness::Auto)
                .max_expansions(3)
                .prefix_length(4)
                .transpositions(false)
                .rewrite(Rewrite::ScoringBoolean)
                .boost(2)
                .name("test"),
            json!({
                "fuzzy": {
                    "test": {
                        "value": 123,
                        "fuzziness": "AUTO",
                        "max_expansions": 3,
                        "prefix_length": 4,
                        "transpositions": false,
                        "rewrite": "scoring_boolean",
                        "boost": 2.0,
                        "_name": "test"
                    }
                }
            })
        );

        with_none(
            Query::bool().filter(Query::fuzzy("test", None::<String>)),
            json!({ "bool": {} })
        )
    }
}
