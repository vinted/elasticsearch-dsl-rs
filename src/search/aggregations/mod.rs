//! An aggregation summarizes your data as metrics, statistics, or other analytics.
//!
//! Aggregations help you answer questions like:
//!
//! 1. What’s the average load time for my website?
//! 2. Who are my most valuable customers based on transaction volume?
//! 3. What would be considered a large file on my network?
//! 4. How many products are in each product category?
//!
//! Elasticsearch organizes aggregations into three categories:
//!
//! - [Metrics](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html) aggregations that calculate metrics, such as a sum or average, from field values.
//! - [Bucket](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html) aggregations that group documents into buckets, also called bins, based on field values, ranges, or other criteria.
//! - [Pipeline](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html) aggregations that take input from other aggregations instead of documents or fields.

pub mod bucket;
pub mod metrics;
pub mod params;
pub mod pipeline;

use crate::Map;

pub use self::bucket::*;
pub use self::metrics::*;
pub use self::params::*;
pub use self::pipeline::*;

macro_rules! aggregation {
    ($($variant:ident($query:ty)),+ $(,)?) => {
        /// A container enum for supported Elasticsearch query types
        #[derive(Clone, PartialEq, Serialize)]
        #[serde(untagged)]
        #[allow(missing_docs, clippy::large_enum_variant)]
        pub enum Aggregation {
            $(
                $variant($query),
            )*
        }

        impl std::fmt::Debug for Aggregation {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variant(q) => q.fmt(f),
                    )+
                }
            }
        }

        $(
            impl From<$query> for Aggregation {
                fn from(q: $query) -> Self {
                    Aggregation::$variant(q)
                }
            }
        )+
    };
}

aggregation!(
    Terms(TermsAggregation),
    TopHits(TopHitsAggregation),
    Cardinality(CardinalityAggregation),
    Avg(AvgAggregation),
    Max(MaxAggregation),
    Min(MinAggregation),
    Sum(SumAggregation),
    Rate(RateAggregation),
    Sampler(SamplerAggregation),
    Filter(FilterAggregation),
    DiversifiedSampler(DiversifiedSamplerAggregation),
    Boxplot(BoxplotAggregation),
    DateHistogram(DateHistogramAggregation),
    GeotileGrid(GeotileGridAggregation),
    BucketSelector(BucketSelectorAggregation),
    Children(ChildrenAggregation),
    Composite(CompositeAggregation),
    Nested(NestedAggregation),
    Range(RangeAggregation),
);

/// Type alias for a collection of aggregations
pub type Aggregations = Map<AggregationName, Aggregation>;
