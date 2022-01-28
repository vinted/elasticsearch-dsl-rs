//! Allows you to execute a search query and get back search hits that match the query.
use crate::search::*;
use crate::util::*;
use std::{collections::BTreeMap, convert::TryInto};

/// Returns search hits that match the query defined in the request.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html>
#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct Search {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    runtime_mappings: BTreeMap<String, RuntimeMapping>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    indices_boost: Vec<KeyValuePair<String, Boost>>,

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
    pub fn indices_boost<F, B>(mut self, field: F, boost: B) -> Self
    where
        F: ToString,
        B: TryInto<Boost>,
    {
        if let Ok(boost) = boost.try_into() {
            self.indices_boost
                .push(KeyValuePair::new(field.to_string(), boost));
        }
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
    pub fn from<S>(mut self, from: S) -> Self
    where
        S: TryInto<u64>,
    {
        if let Ok(from) = from.try_into() {
            self.from = Some(from);
        }
        self
    }

    /// The number of hits to return.
    ///
    /// Defaults to `10`.
    pub fn size<S>(mut self, size: S) -> Self
    where
        S: TryInto<u64>,
    {
        if let Ok(size) = size.try_into() {
            self.size = Some(size);
        }
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

    /// A collection of sorting fields
    pub fn sort<S>(mut self, sort: S) -> Self
    where
        S: Into<Vec<Sort>>,
    {
        self.sort.extend(sort.into());
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
    pub fn rescore<R>(mut self, rescore: R) -> Self
    where
        R: Into<Rescore>,
    {
        let rescore = rescore.into();

        if !rescore.should_skip() {
            self.rescore.push(rescore);
        }

        self
    }

    add_aggregate!();
}
