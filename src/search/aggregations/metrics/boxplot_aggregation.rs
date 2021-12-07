use crate::util::*;
use crate::{Aggregation, Number};

/// A `boxplot` metrics aggregation that computes boxplot of numeric values extracted from the
/// aggregated documents. These values can be generated from specific numeric or [histogram fields](https://www.elastic.co/guide/en/elasticsearch/reference/current/histogram.html)
/// in the documents.
///
/// The `boxplot` aggregation returns essential information for making a [box plot](https://en.wikipedia.org/wiki/Box_plot):
/// minimum, maximum median, first quartile (25th percentile) and third quartile (75th percentile) values.
///
/// The algorithm used by the `boxplot` metric is called TDigest (introduced by Ted Dunning in
/// [Computing Accurate Quantiles using T-Digests](https://github.com/tdunning/t-digest/blob/master/docs/t-digest-paper/histo.pdf)).
///
/// > Boxplot as other percentile aggregations are also [non-deterministic](https://en.wikipedia.org/wiki/Nondeterministic_algorithm).
/// This means you can get slightly different results using the same data.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-boxplot-aggregation.html>
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct BoxplotAggregation {
    #[serde(skip_serializing)]
    pub(crate) name: String,
    boxplot: BoxplotAggregationInner,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct BoxplotAggregationInner {
    field: String,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    compression: Option<Number>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing: Option<Number>,
}

impl Aggregation {
    /// Creates an instance of [`BoxplotAggregation`]
    ///
    /// - `name` - name of the aggregation
    /// - `field` - field to aggregate
    pub fn boxplot(name: impl Into<String>, field: impl Into<String>) -> BoxplotAggregation {
        BoxplotAggregation {
            name: name.into(),
            boxplot: BoxplotAggregationInner {
                field: field.into(),
                compression: None,
                missing: None,
            },
        }
    }
}

impl BoxplotAggregation {
    /// Approximate algorithms must balance memory utilization with estimation accuracy.
    ///
    /// The TDigest algorithm uses a number of "nodes" to approximate percentiles —— the more
    /// nodes available, the higher the accuracy (and large memory footprint) proportional to the
    /// volume of data. The `compression` parameter limits the maximum number of nodes to 20 * `compression`.
    ///
    /// Therefore, by increasing the compression value, you can increase the accuracy of your
    /// percentiles at the cost of more memory. Larger compression values also make the algorithm
    /// slower since the underlying tree data structure grows in size, resulting in more expensive
    /// operations. The default compression value is 100.
    ///
    /// A "node" uses roughly 32 bytes of memory, so under worst-case scenarios (large amount of
    /// data which arrives sorted and in-order) the default settings will produce a TDigest roughly
    /// 64KB in size. In practice data tends to be more random and the TDigest will use less memory.
    pub fn compression(mut self, compression: impl Into<Number>) -> Self {
        self.boxplot.compression = Some(compression.into());
        self
    }

    /// The `missing` parameter defines how documents that are missing a value should be treated.
    /// By default they will be ignored but it is also possible to treat them as if they had a value.
    pub fn missing(mut self, missing: impl Into<Number>) -> Self {
        self.boxplot.missing = Some(missing.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            Aggregation::boxplot("test_boxplot", "test_field"),
            json!({ "boxplot": { "field": "test_field" } })
        );

        with_all_fields(
            Aggregation::boxplot("test_boxplot", "test_field")
                .compression(100)
                .missing(10),
            json!({
                "boxplot": {
                    "field": "test_field",
                    "compression": 100,
                    "missing": 10
                }
            })
        );
    }
}
