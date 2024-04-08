use crate::util::ShouldSkip;

/// Filter the values for which buckets will be created.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum TermsExclude {
    /// Filter buckets by their regular expression pattern.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-terms-aggregation.html#_filtering_values_with_regular_expressions_2>
    Regex(String),

    /// Filter buckets by their exact value.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-terms-aggregation.html#_filtering_values_with_exact_values_2>
    Exact(Vec<String>),
}

impl ShouldSkip for TermsExclude {
    fn should_skip(&self) -> bool {
        match self {
            Self::Regex(ref s) => s.is_empty(),
            Self::Exact(ref v) => v.is_empty(),
        }
    }
}

impl From<String> for TermsExclude {
    fn from(s: String) -> Self {
        Self::Regex(s)
    }
}

impl From<&str> for TermsExclude {
    fn from(s: &str) -> Self {
        Self::Regex(s.to_string())
    }
}

impl From<Vec<String>> for TermsExclude {
    fn from(v: Vec<String>) -> Self {
        Self::Exact(v)
    }
}

impl From<Vec<&str>> for TermsExclude {
    fn from(v: Vec<&str>) -> Self {
        Self::Exact(v.iter().map(|s| s.to_string()).collect())
    }
}

impl From<&[&str]> for TermsExclude {
    fn from(v: &[&str]) -> Self {
        Self::Exact(v.iter().map(|s| s.to_string()).collect())
    }
}

impl<const N: usize> From<[&str; N]> for TermsExclude {
    fn from(value: [&str; N]) -> Self {
        Self::Exact(value.iter().map(|s| s.to_string()).collect())
    }
}
