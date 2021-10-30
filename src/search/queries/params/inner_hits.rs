use crate::search::*;
use crate::util::*;

/// The [parent-join](https://www.elastic.co/guide/en/elasticsearch/reference/current/parent-join.html)
/// and [nested](https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html)
/// features allow the return of documents that have matches in a different scope. In the
/// parent/child case, parent documents are returned based on matches in child documents or
/// child documents are returned based on matches in parent documents. In the nested case,
/// documents are returned based on matches in nested inner objects.
///
/// In both cases, the actual matches in the different scopes that caused a document to be
/// returned are hidden. In many cases, it’s very useful to know which inner nested objects
/// (in the case of nested) or children/parent documents (in the case of parent/child) caused
/// certain information to be returned. The inner hits feature can be used for this. This
/// feature returns per search hit in the search response additional nested hits that caused a
/// search hit to match in a different scope.
///
/// Inner hits can be used by defining an `inner_hits` definition on a `nested`, `has_child`
/// or `has_parent` query and filter.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/inner-hits.html>
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct InnerHits {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _source: Option<SourceFilter>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    from: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    size: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    sort: Vec<Sort>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    highlight: Option<Highlight>,
}

impl InnerHits {
    /// Creates a new instance of [InnerHits](InnerHits)
    pub fn new() -> Self {
        Default::default()
    }

    /// Indicates which source fields are returned for matching documents
    pub fn source(mut self, source: impl Into<SourceFilter>) -> Self {
        self._source = Some(source.into());
        self
    }

    /// Starting document offset.
    ///
    /// Defaults to `0`.
    pub fn from(mut self, from: impl Into<u64>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// The number of hits to return.
    ///
    /// Defaults to `10`.
    pub fn size(mut self, size: impl Into<u64>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// A collection of sorting fields
    pub fn sort(mut self, sort: impl Into<Vec<Sort>>) -> Self {
        self.sort.extend(sort.into());
        self
    }

    /// Highlight
    pub fn highlight(mut self, highlight: impl Into<Highlight>) -> Self {
        self.highlight = Some(highlight.into());
        self
    }
}
