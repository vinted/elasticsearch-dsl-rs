use crate::util::ShouldSkip;

/// Filter the values for which buckets will be created.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum TermsInclude {
    /// Filter buckets by their regular expression pattern.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-terms-aggregation.html#_filtering_values_with_regular_expressions_2>
    Regex(String),

    /// Filter buckets by their exact value.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-terms-aggregation.html#_filtering_values_with_exact_values_2>
    Exact(Vec<String>),

    /// A number of partitions at query-time and processing only one partition in each request.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-terms-aggregation.html#_filtering_values_with_partitions>
    Partitions {
        /// The partition number to return.
        partition: u32,
        /// The number of partitions to create.
        num_partitions: u32,
    },
}

impl ShouldSkip for TermsInclude {
    fn should_skip(&self) -> bool {
        match self {
            Self::Regex(ref s) => s.is_empty(),
            Self::Exact(ref v) => v.is_empty(),
            Self::Partitions { .. } => false,
        }
    }
}

impl From<String> for TermsInclude {
    fn from(s: String) -> Self {
        Self::Regex(s)
    }
}

impl From<&str> for TermsInclude {
    fn from(s: &str) -> Self {
        Self::Regex(s.to_string())
    }
}

impl From<Vec<String>> for TermsInclude {
    fn from(v: Vec<String>) -> Self {
        Self::Exact(v)
    }
}

impl From<Vec<&str>> for TermsInclude {
    fn from(v: Vec<&str>) -> Self {
        Self::Exact(v.iter().map(|s| s.to_string()).collect())
    }
}

impl From<&[&str]> for TermsInclude {
    fn from(v: &[&str]) -> Self {
        Self::Exact(v.iter().map(|s| s.to_string()).collect())
    }
}

impl<const N: usize> From<[&str; N]> for TermsInclude {
    fn from(value: [&str; N]) -> Self {
        Self::Exact(value.iter().map(|s| s.to_string()).collect())
    }
}

impl From<(u32, u32)> for TermsInclude {
    fn from(value: (u32, u32)) -> Self {
        Self::Partitions {
            partition: value.0,
            num_partitions: value.1,
        }
    }
}

impl From<[u32; 2]> for TermsInclude {
    fn from(value: [u32; 2]) -> Self {
        Self::Partitions {
            partition: value[0],
            num_partitions: value[1],
        }
    }
}
