//! Bucket aggregations don’t calculate metrics over fields like the metrics aggregations do,
//! but instead, they create buckets of documents. Each bucket is associated with a criterion
//! (depending on the aggregation type) which determines whether or not a document in the current
//! context "falls" into it. In other words, the buckets effectively define document sets.
//! In addition to the buckets themselves, the `bucket` aggregations also compute
//! and return the number of documents that "fell into" each bucket.
//!
//! Bucket aggregations, as opposed to `metrics` aggregations, can hold sub-aggregations.
//! These sub-aggregations will be aggregated for the buckets created by their "parent" bucket aggregation.
//!
//! There are different bucket aggregators, each with a different "bucketing" strategy.
//! Some define a single bucket, some define fixed number of multiple buckets,
//! and others dynamically create the buckets during the aggregation process.
//!
//! > The maximum number of buckets allowed in a single response is limited by
//! > a dynamic cluster setting named
//! > [`search.max_buckets`](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-settings.html#search-settings-max-buckets).
//! > It defaults to `65,535`. Requests that try to return more than the limit will fail with an exception.
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket.html>

mod bucket_selector_aggregation;
mod children_aggregation;
mod composite_aggregation;
mod date_histogram_aggregation;
mod diversified_sampler_aggregation;
mod filter_aggregation;
mod geotile_grid_aggregation;
mod nested_aggregation;
mod sampler_aggregation;
mod terms_aggregation;

pub use self::bucket_selector_aggregation::*;
pub use self::children_aggregation::*;
pub use self::composite_aggregation::*;
pub use self::date_histogram_aggregation::*;
pub use self::diversified_sampler_aggregation::*;
pub use self::filter_aggregation::*;
pub use self::geotile_grid_aggregation::*;
pub use self::nested_aggregation::*;
pub use self::sampler_aggregation::*;
pub use self::terms_aggregation::*;
