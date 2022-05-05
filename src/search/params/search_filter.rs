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

impl<T> From<Vec<T>> for SourceFilter
where
    T: ToString,
{
    fn from(includes: Vec<T>) -> Self {
        SourceFilter::Includes(includes.iter().map(ToString::to_string).collect())
    }
}

impl<const N: usize, T> From<[T; N]> for SourceFilter
where
    T: ToString,
{
    fn from(includes: [T; N]) -> Self {
        SourceFilter::Includes(includes.iter().map(ToString::to_string).collect())
    }
}

impl<I, E> From<(I, E)> for SourceFilter
where
    I: IntoIterator,
    I::Item: ToString,
    E: IntoIterator,
    E::Item: ToString,
{
    fn from((includes, excludes): (I, E)) -> Self {
        SourceFilter::IncludesExcludes {
            includes: includes.into_iter().map(|x| x.to_string()).collect(),
            excludes: excludes.into_iter().map(|x| x.to_string()).collect(),
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
