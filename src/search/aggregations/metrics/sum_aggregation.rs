use crate::search::*;
use crate::util::*;

/// A `single-value` metrics aggregation that sums up numeric values that are extracted from the
/// aggregated documents. These values can be extracted either from specific numeric or histogram fields.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-sum-aggregation.html>
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SumAggregation {
    #[serde(skip_serializing)]
    pub(crate) name: String,
    sum: SumAggregationInner,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct SumAggregationInner {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing: Option<Number>,
}

impl Aggregation {
    /// Creates an instance of [`SumAggregation`]
    ///
    /// - `name` - name of the aggregation
    /// - `field` - field to aggregate
    pub fn sum(name: impl Into<String>, field: impl Into<String>) -> SumAggregation {
        SumAggregation {
            name: name.into(),
            sum: SumAggregationInner {
                field: field.into(),
                missing: None,
            },
        }
    }
}

impl SumAggregation {
    /// The `missing` parameter defines how documents that are missing a value should be treated. By
    /// default documents missing the value will be ignored but it is also possible to treat them
    /// as if they had a value.
    pub fn missing(mut self, missing: impl Into<Number>) -> Self {
        self.sum.missing = Some(missing.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Aggregation::sum("test_sum", "test_field"),
            json!({ "sum": { "field": "test_field" } }),
        );

        assert_serialize(
            Aggregation::sum("test_sum", "test_field").missing(100.1),
            json!({
                "sum": {
                    "field": "test_field",
                    "missing": 100.1
                }
            }),
        );
    }
}
