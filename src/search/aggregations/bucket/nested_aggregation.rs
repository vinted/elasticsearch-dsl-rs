use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// A special single-bucket aggregation that enables aggregating nested documents.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-nested-aggregation.html>
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct NestedAggregation {
    nested: NestedAggregationInner,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    aggs: Aggregations,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct NestedAggregationInner {
    path: String,
}

impl Aggregation {
    /// Creates an instance of [`NestedAggregation`]
    ///
    /// - `path` - The nested path to aggregate.
    pub fn nested(path: &str) -> NestedAggregation {
        NestedAggregation {
            nested: NestedAggregationInner {
                path: path.to_string(),
            },
            aggs: Aggregations::new(),
        }
    }
}

impl NestedAggregation {
    add_aggregate!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_aggregation(
            Aggregation::nested("nested_path"),
            json!({ "nested": { "path": "nested_path" } }),
        );

        assert_serialize_aggregation(
            Aggregation::nested("nested_path").aggregate(
                "sub_agg",
                Aggregation::terms("test_field"),
            ),
            json!({
                "nested": {
                    "path": "nested_path"
                },
                "aggs": {
                    "sub_agg": {
                        "terms": {
                            "field": "test_field"
                        }
                    }
                }
            }),
        );
    }
}
