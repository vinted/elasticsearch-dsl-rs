use super::CompletionSuggester;

/// Suggester variants
#[derive(Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Suggester {
    Completion(CompletionSuggester),
}

impl std::fmt::Debug for Suggester {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Completion(suggester) => suggester.fmt(f),
        }
    }
}

impl From<CompletionSuggester> for Suggester {
    fn from(value: CompletionSuggester) -> Self {
        Self::Completion(value)
    }
}
