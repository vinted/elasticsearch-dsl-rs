/// Control how the `_source` field is returned with every hit.
///
/// By default operations return the contents of the `_source` field
/// unless you have used the `stored_fields` parameter or if the `_source` field is disabled.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceFilter {
    /// Whether `_source` retrieval should be enabled (`true`) or disabled (`false`)
    Enable(bool),

    /// A wildcard pattern to control what parts of `_source` should be returned
    Include(String),

    /// A collection of wildcard patterns to control what parts of `_source` should be returned
    Includes(Vec<String>),

    /// A collection of wildcard patterns to control what parts of `_source` should
    /// and should not be returned
    IncludesExcludes {
        /// A collection of wildcard patterns to control what parts of `_source` should be returned
        includes: Vec<String>,

        /// A collection of wildcard patterns to control what parts of `_source` should not be
        /// returned
        excludes: Vec<String>,
    },
}

impl From<bool> for SourceFilter {
    fn from(b: bool) -> Self {
        SourceFilter::Enable(b)
    }
}

impl From<String> for SourceFilter {
    fn from(include: String) -> Self {
        SourceFilter::Include(include)
    }
}

impl<'a> From<&'a str> for SourceFilter {
    fn from(include: &'a str) -> Self {
        SourceFilter::Include(include.to_owned())
    }
}

impl From<Vec<String>> for SourceFilter {
    fn from(includes: Vec<String>) -> Self {
        SourceFilter::Includes(includes)
    }
}

impl<'a> From<Vec<&'a str>> for SourceFilter {
    fn from(includes: Vec<&'a str>) -> Self {
        SourceFilter::Includes(includes.iter().map(ToString::to_string).collect())
    }
}

impl<const N: usize> From<[String; N]> for SourceFilter {
    fn from(includes: [String; N]) -> Self {
        SourceFilter::Includes(includes.to_vec())
    }
}

impl<'a, const N: usize> From<[&'a str; N]> for SourceFilter {
    fn from(includes: [&'a str; N]) -> Self {
        SourceFilter::Includes(includes.iter().map(ToString::to_string).collect())
    }
}

impl From<(Vec<String>, Vec<String>)> for SourceFilter {
    fn from(includes_excludes: (Vec<String>, Vec<String>)) -> Self {
        SourceFilter::IncludesExcludes {
            includes: includes_excludes.0,
            excludes: includes_excludes.1,
        }
    }
}

impl<'a> From<(Vec<&'a str>, Vec<&'a str>)> for SourceFilter {
    fn from(includes_excludes: (Vec<&'a str>, Vec<&'a str>)) -> Self {
        SourceFilter::IncludesExcludes {
            includes: includes_excludes
                .0
                .iter()
                .map(ToString::to_string)
                .collect(),
            excludes: includes_excludes
                .1
                .iter()
                .map(ToString::to_string)
                .collect(),
        }
    }
}
