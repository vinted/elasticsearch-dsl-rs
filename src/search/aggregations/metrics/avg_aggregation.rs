use crate::search::*;
use crate::util::*;

/// A `single-value` metrics aggregation that computes the average of numeric values that are extracted
/// from the aggregated documents. These values can be extracted either from specific numeric fields
/// in the documents.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-avg-aggregation.html>
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct AvgAggregation {
    avg: AvgAggregationInner,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct AvgAggregationInner {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing: Option<Number>,
}

impl Aggregation {
    /// Creates an instance of [`AvgAggregation`]
    ///
    /// - `field` - field to aggregate
    pub fn avg<T>(field: T) -> AvgAggregation
    where
        T: ToString,
    {
        AvgAggregation {
            avg: AvgAggregationInner {
                field: field.to_string(),
                missing: None,
            },
        }
    }
}

impl AvgAggregation {
    /// The missing parameter defines how documents that are missing a value should be treated. By
    /// default they will be ignored but it is also possible to treat them as if they had a value.
    pub fn missing<T>(mut self, missing: T) -> Self
    where
        T: Into<Number>,
    {
        self.avg.missing = Some(missing.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_aggregation(
            Aggregation::avg("test_field"),
            json!({ "avg": { "field": "test_field" } }),
        );

        assert_serialize_aggregation(
            Aggregation::avg("test_field").missing(100.1),
            json!({
                "avg": {
                    "field": "test_field",
                    "missing": 100.1
                }
            }),
        );
    }
}
