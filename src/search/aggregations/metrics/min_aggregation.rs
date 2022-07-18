use crate::search::*;
use crate::util::*;

/// A `single-value` metrics aggregation that keeps track and returns the minimum value among numeric
/// values extracted from the aggregated documents.
///
/// > The `min` and `max` aggregation operate on the `double` representation of the data. As a
/// consequence, the result may be approximate when running on longs whose absolute value is greater
/// than `2^53`.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-min-aggregation.html>
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MinAggregation {
    min: MinAggregationInner,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct MinAggregationInner {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing: Option<Number>,
}

impl Aggregation {
    /// Creates an instance of [`MinAggregation`]
    ///
    /// - `field` - field to aggregate
    pub fn min<T>(field: T) -> MinAggregation
    where
        T: ToString,
    {
        MinAggregation {
            min: MinAggregationInner {
                field: field.to_string(),
                missing: None,
            },
        }
    }
}

impl MinAggregation {
    /// The `missing` parameter defines how documents that are missing a value should be treated. By
    /// default they will be ignored but it is also possible to treat them as if they had a value.
    pub fn missing<T>(mut self, missing: T) -> Self
    where
        T: Into<Number>,
    {
        self.min.missing = Some(missing.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_aggregation(
            Aggregation::min("test_field"),
            json!({ "min": { "field": "test_field" } }),
        );

        assert_serialize_aggregation(
            Aggregation::min("test_field").missing(100.1),
            json!({
                "min": {
                    "field": "test_field",
                    "missing": 100.1
                }
            }),
        );
    }
}
