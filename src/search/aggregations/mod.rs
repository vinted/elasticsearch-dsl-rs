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
pub mod pipeline;

pub use self::bucket::*;
pub use self::metrics::*;
pub use self::pipeline::*;
use crate::implement_aggregations;

/// Type alias for a collection of aggregations
pub type Aggregations = std::collections::BTreeMap<String, Aggregation>;

/// Container type for aggregations
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Aggregation {
    /// Group documents into buckets, also called bins, based on field values, ranges, or other criteria.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html>
    Bucket(BucketAggregation),

    /// Calculate metrics, such as a sum or average, from field values.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html>
    Metrics(MetricsAggregation),

    /// Take input from other aggregations instead of documents or fields.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html>
    Pipeline(PipelineAggregation),
}

impl Aggregation {
    /// Gets aggregation name, needed mainly for building blocks
    ///
    /// TODO: Fix cloning in another PR and switch to references
    pub fn name(&self) -> String {
        match self {
            Self::Bucket(a) => match a {
                BucketAggregation::Terms(a) => a.name.clone(),
            },
            Self::Metrics(a) => match a {
                MetricsAggregation::TopHits(a) => a.name.clone(),
            },
            Self::Pipeline(_) => panic!("No pipeline aggregations yet"),
        }
    }
}

implement_aggregations! {
    Bucket {
        Terms(TermsAggregation),
    }
}

implement_aggregations! {
    Metrics {
        TopHits(TopHitsAggregation),
    }
}

implement_aggregations! {
    Pipeline { }
}
