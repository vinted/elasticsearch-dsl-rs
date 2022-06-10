use std::borrow::Cow;

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

// -- Boolean

impl From<bool> for SourceFilter {
    fn from(b: bool) -> Self {
        SourceFilter::Enable(b)
    }
}

// -- Include single field only

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

impl From<Cow<'_, str>> for SourceFilter {
    fn from(include: Cow<'_, str>) -> Self {
        SourceFilter::Include(include.to_string())
    }
}

// -- Include multiple fields

impl From<Vec<String>> for SourceFilter {
    fn from(includes: Vec<String>) -> Self {
        SourceFilter::Includes(includes)
    }
}

impl From<Vec<&str>> for SourceFilter {
    fn from(includes: Vec<&str>) -> Self {
        SourceFilter::Includes(includes.iter().map(ToString::to_string).collect())
    }
}

impl From<Vec<Cow<'_, str>>> for SourceFilter {
    fn from(includes: Vec<Cow<'_, str>>) -> Self {
        SourceFilter::Includes(includes.iter().map(ToString::to_string).collect())
    }
}

impl<const N: usize> From<[String; N]> for SourceFilter {
    fn from(includes: [String; N]) -> Self {
        SourceFilter::Includes(includes.to_vec())
    }
}

impl<const N: usize> From<[&str; N]> for SourceFilter {
    fn from(includes: [&str; N]) -> Self {
        SourceFilter::Includes(includes.iter().map(ToString::to_string).collect())
    }
}

impl<const N: usize> From<[Cow<'_, str>; N]> for SourceFilter {
    fn from(includes: [Cow<'_, str>; N]) -> Self {
        SourceFilter::Includes(includes.iter().map(ToString::to_string).collect())
    }
}

// -- Include exclude fields

impl From<(Vec<String>, Vec<String>)> for SourceFilter {
    fn from((includes, excludes): (Vec<String>, Vec<String>)) -> Self {
        SourceFilter::IncludesExcludes { includes, excludes }
    }
}

impl From<(Vec<&str>, Vec<&str>)> for SourceFilter {
    fn from((includes, excludes): (Vec<&str>, Vec<&str>)) -> Self {
        SourceFilter::IncludesExcludes {
            includes: includes.iter().map(ToString::to_string).collect(),
            excludes: excludes.iter().map(ToString::to_string).collect(),
        }
    }
}

impl From<(Vec<Cow<'_, str>>, Vec<Cow<'_, str>>)> for SourceFilter {
    fn from((includes, excludes): (Vec<Cow<'_, str>>, Vec<Cow<'_, str>>)) -> Self {
        SourceFilter::IncludesExcludes {
            includes: includes.iter().map(ToString::to_string).collect(),
            excludes: excludes.iter().map(ToString::to_string).collect(),
        }
    }
}

impl<const M: usize, const N: usize> From<([String; M], [String; N])> for SourceFilter {
    fn from((includes, excludes): ([String; M], [String; N])) -> Self {
        SourceFilter::IncludesExcludes {
            includes: includes.to_vec(),
            excludes: excludes.to_vec(),
        }
    }
}

impl<const M: usize, const N: usize> From<([&str; M], [&str; N])> for SourceFilter {
    fn from((includes, excludes): ([&str; M], [&str; N])) -> Self {
        SourceFilter::IncludesExcludes {
            includes: includes.iter().map(ToString::to_string).collect(),
            excludes: excludes.iter().map(ToString::to_string).collect(),
        }
    }
}

impl<const M: usize, const N: usize> From<([Cow<'_, str>; M], [Cow<'_, str>; N])> for SourceFilter {
    fn from((includes, excludes): ([Cow<'_, str>; M], [Cow<'_, str>; N])) -> Self {
        SourceFilter::IncludesExcludes {
            includes: includes.iter().map(ToString::to_string).collect(),
            excludes: excludes.iter().map(ToString::to_string).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{util::assert_serialize, Search};

    #[test]
    fn adds_boolean() {
        assert_serialize(
            Search::new().source(false),
            json!({
                "_source": false
            }),
        );
    }

    #[test]
    fn adds_includes() {
        assert_serialize(
            Search::new().source(["abc"]),
            json!({
                "_source": ["abc"]
            }),
        );
    }

    #[test]
    fn adds_includes_excludes() {
        assert_serialize(
            Search::new().source((["abc"], ["def"])),
            json!({
                "_source": {
                    "includes": ["abc"],
                    "excludes": ["def"]
                }
            }),
        );
    }
}
