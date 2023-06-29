//! Allows you to execute a search query and get back search hits that match the query.
use crate::search::*;
use crate::util::*;
use crate::Map;
use crate::Set;

/// Returns search hits that match the query defined in the request.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html>
#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct Search {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    runtime_mappings: Map<String, RuntimeMapping>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    indices_boost: Vec<KeyValuePair<String, f32>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    min_score: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _source: Option<SourceFilter>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    stats: Vec<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    from: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    size: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    query: Option<Query>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    sort: SortCollection,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    aggs: Aggregations,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    track_total_hits: Option<TrackTotalHits>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    highlight: Option<Highlight>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    rescore: RescoreCollection,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    suggest: Map<String, Suggester>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    stored_fields: StoredFields,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    docvalue_fields: Set<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    post_filter: Option<Query>,
}

impl Search {
    /// Creates a default search instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Add runtime mapping to the search request
    pub fn runtime_mapping<S>(mut self, name: S, mapping: RuntimeMapping) -> Self
    where
        S: ToString,
    {
        let _ = self.runtime_mappings.insert(name.to_string(), mapping);
        self
    }

    /// Allows to configure different boost level per index when searching
    /// across more than one indices. This is very handy when hits coming from
    /// one index matter more than hits coming from another index (think social
    /// graph where each user has an index).
    pub fn indices_boost<T, U>(mut self, field: T, boost: U) -> Self
    where
        T: ToString,
        U: num_traits::AsPrimitive<f32>,
    {
        self.indices_boost
            .push(KeyValuePair::new(field.to_string(), boost.as_()));
        self
    }

    /// Exclude documents which have a `_score` less than the minimum specified
    /// in `min_score`
    ///
    /// Note, most times, this does not make much sense, but is provided for
    /// advanced use cases
    pub fn min_score<F>(mut self, min_score: F) -> Self
    where
        F: Into<f32>,
    {
        self.min_score = Some(min_score.into());
        self
    }

    /// Indicates which source fields are returned for matching documents
    pub fn source<S>(mut self, source: S) -> Self
    where
        S: Into<SourceFilter>,
    {
        self._source = Some(source.into());
        self
    }

    /// Specific `tag` of the request for logging and statistical purposes.
    pub fn stats<S>(mut self, stats: S) -> Self
    where
        S: ToString,
    {
        self.stats.push(stats.to_string());
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

    /// Defines the search definition using the
    /// [Query DSL](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl.html).
    pub fn query<Q>(mut self, query: Q) -> Self
    where
        Q: Into<Query>,
    {
        self.query = Some(query.into());
        self
    }

    /// When you use the `post_filter` parameter to filter search results, the search hits are filtered after the
    /// aggregations are calculated. A post filter has no impact on the aggregation results.
    pub fn post_filter<Q>(mut self, post_filter: Q) -> Self
    where
        Q: Into<Query>,
    {
        self.post_filter = Some(post_filter.into());
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

    /// Track total hits
    pub fn track_total_hits<T>(mut self, track_total_hits: T) -> Self
    where
        T: Into<TrackTotalHits>,
    {
        self.track_total_hits = Some(track_total_hits.into());
        self
    }

    /// Highlight
    pub fn highlight<H>(mut self, highlight: H) -> Self
    where
        H: Into<Highlight>,
    {
        self.highlight = Some(highlight.into());
        self
    }

    /// Rescore
    pub fn rescore<T>(mut self, rescore: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Rescore>,
    {
        self.rescore.extend(rescore);
        self
    }

    /// Suggest
    pub fn suggest<T, U>(mut self, name: T, suggester: U) -> Self
    where
        T: ToString,
        U: Into<Suggester>,
    {
        let _ = self.suggest.insert(name.to_string(), suggester.into());
        self
    }

    /// A collection of stored fields
    pub fn stored_fields<T>(mut self, stored_fields: T) -> Self
    where
        T: Into<StoredFields>,
    {
        self.stored_fields = stored_fields.into();
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

    add_aggregate!();
}
