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

macro_rules! aggregation {
    ($name:ident { $($variant:ident($query:ty)),+ $(,)? }) => {
        /// A container enum for supported Elasticsearch query types
        #[derive(Debug, Clone, PartialEq, Serialize)]
        #[serde(untagged)]
        #[allow(missing_docs)]
        pub enum $name {
            $(
                $variant($query),
            )*
        }

        $(
            impl From<$query> for $name {
                fn from(q: $query) -> Self {
                    $name::$variant(q)
                }
            }
        )+

        impl $name {
            /// Gets aggregation name
            pub fn name(&self) -> String {
                match self {
                    $(
                        Self::$variant(a) => a.name.clone(),
                    )+
                }
            }
        }
    };
}

aggregation!(Aggregation {
    Terms(TermsAggregation),
    TopHits(TopHitsAggregation),
    Cardinality(CardinalityAggregation)
});

/// Type alias for a collection of aggregations
pub type Aggregations = std::collections::BTreeMap<String, Aggregation>;
