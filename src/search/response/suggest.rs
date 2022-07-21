use super::SuggestOption;

/// Suggester response item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Suggest {
    /// Suggestion text
    pub text: String,

    /// Suggestion length
    pub length: u64,

    /// Suggestion offset
    pub offset: u64,

    /// Suggestion options
    pub options: Vec<SuggestOption>,
}
