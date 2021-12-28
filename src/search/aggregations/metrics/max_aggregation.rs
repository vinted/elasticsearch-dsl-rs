use crate::search::*;
use crate::util::*;

/// A `single-value` metrics aggregation that keeps track and returns the maximum value among the
/// numeric values extracted from the aggregated documents.
///
/// > The `min` and `max` aggregation operate on the `double` representation of the data. As a
/// consequence, the result may be approximate when running on longs whose absolute value is greater
/// than `2^53`.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-max-aggregation.html>
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MaxAggregation {
    #[serde(skip_serializing)]
    pub(crate) name: String,
    max: MaxAggregationInner,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct MaxAggregationInner {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing: Option<Number>,
}

impl Aggregation {
    /// Creates an instance of [`MaxAggregation`]
    ///
    /// - `name` - name of the aggregation
    /// - `field` - field to aggregate
    pub fn max(name: impl Into<String>, field: impl Into<String>) -> MaxAggregation {
        MaxAggregation {
            name: name.into(),
            max: MaxAggregationInner {
                field: field.into(),
                missing: None,
            },
        }
    }
}

impl MaxAggregation {
    /// The `missing` parameter defines how documents that are missing a value should be treated. By
    /// default they will be ignored but it is also possible to treat them as if they had a value.
    pub fn missing(mut self, missing: impl Into<Number>) -> Self {
        self.max.missing = Some(missing.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Aggregation::max("test_max", "test_field"),
            json!({ "max": { "field": "test_field" } }),
        );

        assert_serialize(
            Aggregation::max("test_max", "test_field").missing(100.1),
            json!({
                "max": {
                    "field": "test_field",
                    "missing": 100.1
                }
            }),
        );
    }
}
