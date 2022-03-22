use crate::search::*;
use crate::util::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
/// A single bucket aggregation that narrows the set of documents to those that match a query.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-filter-aggregation.html>
pub struct FilterAggregation {
    filter: Query,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    aggs: Aggregations,
}

impl Aggregation {
    /// Creates an instance of [`FilterAggregation`]
    ///
    /// - `query` - query to filter by
    pub fn filter<Q>(query: Q) -> FilterAggregation
    where
        Q: Into<Query>,
    {
        FilterAggregation {
            filter: query.into(),
            aggs: Aggregations::new(),
        }
    }
}

impl FilterAggregation {
    add_aggregate!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Aggregation::filter(Query::term("type", "t-shirt"))
                .aggregate("sizes", Aggregation::terms("size")),
            json!({
                "filter": { "term": { "type": { "value": "t-shirt"} } },
                "aggs": {
                    "sizes": { "terms": { "field": "size" } }
                }
            }),
        );
    }
}
