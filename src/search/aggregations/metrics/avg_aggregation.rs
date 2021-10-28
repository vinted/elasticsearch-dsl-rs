use crate::util::*;
use crate::{Aggregation, Scalar};

/// A single-value metrics aggregation that computes the average of numeric values that are extracted
/// from the aggregated documents. These values can be extracted either from specific numeric fields
/// in the documents.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-avg-aggregation.html>
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct AvgAggregation {
    #[serde(skip_serializing)]
    pub(crate) name: String,
    avg: AvgAggregationInner,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct AvgAggregationInner {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing: Option<Scalar>,
}

impl Aggregation {
    /// Creates an instance of [`AvgAggregation`]
    ///
    /// - `name` - name of the aggregation
    pub fn avg(name: impl Into<String>, field: impl Into<String>) -> AvgAggregation {
        AvgAggregation {
            name: name.into(),
            avg: AvgAggregationInner {
                field: field.into(),
                missing: None,
            },
        }
    }
}

impl AvgAggregation {
    /// The missing parameter defines how documents that are missing a value should be treated. By
    /// default they will be ignored but it is also possible to treat them as if they had a value.
    pub fn missing(mut self, missing: impl Into<Scalar>) -> Self {
        let missing = match missing.into() {
            Scalar::Bool(_) | Scalar::String(_) | Scalar::DateTime(_) => None,
            missing => Some(missing),
        };
        self.avg.missing = missing;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            Aggregation::avg("test_avg", "test_field"),
            json!({ "avg": { "field": "test_field" } })
        );

        with_all_fields(
            Aggregation::avg("test_avg", "test_field")
                .missing(100.1),
            json!({
                "avg": {
                    "field": "test_field",
                    "missing": 100.1
                }
            })
        );
    }
}
