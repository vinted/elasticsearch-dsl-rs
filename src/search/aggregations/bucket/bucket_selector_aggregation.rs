use std::collections::HashMap;
use serde::Serialize;

use crate::search::*;
use crate::util::*;

/// Specifies the path to the buckets to filter in a bucket selector aggregation.
///
/// This can either be a single path, referencing a single metric, or multiple paths
/// in case of more complex aggregations.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum BucketsPath {
    /// A single path referencing a metric.
    Single(String),
    /// Multiple paths in the form of key-value pairs.
    /// Each key corresponds to an alias, and each value is a path to the metric.
    Multi(HashMap<String, String>),
}

impl From<&str> for BucketsPath {
    fn from(path: &str) -> Self {
        BucketsPath::Single(path.to_string())
    }
}

impl From<String> for BucketsPath {
    fn from(path: String) -> Self {
        BucketsPath::Single(path)
    }
}

impl From<Vec<(&str, &str)>> for BucketsPath {
    fn from(paths: Vec<(&str, &str)>) -> Self {
        BucketsPath::Multi(paths.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect())
    }
}

impl From<Vec<(String, String)>> for BucketsPath {
    fn from(paths: Vec<(String, String)>) -> Self {
        BucketsPath::Multi(paths.into_iter().collect())
    }
}


#[derive(Debug, Clone, Serialize, PartialEq)]
/// A parent pipeline aggregation which allows the user to specify a script to run
/// for each bucket on the set of values returned by another aggregation.
pub struct BucketSelectorAggregation {
    bucket_selector: BucketSelectorAggregationInner,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    aggs: Aggregations,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct BucketSelectorAggregationInner {
    buckets_path: BucketsPath,

    script: Script,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    gap_policy: Option<GapPolicy>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    format: Option<String>,
}

impl Aggregation {
    /// Creates an instance of [`BucketSelectorAggregation`]
    ///
    /// - `buckets_path` - the path to the buckets to filter
    /// - `script` - the script to execute for filtering
    pub fn bucket_selector<B, S>(buckets_path: B, script: S) -> BucketSelectorAggregation
    where
        B: Into<BucketsPath>,
        S: Into<Script>,
    {
        BucketSelectorAggregation {
            bucket_selector: BucketSelectorAggregationInner {
                buckets_path: buckets_path.into(),
                script: script.into(),
                gap_policy: None,
                format: None,
            },
            aggs: Aggregations::new(),
        }
    }
}

impl BucketSelectorAggregation {
    /// Sets the gap policy for the bucket selector aggregation.
    ///
    /// The gap policy determines how documents with missing values are treated.
    /// The default policy is to skip gaps.
    pub fn gap_policy(mut self, gap_policy: GapPolicy) -> Self {
        self.bucket_selector.gap_policy = Some(gap_policy);
        self
    }

    /// Sets the format for the bucket selector aggregation.
    ///
    /// The format parameter can be used to specify the format of the output values.
    pub fn format<T>(mut self, format: T) -> Self
    where
        T: ToString,
    {
        self.bucket_selector.format = Some(format.to_string());
        self
    }

    add_aggregate!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_aggregation(
            Aggregation::bucket_selector("the_sum", Script::source("params.the_sum > 1000").lang("painless")),
            json!({
                "bucket_selector": {
                    "buckets_path": "the_sum",
                    "script": {
                        "lang": "painless",
                         "source": "params.the_sum > 1000"
                    }
                }
            }),
        );

        assert_serialize_aggregation(
            Aggregation::bucket_selector("the_sum", Script::source("params.the_sum > 1000").lang("painless"))
                .gap_policy(GapPolicy::Skip)
                .format("###.00"),
            json!({
                "bucket_selector": {
                    "buckets_path": "the_sum",
                   "script": {
                        "lang": "painless",
                        "source": "params.the_sum > 1000"
                    },
                    "gap_policy": "skip",
                    "format": "###.00"
                }
            }),
        );

        assert_serialize_aggregation(
            Aggregation::bucket_selector(vec![("sum_value", "the_sum")], Script::source("params.sum_value > 1000").lang("painless")),
            json!({
                "bucket_selector": {
                    "buckets_path": {
                        "sum_value": "the_sum"
                    },
                    "script": {
                        "lang": "painless",
                        "source":"params.sum_value > 1000"
                    }
                }
            }),
        );
    }
}
