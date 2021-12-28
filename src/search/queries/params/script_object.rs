//! With scripting, you can evaluate custom expressions in Elasticsearch. For example, you can use
//! a script to return a computed value as a field or evaluate a custom score for a query.
//!
//! The default scripting language is [Painless](https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting-painless.html).
//! Additional `lang` plugins are available to run scripts written in other languages. You can
//! specify the language of the script anywhere that scripts run.
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html>

use crate::util::*;
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;

/// Wherever scripting is supported in the Elasticsearch APIs, the syntax follows the same pattern;
/// you specify the language of your script, provide the script logic (or source, and add parameters
/// that are passed into the script.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting-using.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Script {
    #[serde(rename = "script")]
    inner: Inner,
}

/// The script itself, which you specify as `source` for an inline script or
/// `id` for a stored script. Use the
/// [stored script APIs](https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting-using.html#prefer-params)
/// to create and manage stored scripts.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptSource {
    /// Inline script
    Source(String),

    /// Stored script
    Id(String),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    #[serde(flatten)]
    source: ScriptSource,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    lang: Option<ScriptLang>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    params: BTreeMap<String, serde_json::Value>,
}

impl Script {
    /// Creates an instance of inlined [`Script`]
    pub fn source<S>(source: S) -> Self
    where
        S: ToString,
    {
        Self {
            inner: Inner {
                source: ScriptSource::Source(source.to_string()),
                lang: None,
                params: BTreeMap::new(),
            },
        }
    }

    /// Creates an instance of stored [`Script`]
    pub fn id<S>(id: S) -> Self
    where
        S: ToString,
    {
        Self {
            inner: Inner {
                source: ScriptSource::Id(id.to_string()),
                lang: None,
                params: BTreeMap::new(),
            },
        }
    }

    /// Specifies the language the script is written in. Defaults to `painless`.
    pub fn lang<S>(mut self, lang: S) -> Self
    where
        S: Into<ScriptLang>,
    {
        self.inner.lang = Some(lang.into());
        self
    }

    /// Specifies any named parameters that are passed into the script as variables. [Use parameters](https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting-using.html#prefer-params)
    /// instead of hard-coded values to decrease compile time.
    pub fn param<T, S>(mut self, name: S, param: T) -> Self
    where
        S: ToString,
        T: Serialize,
    {
        if let Ok(param) = serde_json::to_value(param) {
            let _ = self.inner.params.entry(name.to_string()).or_insert(param);
        }
        self
    }
}
/// Available scripting language
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptLang {
    /// ***Painless*** is a performant, secure scripting language designed specifically for Elasticsearch.
    /// You can use Painless to safely write inline and stored scripts anywhere scripts are supported
    /// in Elasticsearch.
    ///
    /// Painless provides numerous capabilities that center around the following core principles:
    ///
    /// - **Safety**: Ensuring the security of your cluster is of utmost importance. To that end, Painless
    /// uses a fine-grained allowlist with a granularity down to the members of a class. Anything
    /// that is not part of the allowlist results in a compilation error. See the [Painless API
    /// Reference](https://www.elastic.co/guide/en/elasticsearch/painless/7.15/painless-api-reference.html)
    /// for a complete list of available classes, methods, and fields per script context.
    ///
    /// - **Performance**: Painless compiles directly into JVM bytecode to take advantage of all possible
    /// optimizations that the JVM provides. Also, Painless typically avoids features that require
    /// additional slower checks at runtime.
    ///
    /// - **Simplicity**: Painless implements a syntax with a natural familiarity to anyone with some
    /// basic coding experience. Painless uses a subset of Java syntax with some additional
    /// improvements to enhance readability and remove boilerplate.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting-painless.html>
    Painless,
    /// Lucene’s expressions compile a `javascript` expression to bytecode. They are designed for
    /// high-performance custom ranking and sorting functions and are enabled for `inline` and `stored`
    /// scripting by default.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting-expression.html>
    Expression,
    /// A template language to search with templates
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-template.html>
    Mustache,
    /// Custom language refer to [Advanced scripts using script engines](https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting-engine.html)
    Custom(String),
}

impl Serialize for ScriptLang {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Painless => serializer.serialize_str("painless"),
            Self::Expression => serializer.serialize_str("expression"),
            Self::Mustache => serializer.serialize_str("mustache"),
            Self::Custom(lang) => lang.serialize(serializer),
        }
    }
}

impl<T> From<T> for ScriptLang
where
    T: ToString,
{
    fn from(value: T) -> Self {
        let value = value.to_string();

        match value.as_str() {
            "painless" => Self::Painless,
            "expression" => Self::Expression,
            "mustache" => Self::Mustache,
            _ => Self::Custom(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Script::source("Math.log(_score * 2) * params['multiplier'].len()")
                .param("multiplier", [1, 2, 3])
                .lang(ScriptLang::Painless),
            json!({
                "script": {
                    "source": "Math.log(_score * 2) * params['multiplier'].len()",
                    "lang": "painless",
                    "params": {
                        "multiplier": [1, 2, 3]
                    }
                }
            }),
        );

        assert_serialize(
            Script::source("doc['my_field'].value * params['multiplier']")
                .param("multiplier", 1)
                .lang("my_lang"),
            json!({
                "script": {
                    "source": "doc['my_field'].value * params['multiplier']",
                    "lang": "my_lang",
                    "params": {
                        "multiplier": 1
                    }
                }
            }),
        );

        assert_serialize(
            Script::id(123).param("multiplier", [1, 2, 3]),
            json!({
                "script": {
                    "id": "123",
                    "params": {
                        "multiplier": [1, 2, 3]
                    }
                }
            }),
        );
    }
}
