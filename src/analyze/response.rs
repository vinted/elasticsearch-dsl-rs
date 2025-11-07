use serde::{de, Deserialize, Serialize, Serializer};

/// Elasticsearch analyze API response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalyzeResponse {
    /// Standard response, when `explain` value is `false`
    #[serde(rename = "tokens")]
    Standard(Vec<Token>),

    /// Explained response, when `explain` value is `true`
    #[serde(rename = "detail")]
    Explained(ExplainedResponse),
}

/// Extracted token from text using tokenizer
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Token {
    /// The characters of the current token
    pub token: String,

    /// The start offset of the current token
    pub start_offset: u32,

    /// The end offset of the current token
    pub end_offset: u32,

    /// The type of the current token
    #[serde(rename = "type")]
    pub ty: TokenType,

    /// The position of the current token
    pub position: u32,

    /// Token in bytes
    pub bytes: Option<String>,

    /// Whether or not the current token is marked as a keyword
    pub keyword: Option<bool>,

    /// The position length of the current token
    pub position_length: Option<u32>,

    /// Term frequency in given text analysis
    pub term_frequency: Option<u32>,
}

/// Explained response structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ExplainedResponse {
    custom_analyzer: bool,

    analyzer: Option<AnalysisObject>,

    #[serde(default, rename = "charfilters")]
    char_filters: Vec<CharFilter>,

    tokenizer: Option<AnalysisObject>,

    #[serde(default, rename = "tokenfilters")]
    token_filters: Vec<AnalysisObject>,
}

/// Structure for analyzer, tokenizer and token filters
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AnalysisObject {
    name: String,
    tokens: Vec<Token>,
}

/// Structure for char filters
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CharFilter {
    name: String,
    filtered_text: Vec<String>,
}

/// Type of token
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TokenType {
    /// Alphanumeric token
    #[default]
    Alphanum,

    /// Synonym token
    Synonym,

    /// Word token
    Word,

    /// Hangul (Korean alphabet) token
    Hangul,

    /// Numeric token
    Num,

    /// Email token
    Email,

    /// Words with apostrophe token
    Apostrophe,

    /// CJK (Chinese, Japanese, and Korean) tokens
    Double,

    /// Normalized CJK (Chinese, Japanese, and Korean) tokens.
    /// Normalizes width differences in CJK (Chinese, Japanese, and Korean) characters as follows:
    /// Folds full-width ASCII character variants into the equivalent basic Latin characters
    /// Folds half-width Katakana character variants into the equivalent Kana characters
    Katakana,

    /// Acronym token
    Acronym,

    /// Gram token
    Gram,

    /// Fingerprint token
    Fingerprint,

    /// Shingle token
    Shingle,

    /// Other token
    Other(String),
}

impl Serialize for TokenType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Alphanum => "<ALPHANUM>",
            Self::Synonym => "SYNONYM",
            Self::Word => "word",
            Self::Hangul => "<HANGUL>",
            Self::Num => "<NUM>",
            Self::Email => "<EMAIL>",
            Self::Apostrophe => "<APOSTROPHE>",
            Self::Double => "<DOUBLE>",
            Self::Katakana => "<KATAKANA>",
            Self::Acronym => "<ACRONYM>",
            Self::Gram => "gram",
            Self::Fingerprint => "fingerprint",
            Self::Shingle => "shingle",
            Self::Other(other) => other,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TokenType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(match String::deserialize(deserializer)?.as_str() {
            "<ALPHANUM>" => Self::Alphanum,
            "SYNONYM" => Self::Synonym,
            "word" => Self::Word,
            "<HANGUL>" => Self::Hangul,
            "<NUM>" => Self::Num,
            "<EMAIL>" => Self::Email,
            "<APOSTROPHE>" => Self::Apostrophe,
            "<DOUBLE>" => Self::Double,
            "<KATAKANA>" => Self::Katakana,
            "<ACRONYM>" => Self::Acronym,
            "gram" => Self::Gram,
            "fingerprint" => Self::Fingerprint,
            "shingle" => Self::Shingle,
            other => Self::Other(other.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserialize_standard() {
        let json_response = json!({
            "tokens": [
                {
                    "token": "test1",
                    "start_offset": 0,
                    "end_offset": 6,
                    "type": "<ALPHANUM>",
                    "position": 0
                },
                {
                    "token": "test2",
                    "start_offset": 7,
                    "end_offset": 11,
                    "type": "<ALPHANUM>",
                    "position": 1
                }
            ]
        });

        let token_1 = Token {
            token: "test1".to_string(),
            start_offset: 0,
            end_offset: 6,
            ty: TokenType::Alphanum,
            position: 0,
            bytes: None,
            keyword: None,
            position_length: None,
            term_frequency: None,
        };
        let token_2 = Token {
            token: "test2".to_string(),
            start_offset: 7,
            end_offset: 11,
            ty: TokenType::Alphanum,
            position: 1,
            bytes: None,
            keyword: None,
            position_length: None,
            term_frequency: None,
        };

        let expected = AnalyzeResponse::Standard(vec![token_1, token_2]);
        let result: AnalyzeResponse = serde_json::from_value(json_response).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn deserialize_explained() {
        let json_response = json!({
            "detail": {
                "custom_analyzer": true,
                "charfilters": [
                    {
                        "name": "html_strip",
                        "filtered_text": [
                            "test"
                        ]
                    }
                ],
                "tokenizer": {
                    "name": "lowercase",
                    "tokens": [
                        {
                            "token": "test",
                            "start_offset": 0,
                            "end_offset": 6,
                            "type": "SYNONYM",
                            "position": 0
                        }
                    ]
                },
                "tokenfilters": [
                    {
                        "name": "__anonymous__stop",
                        "tokens": [
                            {
                                "token": "test",
                                "start_offset": 0,
                                "end_offset": 6,
                                "type": "SYNONYM",
                                "position": 0
                            }
                        ]
                    }
                ]
            }
        });

        let token = Token {
            token: "test".to_string(),
            start_offset: 0,
            end_offset: 6,
            ty: TokenType::Synonym,
            position: 0,
            bytes: None,
            keyword: None,
            position_length: None,
            term_frequency: None,
        };

        let expected = AnalyzeResponse::Explained(ExplainedResponse {
            custom_analyzer: true,
            analyzer: None,
            char_filters: vec![CharFilter {
                name: "html_strip".to_string(),
                filtered_text: vec!["test".to_string()],
            }],
            tokenizer: Some(AnalysisObject {
                name: "lowercase".to_string(),
                tokens: vec![token.clone()],
            }),
            token_filters: vec![AnalysisObject {
                name: "__anonymous__stop".to_string(),
                tokens: vec![token],
            }],
        });

        let result: AnalyzeResponse = serde_json::from_value(json_response).unwrap();

        assert_eq!(expected, result);
    }
}
