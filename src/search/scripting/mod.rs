//! With scripting, you can evaluate custom expressions in Elasticsearch. For example, you can use
//! a script to return a computed value as a field or evaluate a custom score for a query.
//!
//! The default scripting language is [Painless](https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting-painless.html).
//! Additional `lang` plugins are available to run scripts written in other languages. You can
//! specify the language of the script anywhere that scripts run.
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html>

use crate::util::*;
use crate::Query;
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;

/// Wherever scripting is supported in the Elasticsearch APIs, the syntax follows the same pattern;
/// you specify the language of your script, provide the script logic (or source, and add parameters
/// that are passed into the script.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting-using.html>
#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Script {
    #[serde(rename = "script")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
struct Inner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    lang: Option<ScriptLang>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    source: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    id: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    params: BTreeMap<String, serde_json::Value>,
}

impl Query {
    /// Creates an instance of [`Script`]
    pub fn script() -> Script {
        Script::default()
    }
}

impl Script {
    /// Specifies the language the script is written in. Defaults to `painless`.
    pub fn lang(mut self, lang: impl Into<ScriptLang>) -> Self {
        self.inner.lang = Some(lang.into());
        self
    }

    /// The script itself, which you specify as `source` for an inline script.
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.inner.source = Some(source.into());
        self
    }

    /// The script itself, which you specify as id for a stored script. Use the [stored script APIs](https://www.elastic.co/guide/en/elasticsearch/reference/current/script-apis.html#stored-script-apis)
    /// to create and manage stored scripts.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.inner.id = Some(id.into());
        self
    }

    /// Specifies any named parameters that are passed into the script as variables. [Use parameters](https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting-using.html#prefer-params)
    /// instead of hard-coded values to decrease compile time.
    pub fn param<T>(mut self, name: impl Into<String>, param: T) -> Self
    where
        T: Serialize,
    {
        if let Ok(param) = serde_json::to_value(param) {
            let _ = self.inner.params.entry(name.into()).or_insert(param);
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

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(Query::script(), json!({ "script": {}}));

        with_all_fields(
            Query::script()
            .source("Math.log(_score * 2) * params['multiplier'].len()")
            .param("multiplier", vec![ 1, 2, 3])
            .lang(ScriptLang::Painless),
            json!({ "script": {
                "source": "Math.log(_score * 2) * params['multiplier'].len()",
                "lang": "painless",
                "params": {
                    "multiplier": [ 1, 2, 3]
                }
            }})
        );

        with_all_fields_custom_script_lang(
            Query::script()
            .source("doc['my_field'].value * params['multiplier']")
            .param("multiplier", 1)
            .lang(ScriptLang::Custom("my_lang".into())),
            json!({ "script": {
                "source": "doc['my_field'].value * params['multiplier']",
                "lang": "my_lang",
                "params": {
                    "multiplier": 1
                }
            }})
        );
    }
}
