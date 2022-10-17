use serde_json::Value;

/// Number of matching terms to be required
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TermsSetMinimumShouldMatch {
    /// [Numeric](https://www.elastic.co/guide/en/elasticsearch/reference/current/number.html)
    /// field containing the number of matching terms required to return a document.
    #[serde(rename = "minimum_should_match_field")]
    Field(String),

    /// Custom script containing the number of matching terms required to return a document.
    ///
    /// For parameters and valid values, see
    /// [Scripting](https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html).
    ///
    /// For an example query using the `minimum_should_match_script` parameter, see
    /// [How to use the `minimum_should_match_script` parameter](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-terms-set-query.html#terms-set-query-script).
    #[serde(rename = "minimum_should_match_script")]
    Script(TermsSetScript),
}

impl From<String> for TermsSetMinimumShouldMatch {
    fn from(field: String) -> Self {
        Self::Field(field)
    }
}

impl<'a> From<&'a str> for TermsSetMinimumShouldMatch {
    fn from(field: &'a str) -> Self {
        Self::Field(field.to_string())
    }
}

impl From<TermsSetScript> for TermsSetMinimumShouldMatch {
    fn from(script: TermsSetScript) -> Self {
        Self::Script(script)
    }
}

/// Custom script containing the number of matching terms required to return a document.
///
/// For parameters and valid values, see
/// [Scripting](https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TermsSetScript {
    source: String,
    params: Option<Value>,
}

impl From<String> for TermsSetScript {
    fn from(source: String) -> Self {
        Self {
            source,
            params: None,
        }
    }
}

impl<'a> From<&'a str> for TermsSetScript {
    fn from(source: &'a str) -> Self {
        Self {
            source: source.to_string(),
            params: None,
        }
    }
}

impl TermsSetScript {
    /// Creates an instance of [TermsSetScript]
    pub fn new<T>(source: T) -> Self
    where
        T: ToString,
    {
        Self {
            source: source.to_string(),
            params: None,
        }
    }

    /// Assign params
    pub fn params(mut self, params: Value) -> Self {
        self.params = Some(params);
        self
    }
}
