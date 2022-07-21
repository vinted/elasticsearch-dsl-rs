use super::{SuggestContextQuery, SuggestFuzziness, Suggester};
use crate::{util::ShouldSkip, Map};

/// The `completion` suggester provides auto-complete/search-as-you-type functionality. This is a
/// navigational feature to guide users to relevant results as they are typing, improving search
/// precision. It is not meant for spell correction or did-you-mean functionality like the `term`
/// or `phrase` suggesters.
///
/// Ideally, auto-complete functionality should be as fast as a user types to provide instant
/// feedback relevant to what a user has already typed in. Hence, `completion` suggester is
/// optimized for speed. The suggester uses data structures that enable fast lookups, but are
/// costly to build and are stored in-memory.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CompletionSuggester {
    prefix: String,

    completion: CompletionSuggesterCompletion,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct CompletionSuggesterCompletion {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    analyzer: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fuzzy: Option<SuggestFuzziness>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    size: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    skip_duplicates: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    contexts: Map<String, Vec<SuggestContextQuery>>,
}

impl Suggester {
    /// Creates an instance of [CompletionSuggester]
    pub fn completion<T, U>(field: T, prefix: U) -> CompletionSuggester
    where
        T: ToString,
        U: ToString,
    {
        CompletionSuggester {
            prefix: prefix.to_string(),
            completion: CompletionSuggesterCompletion {
                field: field.to_string(),
                analyzer: None,
                fuzzy: None,
                size: None,
                skip_duplicates: None,
                contexts: Default::default(),
            },
        }
    }
}

impl CompletionSuggester {
    /// Overrides search time analyzer
    pub fn analyzer<T>(mut self, analyzer: T) -> Self
    where
        T: ToString,
    {
        self.completion.analyzer = Some(analyzer.to_string());
        self
    }

    /// Allows searching with typos and still getting results back
    pub fn fuzzy(mut self, fuzzy: SuggestFuzziness) -> Self {
        self.completion.fuzzy = Some(fuzzy);
        self
    }

    /// The number of suggestions to return (defaults to `5`)
    pub fn size(mut self, size: u64) -> Self {
        self.completion.size = Some(size);
        self
    }

    /// Whether duplicate suggestions should be filtered out (defaults to false).
    pub fn skip_duplicates(mut self, skip_duplicates: bool) -> Self {
        self.completion.skip_duplicates = Some(skip_duplicates);
        self
    }

    /// Adds suggest context to filter or boost by an indexed context
    pub fn context<T, U>(mut self, context: T, suggest_context_queries: U) -> Self
    where
        T: ToString,
        U: IntoIterator<Item = SuggestContextQuery>,
    {
        let _ = self.completion.contexts.insert(
            context.to_string(),
            suggest_context_queries.into_iter().collect(),
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{util::assert_serialize, Fuzziness};

    #[test]
    fn serializes() {
        assert_serialize(
            Suggester::completion("autocomplete_field", "search text"),
            json!({
                "prefix": "search text",
                "completion": {
                    "field": "autocomplete_field"
                }
            }),
        );

        assert_serialize(
            Suggester::completion("autocomplete_field", "search text")
                .analyzer("search_analyzer")
                .size(10)
                .skip_duplicates(true)
                .fuzzy(
                    SuggestFuzziness::new()
                        .fuzziness(Fuzziness::Auto)
                        .min_length(2)
                        .prefix_length(3)
                        .transpositions(true)
                        .unicode_aware(false),
                )
                .context(
                    "place_type",
                    [
                        SuggestContextQuery::new("cafe"),
                        SuggestContextQuery::new("restaurants").boost(2),
                    ],
                ),
            json!({
                "prefix": "search text",
                "completion": {
                    "field": "autocomplete_field",
                    "size": 10,
                    "skip_duplicates": true,
                    "analyzer": "search_analyzer",
                    "fuzzy": {
                        "fuzziness": "AUTO",
                        "min_length": 2,
                        "prefix_length": 3,
                        "transpositions": true,
                        "unicode_aware": false
                    },
                    "contexts": {
                        "place_type": [
                            { "context": "cafe" },
                            { "context": "restaurants", "boost": 2 }
                        ]
                    }
                }
            }),
        );
    }
}
