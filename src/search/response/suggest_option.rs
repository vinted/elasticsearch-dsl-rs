use crate::util::ShouldSkip;
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;

/// Suggester response option variants
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SuggestOption {
    /// Completion suggester response option
    Completion(CompletionSuggestOption),

    /// Term suggester response option
    Term(TermSuggestOption),

    /// Phrase suggester response option
    Phrase(PhraseSuggestOption),
}

impl std::fmt::Debug for SuggestOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Completion(suggest_option) => suggest_option.fmt(f),
            Self::Term(suggest_option) => suggest_option.fmt(f),
            Self::Phrase(suggest_option) => suggest_option.fmt(f),
        }
    }
}

/// Suggester response item option
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompletionSuggestOption {
    /// Suggested text
    pub text: String,

    /// Document index
    #[serde(rename = "_index")]
    pub index: String,

    /// Document id
    #[serde(rename = "_id")]
    pub id: String,

    /// Document score for completion suggester, suggest score for term, phrase
    #[serde(alias = "_score")]
    pub score: f32,

    /// Document source
    ///
    /// Not using [crate::Source] due to a bug in enums and RawValue
    /// <https://github.com/serde-rs/json/issues/779>
    #[serde(
        skip_serializing_if = "ShouldSkip::should_skip",
        rename = "_source",
        default
    )]
    pub source: Option<serde_json::Value>,

    /// The contexts associated with the completed document
    ///
    /// Contexts always return either as a category or as geohash
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    pub contexts: BTreeMap<String, Vec<String>>,
}

impl CompletionSuggestOption {
    /// Parses document source into a concrete type
    pub fn parse<T>(&self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        serde_json::from_value(self.source.clone().unwrap_or_default())
    }
}

/// Term suggester response item option
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TermSuggestOption {
    /// Suggested text
    pub text: String,

    /// Suggest score
    pub score: f32,

    /// Term frequency
    #[serde(rename = "freq")]
    pub frequency: u64,
}

/// Phrase suggester response item option
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhraseSuggestOption {
    /// Suggested text
    pub text: String,

    /// Suggest score
    pub score: f32,

    /// Phrase suggestions only, true if matching documents for the collate query were found
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    pub collate_match: Option<bool>,

    /// Highlighted version of text
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    pub highlighted: Option<String>,
}
