//! Analyze API query.
//! <https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-analyze.html#analyze-api-query-params>
mod response;

use serde::ser::{Serialize, SerializeStruct, Serializer};

pub use self::response::*;
use crate::util::ShouldSkip;

/// Performs analysis on a text string and returns the resulting tokens.
/// The basic `analyze`:
/// ```
/// # use elasticsearch_dsl::analyze::*;
/// # let query = Analyze::new("test this text");
/// ```
/// To `analyze` with custom analyzer:
/// ```
/// # use elasticsearch_dsl::analyze::*;
/// # use serde_json::json;
/// let custom_analyzer = CustomAnalyzer::new("whitespace")
///    .filter([
///        StringOrObject::String("lowercase".to_string()),
///        StringOrObject::Object(json!({"type": "stop", "stopwords": ["a", "is", "this"]})),
///    ]);
/// let test = Analyze::new(["test this text", "and this one please"])
///    .analyzer(custom_analyzer)
///    .explain(true)
///    .attributes(["attributes"]);
/// ```
/// To `analyze` custom normalizer:
/// ```
/// # use elasticsearch_dsl::analyze::*;
/// # use serde_json::json;
/// let custom_normalizer = CustomNormalizer::new()
///    .char_filter([
///        json!({ "type": "mapping", "mappings": ["٠ => 0", "١ => 1", "٢ => 2"] }),
///    ])
///    .filter(["snowball"]);
/// let test = Analyze::new(["test this text", "and this one please"])
///    .analyzer(custom_normalizer)
///    .explain(true)
///    .attributes(["attributes"]);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct Analyze {
    text: StringOrVecString,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip", flatten)]
    analyzer: Option<Analysis>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    attributes: Vec<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    explain: Option<bool>,
}

/// Structure of custom analyzer.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct CustomAnalyzer {
    tokenizer: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    char_filter: Vec<StringOrObject>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    filter: Vec<StringOrObject>,
}

/// Structure of custom normalizer
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct CustomNormalizer {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    char_filter: Vec<StringOrObject>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    filter: Vec<StringOrObject>,
}

/// Analysis types
#[derive(Debug, Clone, PartialEq)]
pub enum Analysis {
    /// The name of the analyzer that should be applied to the provided text.
    /// This could be a `built-in analyzer`, or an analyzer that’s been configured in the index.
    /// If this parameter is not specified, the analyze API uses the analyzer defined in the field’s mapping.
    /// If no field is specified, the analyze API uses the default analyzer for the index.
    /// If no index is specified, or the index does not have a default analyzer, the analyze API uses the `standard analyzer`.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-analyzers.html>
    BuiltInAnalyzer(String),

    /// Custom analyzer that should be applied to the provided text.
    CustomAnalyzer(CustomAnalyzer),

    /// The name of built-in normalizer to use to convert text into a single token.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-normalizers.html>
    BuiltInNormalizer(String),

    /// The custom normalizer to use to convert text into a single token.
    CustomNormalizer(CustomNormalizer),

    /// Field used to derive the analyzer. To use this parameter, you must specify an index.
    /// If specified, the analyzer parameter overrides this value.
    /// If no field is specified, the analyze API uses the default analyzer for the index.
    /// If no index is specified or the index does not have a default analyzer, the analyze API uses the `standard analyzer`.
    Field(String),
}

/// Structure of filters
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum StringOrObject {
    /// Built-in filters
    String(String),

    /// Custom filters
    Object(serde_json::Value),
}

/// Type for text field. Text can be string or array of strings
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum StringOrVecString {
    /// One text input to analyze
    String(String),

    /// Multiple text inputs to analyze
    VecString(Vec<String>),
}

impl Analyze {
    /// Creates an instance of [Analyze]
    ///
    /// - `text` - Text to analyze. If an array of strings is provided, it is analyzed as a multi-value field.
    pub fn new(text: impl Into<StringOrVecString>) -> Self {
        Self {
            text: text.into(),
            analyzer: None,
            attributes: vec![],
            explain: None,
        }
    }

    /// Specify an analyzer, either it's buil-in analyzer, custom analyzer, built-in normalizer,
    /// custom normalizer or field
    pub fn analyzer(mut self, analyzer: impl Into<Analysis>) -> Self {
        self.analyzer = Some(analyzer.into());

        self
    }

    /// Array of token attributes used to filter the output of the explain parameter.
    pub fn attributes<I>(mut self, attributes: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        self.attributes = attributes.into_iter().map(Into::into).collect();

        self
    }

    /// If `true`, the response includes token attributes and additional details. Defaults to `false`. `experimental`
    pub fn explain(mut self, explain: bool) -> Self {
        self.explain = Some(explain);

        self
    }
}

impl CustomNormalizer {
    /// Create instance of custom normalizer
    pub fn new() -> Self {
        Default::default()
    }

