use crate::search::*;
use crate::util::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
/// A single bucket aggregation that selects child documents that have the specified type,
/// then executes a sub-aggregation for the children documents.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-children-aggregation.html>
pub struct ChildrenAggregation {
    children: ChildrenAggregationInner,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    aggs: Aggregations,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct ChildrenAggregationInner {
    #[serde(rename = "type")]
    type_: String,
}

impl Aggregation {
    /// Creates an instance of [`ChildrenAggregation`]
    ///
    /// - `type_` - type of the child documents
    pub fn children<T>(type_: T) -> ChildrenAggregation
    where
        T: ToString,
    {
        ChildrenAggregation {
            children: ChildrenAggregationInner {
                type_: type_.to_string(),
            },
            aggs: Aggregations::new(),
        }
    }
}

impl ChildrenAggregation {
    add_aggregate!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_aggregation(
            Aggregation::children("answer"),
            json!({ "children": { "type": "answer" } }),
        );

        assert_serialize_aggregation(
            Aggregation::children("answer").aggregate(
                "avg_score",
                Aggregation::avg("score"),
            ),
            json!({
                "children": {
                    "type": "answer"
                },
                "aggs": {
                    "avg_score": {
                        "avg": {
                            "field": "score"
                        }
                    }
                }
            }),
        );
    }
}
