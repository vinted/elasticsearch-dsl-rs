use crate::util::*;
use crate::Aggregation;

/// A `single-value` metrics aggregation that calculates an approximate count of distinct values.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-cardinality-aggregation.html>
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CardinalityAggregation {
    #[serde(skip_serializing)]
    pub(crate) name: String,
    cardinality: CardinalityAggregationInner,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct CardinalityAggregationInner {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    precision_threshold: Option<u16>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing: Option<String>,
}

impl Aggregation {
    /// Creates an instance of [`CardinalityAggregation`]
    ///
    /// - `name` - name of the aggregation
    pub fn cardinality(
        name: impl Into<String>,
        field: impl Into<String>,
    ) -> CardinalityAggregation {
        CardinalityAggregation {
            name: name.into(),
            cardinality: CardinalityAggregationInner {
                field: field.into(),
                precision_threshold: None,
                missing: None,
            },
        }
    }
}

impl CardinalityAggregation {
    /// The `precision_threshold` options allows to trade memory for accuracy, and defines a unique count below
    /// which counts are expected to be close to accurate. Above this value, counts might become a bit more fuzzy.
    /// The maximum supported value is 40000, thresholds above this number will have the same effect as a threshold
    /// of 40000. The default value is 3000
    pub fn precision_threshold(mut self, precision_threshold: impl Into<u16>) -> Self {
        self.cardinality.precision_threshold = Some(precision_threshold.into());
        self
    }

    /// The `missing` parameter defines how documents that are missing a value should be treated. By default they will
    /// be ignored but it is also possible to treat them as if they had a value.
    pub fn missing(mut self, missing: impl Into<String>) -> Self {
        self.cardinality.missing = Some(missing.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            Aggregation::cardinality("test_cardinality", "test_field"),
            json!({ "cardinality": { "field": "test_field" } })
        );

        with_all_fields(
            Aggregation::cardinality("test_cardinality", "test_field")
                .precision_threshold(100u16)
                .missing("N/A"),
            json!({
                "cardinality": {
                    "field": "test_field",
                    "precision_threshold": 100,
                    "missing": "N/A"
                }
            })
        );
    }
}
