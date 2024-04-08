use crate::search::*;
use crate::util::*;
use crate::Set;

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
    name: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    from: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    size: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    sort: SortCollection,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    highlight: Option<Highlight>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    docvalue_fields: Set<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    collapse: Option<InnerHitsCollapse>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
struct InnerHitsCollapse {
    field: String,
}

impl InnerHits {
    /// Creates a new instance of [InnerHits](InnerHits)
    pub fn new() -> Self {
        Default::default()
    }

    /// Indicates which source fields are returned for matching documents
    pub fn source<T>(mut self, source: T) -> Self
    where
        T: Into<SourceFilter>,
    {
        self._source = Some(source.into());
        self
    }

    /// Inner hit name, useful when multiple `inner_hits` exist in a single search request
    pub fn name<T>(mut self, name: T) -> Self
    where
        T: ToString,
    {
        self.name = Some(name.to_string());
        self
    }

    /// Starting document offset.
    ///
    /// Defaults to `0`.
    pub fn from(mut self, from: u64) -> Self {
        self.from = Some(from);
        self
    }

    /// The number of hits to return.
    ///
    /// Defaults to `10`.
    pub fn size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    /// A collection of sorting fields
    pub fn sort<T>(mut self, sort: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Sort>,
    {
        self.sort.extend(sort);
        self
    }

    /// Highlight
    pub fn highlight<T>(mut self, highlight: T) -> Self
    where
        T: Into<Highlight>,
    {
        self.highlight = Some(highlight.into());
        self
    }

    /// A collection of docvalue fields
    pub fn docvalue_fields<T>(mut self, docvalue_fields: T) -> Self
    where
        T: IntoIterator,
        T::Item: ToString,
    {
        self.docvalue_fields
            .extend(docvalue_fields.into_iter().map(|x| x.to_string()));
        self
    }

    /// A field to collapse by
    pub fn collapse<T>(mut self, collapse: T) -> Self
    where
        T: ToString,
    {
        self.collapse = Some(InnerHitsCollapse {
            field: collapse.to_string(),
        });
        self
    }
}
