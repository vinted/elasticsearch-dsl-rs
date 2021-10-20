//! Allows you to execute a search query and get back search hits that match the query.
use super::{
    Aggregation, Aggregations, Highlight, Query, Rescore, Sort, SourceFilter, TrackTotalHits,
};
use crate::util::*;

/// Returns search hits that match the query defined in the request.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html>
#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct Search {
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
    sort: Vec<Sort>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    aggs: Aggregations,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    track_total_hits: Option<TrackTotalHits>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    highlight: Option<Highlight>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    rescore: Vec<Rescore>,
}

impl Search {
    /// Creates a default search instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Indicates which source fields are returned for matching documents
    pub fn source(mut self, source: impl Into<SourceFilter>) -> Self {
        self._source = Some(source.into());
        self
    }

    /// Specific `tag` of the request for logging and statistical purposes.
    pub fn stats(mut self, stats: impl Into<String>) -> Self {
        self.stats.push(stats.into());
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

    /// Defines the search definition using the
    /// [Query DSL](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl.html).
    pub fn query(mut self, query: impl Into<Query>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// A collection of sorting fields
    pub fn sort(mut self, sort: impl Into<Vec<Sort>>) -> Self {
        self.sort.extend(sort.into());
        self
    }

    /// Aggregations
    pub fn aggregate(mut self, aggregation: impl Into<Aggregation>) -> Self {
        let aggregation = aggregation.into();
        let _ = self.aggs.entry(aggregation.name()).or_insert(aggregation);
        self
    }

    /// Track total hits
    pub fn track_total_hits(mut self, track_total_hits: impl Into<TrackTotalHits>) -> Self {
        self.track_total_hits = Some(track_total_hits.into());
        self
    }

    /// Highlight
    pub fn highlight(mut self, highlight: impl Into<Highlight>) -> Self {
        self.highlight = Some(highlight.into());
        self
    }

    /// Rescore
    pub fn rescore(mut self, rescore: impl Into<Rescore>) -> Self {
        let rescore = rescore.into();

        if !rescore.should_skip() {
            self.rescore.push(rescore);
        }

        self
    }
}
