use crate::search::*;
use crate::util::*;
use std::convert::TryInto;

#[derive(Debug, Clone, Serialize, PartialEq)]
/// A multi-bucket value source based aggregation where buckets are dynamically built - one per unique value.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-terms-aggregation.html>
pub struct TermsAggregation {
    #[serde(skip_serializing)]
    pub(crate) name: String,
    terms: TermsAggregationInner,
    /// Sub-aggregations
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub(crate) aggs: Aggregations,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct TermsAggregationInner {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    size: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    show_term_doc_count_error: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    order: Vec<TermsAggregationOrder>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    min_doc_count: Option<u16>,
}

/// Terms Aggregation sorting struct
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TermsAggregationOrder(KeyValuePair<SortField, SortOrder>);

impl TermsAggregationOrder {
    /// Creates an instance of [TermsAggregationOrder](TermsAggregationOrder)
    ///
    /// - `field` - Field to sort by
    /// - `order` - Ordering direction
    pub fn new(field: impl Into<SortField>, order: SortOrder) -> Self {
        Self(KeyValuePair::new(field.into(), order))
    }
}

impl<K> From<(K, SortOrder)> for TermsAggregationOrder
where
    K: Into<SortField>,
{
    fn from((key, value): (K, SortOrder)) -> Self {
        Self::new(key, value)
    }
}

impl Aggregation {
    /// Creates an instance of [`TermsAggregation`]
    ///
    /// - `name` - name of the aggregation
    /// - `field` - field to group by
    pub fn terms(name: impl Into<String>, field: impl Into<String>) -> TermsAggregation {
        TermsAggregation {
            name: name.into(),
            terms: TermsAggregationInner {
                field: field.into(),
                size: None,
                show_term_doc_count_error: None,
                order: vec![],
                min_doc_count: None,
            },
            aggs: Aggregations::new(),
        }
    }
}

impl TermsAggregation {
    /// The `size` parameter can be set to define how many term buckets should be returned out of the overall terms list.
    ///
    /// By default, the node coordinating the search process will request each shard to provide its own top `size` term buckets
    /// and once all shards respond, it will reduce the results to the final list that will then be returned to the client.
    ///
    /// This means that if the number of unique terms is greater than `size`, the returned list is slightly off and not accurate
    /// (it could be that the term counts are slightly off and it could even be that a term that should have been in the top `size` buckets was not returned).
    pub fn size(mut self, size: impl TryInto<u64>) -> Self {
        if let Ok(size) = size.try_into() {
            self.terms.size = Some(size);
        }
        self
    }

    /// Shows an error value for each term returned by the aggregation which represents the worst case error in the document
    /// count and can be useful when deciding on a value for the shard_size parameter.
    /// This is calculated by summing the document counts for the last term returned by all shards which did not return the term.
    pub fn show_term_doc_count_error(mut self, show_term_doc_count_error: bool) -> Self {
        self.terms.show_term_doc_count_error = Some(show_term_doc_count_error);
        self
    }

    /// The order of the buckets can be customized by setting the order parameter.
    /// By default, the buckets are ordered by their doc_count descending.
    /// Order field allows changing this behavior.
    ///
    /// > Sorting by ascending `_count` or by sub aggregation is discouraged as it increases the
    /// [error](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-terms-aggregation.html#search-aggregations-bucket-terms-aggregation-approximate-counts)
    /// on document counts. It is fine when a single shard is queried, or when the field that is
    /// being aggregated was used as a routing key at index time: in these cases results will be
    /// accurate since shards have disjoint values. However otherwise, errors are unbounded.
    /// One particular case that could still be useful is sorting by
    /// [min](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-min-aggregation.html) or
    /// [max](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-max-aggregation.html)
    /// aggregation: counts will not be accurate but at least the top buckets will be correctly picked.
    pub fn order(mut self, order: impl Into<TermsAggregationOrder>) -> Self {
        self.terms.order.push(order.into());
        self
    }

    /// Only returns terms that match more than a configured number of hits using the `min_doc_count`
    ///
    /// Default value is `1`
    pub fn min_doc_count(mut self, min_doc_count: impl Into<u16>) -> Self {
        self.terms.min_doc_count = Some(min_doc_count.into());
        self
    }

    add_aggregate!();
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_field_only(
            Aggregation::terms("test_agg", "test_field"),
            json!({ "terms": { "field": "test_field" } })
        );

        with_all_fields(
            Aggregation::terms("test_agg", "test_field")
                .size(5u16)
                .min_doc_count(2u16)
                .show_term_doc_count_error(false)
                .order(TermsAggregationOrder::new("test_order", SortOrder::Asc)),
            json!({
                "terms": {
                    "field": "test_field",
                    "size": 5,
                    "min_doc_count": 2,
                    "show_term_doc_count_error": false,
                    "order": [
                        { "test_order": "asc" }
                    ]
                }
            })
        );

        with_sub_aggregations(
            Aggregation::terms("test_agg", "test_field")
                .size(0u16)
                .order(("test_order", SortOrder::Asc))
                .aggregate(
                    Aggregation::terms("test_sub_agg", "test_field2").size(3u16)
                ),
            json!({
                "terms": {
                    "field": "test_field",
                    "size": 0,
                    "order": [
                        { "test_order": "asc" }
                    ]
                },
                "aggs": {
                    "test_sub_agg": {
                        "terms": {
                            "field": "test_field2",
                            "size": 3
                        }
                    }
                }
            })
        );
    }
}