    /// Array of character filters used to preprocess characters before the tokenizer.
    /// See `Character filters reference` for a list of character filters.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-charfilters.html>
    pub fn char_filter<I>(mut self, char_filter: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<StringOrObject>,
    {
        self.char_filter
            .extend(char_filter.into_iter().map(Into::into));

        self
    }

    /// Array of token filters used to apply after the tokenizer.
    /// See `Token filter reference` for a list of token filters.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-tokenfilters.html>
    pub fn filter<I>(mut self, filter: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<StringOrObject>,
    {
        self.filter.extend(filter.into_iter().map(Into::into));

        self
    }
}

impl CustomAnalyzer {
    /// Create instance of custom analyzer and sets tokenizer
    /// Tokenizer to use to convert text into tokens. See `Tokenizer reference` for a list of tokenizers.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-tokenizers.html>
    pub fn new(tokenizer: impl Into<String>) -> Self {
        Self {
            tokenizer: tokenizer.into(),
            char_filter: vec![],
            filter: vec![],
        }
    }

    /// Array of character filters used to preprocess characters before the tokenizer.
    /// See `Character filters reference` for a list of character filters.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-charfilters.html>
    pub fn char_filter<I>(mut self, char_filter: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<StringOrObject>,
    {
        self.char_filter
            .extend(char_filter.into_iter().map(Into::into));

        self
    }

    /// Array of token filters used to apply after the tokenizer.
    /// See `Token filter reference` for a list of token filters.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/analysis-tokenfilters.html>
    pub fn filter<I>(mut self, filter: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<StringOrObject>,
    {
        self.filter.extend(filter.into_iter().map(Into::into));

        self
    }
}

impl Analysis {
    /// Creates an instance of [`Analysis::Field`]
    pub fn field(value: impl Into<String>) -> Self {
        Self::Field(value.into())
    }

    /// Creates an instance of [`Analysis::BuiltInAnalyzer`]
    pub fn analyzer(value: impl Into<String>) -> Self {
        Self::BuiltInAnalyzer(value.into())
    }

    /// Creates an instance of [`Analysis::BuiltInNormalizer`]
    pub fn normalizer(value: impl Into<String>) -> Self {
        Self::BuiltInNormalizer(value.into())
    }
}

impl<'a> From<&'a str> for StringOrObject {
    fn from(value: &'a str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for StringOrObject {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<serde_json::Value> for StringOrObject {
    fn from(value: serde_json::Value) -> Self {
        Self::Object(value)
    }
}

impl From<CustomAnalyzer> for Analysis {
    fn from(value: CustomAnalyzer) -> Self {
        Self::CustomAnalyzer(value)
    }
}

impl From<CustomNormalizer> for Analysis {
    fn from(value: CustomNormalizer) -> Self {
        Self::CustomNormalizer(value)
    }
}

impl From<String> for StringOrVecString {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for StringOrVecString {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<Vec<&str>> for StringOrVecString {
    fn from(value: Vec<&str>) -> Self {
        Self::VecString(value.into_iter().map(Into::into).collect())
    }
}

impl<const N: usize> From<[&str; N]> for StringOrVecString {
    fn from(value: [&str; N]) -> Self {
        Self::VecString(value.iter().map(ToString::to_string).collect())
    }
}

impl<'a> From<&'a [&str]> for StringOrVecString {
    fn from(value: &'a [&str]) -> Self {
        Self::VecString(value.iter().map(ToString::to_string).collect())
    }
}

impl Serialize for Analysis {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Analysis::BuiltInAnalyzer(name) => {
                let mut state = serializer.serialize_struct("analysis_analyzer", 1)?;
                state.serialize_field("analyzer", name)?;
                state.end()
            }
            Analysis::CustomAnalyzer(analyzer) => analyzer.serialize(serializer),
            Analysis::BuiltInNormalizer(name) => {
                let mut state = serializer.serialize_struct("analysis_normalizer", 1)?;
                state.serialize_field("normalizer", name)?;
                state.end()
            }
            Analysis::CustomNormalizer(normalizer) => normalizer.serialize(serializer),
            Analysis::Field(name) => {
                let mut state = serializer.serialize_struct("analysis_field", 1)?;
                state.serialize_field("field", name)?;
                state.end()
            }
        }
    }
}

impl Default for StringOrVecString {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            Analyze::new("analyze these pants"),
            json!({
                "text": "analyze these pants"
            })
        );

        with_built_in_analyzer(
            Analyze::new("analyze these pants")
                .analyzer(
                    Analysis::analyzer("test_default")
                ),
            json!({
                "text": "analyze these pants",
                "analyzer": "test_default"
            })
        );

        with_custom_analyzer(
            Analyze::new(["here is one to test", "and here is another one"])
                .analyzer(CustomAnalyzer::new("lowercase")
                    .char_filter(["html_strip", "test_strip"])
                    .filter([json!({"type": "stop", "stopwords": ["a", "is", "this"]})]))
                .attributes(["score", "keyword"])
                .explain(true),
            json!({
                "attributes": [
                    "score",
                    "keyword"
                ],
                "char_filter": [
                    "html_strip",
                    "test_strip"
                ],
                "filter" : [{"type": "stop", "stopwords": ["a", "is", "this"]}],
                "tokenizer": "lowercase",
                "explain": true,
                "text": ["here is one to test", "and here is another one"]
            })
        );

        with_built_in_normalizer(
            Analyze::new("analyze these pants")
                .analyzer(
                    Analysis::normalizer("asciifolding")
                ),
            json!({
                "text": "analyze these pants",
                "normalizer": "asciifolding"
            })
        );

        with_custom_normalizer(
            Analyze::new(["here is one to test", "and here is another one"])
                .analyzer(CustomNormalizer::new()
                    .char_filter(["html_strip", "test_strip"])
                    .filter([json!({"type": "stop", "stopwords": ["a", "is", "this"]})]))
                .attributes(["score", "keyword"])
                .explain(true),
            json!({
                "attributes": [
                    "score",
                    "keyword"
                ],
                "char_filter": [
                    "html_strip",
                    "test_strip"
                ],
                "filter" : [{"type": "stop", "stopwords": ["a", "is", "this"]}],
                "explain": true,
                "text": ["here is one to test", "and here is another one"]
            })
        );

        with_field(
            Analyze::new("analyze these pants")
                .analyzer(
                    Analysis::field("title")
                ),
            json!({
                "text": "analyze these pants",
                "field": "title"
            })
        );
    }
}
