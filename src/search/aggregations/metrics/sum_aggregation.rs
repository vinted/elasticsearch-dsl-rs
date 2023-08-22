use crate::search::*;
use crate::util::*;

/// A `single-value` metrics aggregation that sums up numeric values that are extracted from the
/// aggregated documents. These values can be extracted either from specific numeric or histogram fields.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-sum-aggregation.html>
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SumAggregation {
    sum: SumAggregationInner,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct SumAggregationInner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    field: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    script: Option<Script>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing: Option<Number>,
}

impl Aggregation {
    /// Creates an instance of [`SumAggregation`]
    ///
    /// - `field` - field to aggregate
    pub fn sum<T>(field: T) -> SumAggregation
    where
        T: ToString,
    {
        SumAggregation {
            sum: SumAggregationInner {
                field: field.to_string().into(),
                script: None,
                missing: None,
            },
        }
    }

    /// Creates an instance of [`SumAggregation`]
    ///
    /// - `script` - script to aggregate
    pub fn sum_script(script: Script) -> SumAggregation {
        SumAggregation {
            sum: SumAggregationInner {
                script: script.into(),
                field: None,
                missing: None,
            },
        }
    }
}

impl SumAggregation {
    /// The `missing` parameter defines how documents that are missing a value should be treated. By
    /// default documents missing the value will be ignored but it is also possible to treat them
    /// as if they had a value.
    pub fn missing<T>(mut self, missing: T) -> Self
    where
        T: Into<Number>,
    {
        self.sum.missing = Some(missing.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_aggregation(
            Aggregation::sum("test_field"),
            json!({ "sum": { "field": "test_field" } }),
        );

        assert_serialize_aggregation(
            Aggregation::sum("test_field").missing(100.1),
            json!({
                "sum": {
                    "field": "test_field",
                    "missing": 100.1
                }
            }),
        );

        assert_serialize_aggregation(
            Aggregation::sum_script(Script::source("_score")),
            json!({
                "sum": {
                    "script": {
                        "source": "_score"
                    }
                }
            }),
        );
    }
}
