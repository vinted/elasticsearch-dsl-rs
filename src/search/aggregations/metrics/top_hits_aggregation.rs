use crate::search::*;
use crate::util::*;
use std::convert::TryInto;

/// A `top_hits` metric aggregation keeps track of the most relevant document being aggregated.
/// This aggregation is intended to be used as a sub aggregation,
/// so that the top matching documents can be aggregated per bucket.
///
/// > We do not recommend using `top_hits` as a top-level aggregation.
/// If you want to group search hits, use the
/// [`collapse`](https://www.elastic.co/guide/en/elasticsearch/reference/current/collapse-search-results.html)
/// parameter instead.
///
/// The `top_hits` aggregation can effectively be used to group result sets
/// by certain fields via a bucket aggregation. One or more bucket aggregations
/// determines by which properties a result set get sliced into.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-top-hits-aggregation.html>
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TopHitsAggregation {
    top_hits: TopHitsAggregationInner,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct TopHitsAggregationInner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _source: Option<SourceFilter>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    from: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    size: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    sort: Vec<Sort>,
}

impl Aggregation {
    /// Creates an instance of [`TopHitsAggregation`]
    pub fn top_hits() -> TopHitsAggregation {
        TopHitsAggregation {
            top_hits: TopHitsAggregationInner {
                _source: None,
                from: None,
                size: None,
                sort: vec![],
            },
        }
    }
}

impl TopHitsAggregation {
    /// Indicates which source fields are returned for matching documents
    pub fn source(mut self, source: impl Into<SourceFilter>) -> Self {
        self.top_hits._source = Some(source.into());
        self
    }

    /// The offset from the first result you want to fetch.
    pub fn from(mut self, from: impl TryInto<u64>) -> Self {
        if let Ok(from) = from.try_into() {
            self.top_hits.from = Some(from);
        }
        self
    }

    /// The maximum number of top matching hits to return per bucket.
    ///
    /// By default the top three matching hits are returned.
    pub fn size(mut self, size: impl TryInto<u64>) -> Self {
        if let Ok(size) = size.try_into() {
            self.top_hits.size = Some(size);
        }
        self
    }

    /// A collection of sorting fields
    pub fn sort(mut self, sort: impl Into<Vec<Sort>>) -> Self {
        self.top_hits.sort.extend(sort.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_aggregation(Aggregation::top_hits(), json!({ "top_hits": { } }));

        assert_serialize_aggregation(
            Aggregation::top_hits()
                .source(false)
                .from(2u8)
                .size(10u8)
                .sort(Sort::new("sort_field").order(SortOrder::Desc)),
            json!({
                "top_hits": {
                    "_source": false,
                    "from": 2,
                    "size": 10,
                    "sort": [
                        { "sort_field": { "order": "desc" } }
                    ]
                }
            }),
        );
    }
}
