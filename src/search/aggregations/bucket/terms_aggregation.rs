use crate::search::*;
use crate::util::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
/// A multi-bucket value source based aggregation where buckets are dynamically built - one per unique value.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-terms-aggregation.html>
pub struct TermsAggregation {
    terms: TermsAggregationInner,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    aggs: Aggregations,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct TermsAggregationInner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    field: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    size: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    show_term_doc_count_error: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    order: TermsOrderCollection,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    min_doc_count: Option<u16>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing: Option<Term>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    include: Option<TermsInclude>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    exclude: Option<TermsExclude>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    script: Option<Script>,
}

impl Aggregation {
    /// Creates an instance of [`TermsAggregation`]
    ///
    /// - `field` - field to group by
    pub fn terms<T>(field: T) -> TermsAggregation
    where
        T: ToString,
    {
        TermsAggregation {
            terms: TermsAggregationInner {
                field: Some(field.to_string()),
                size: None,
                show_term_doc_count_error: None,
                order: Default::default(),
                min_doc_count: None,
                missing: None,
                include: None,
                exclude: None,
                script: None,
            },
            aggs: Aggregations::new(),
        }
    }

    /// Creates an instance of [`TermsAggregation`] with a script
    pub fn terms_with_script(script: Script) -> TermsAggregation {
        TermsAggregation {
            terms: TermsAggregationInner {
                field: None,
                size: None,
                show_term_doc_count_error: None,
                order: Default::default(),
                min_doc_count: None,
                missing: None,
                include: None,
                exclude: None,
                script: Some(script),
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
    pub fn size(mut self, size: u64) -> Self {
        self.terms.size = Some(size);
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
    pub fn order<T>(mut self, order: T) -> Self
    where
        T: Into<TermsOrderCollection>,
    {
        self.terms.order = order.into();
        self
    }

    /// Only returns terms that match more than a configured number of hits using the `min_doc_count`
    ///
    /// Default value is `1`
    pub fn min_doc_count(mut self, min_doc_count: u16) -> Self {
        self.terms.min_doc_count = Some(min_doc_count);
        self
    }

    /// The missing parameter defines how documents that are missing a value should be treated.
    /// By default they will be ignored but it is also possible to treat them as if they had a value.
    pub fn missing<T>(mut self, missing: T) -> Self
    where
        T: Serialize,
    {
        self.terms.missing = Term::new(missing);
        self
    }

    /// The `include` parameter can be set to include only specific terms in the response.
    pub fn include<T>(mut self, include: T) -> Self
    where
        T: Into<TermsInclude>,
    {
        self.terms.include = Some(include.into());
        self
    }

    /// The `exclude` parameter can be set to exclude specific terms from the response.
    pub fn exclude<T>(mut self, exclude: T) -> Self
    where
        T: Into<TermsExclude>,
    {
        self.terms.exclude = Some(exclude.into());
        self
    }

    /// Sets the script for the aggregation.
    pub fn script(mut self, script: Script) -> Self {
        self.terms.script = Some(script);
        self
    }

    /// The field can be Keyword, Numeric, ip, boolean, or binary.
    pub fn field<T>(mut self, field: T) -> Self
    where
        T: Into<String>
    {
        self.terms.field = Some(field.into());
        self
    }

    add_aggregate!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_aggregation(
            Aggregation::terms("test_field"),
            json!({ "terms": { "field": "test_field" } }),
        );

        assert_serialize_aggregation(
            Aggregation::terms("test_field")
                .size(5)
                .min_doc_count(2)
                .show_term_doc_count_error(false)
                .missing("N/A")
                .order(TermsOrder::new("test_order", SortOrder::Asc)),
            json!({
                "terms": {
                    "field": "test_field",
                    "size": 5,
                    "min_doc_count": 2,
                    "show_term_doc_count_error": false,
                    "missing": "N/A",
                    "order": [
                        { "test_order": "asc" }
                    ]
                }
            }),
        );

        assert_serialize_aggregation(
            Aggregation::terms("test_field")
                .size(0)
                .order(TermsOrder::ascending("test_order"))
                .missing(123)
                .include(["mazda", "honda"])
                .exclude("water_.*")
                .aggregate(
                    "test_sub_agg",
                    Aggregation::terms("test_field2")
                        .size(3)
                        .missing(false)
                        .include([0, 20]),
                ),
            json!({
                "terms": {
                    "field": "test_field",
                    "size": 0,
                    "missing": 123,
                    "include": ["mazda", "honda"],
                    "exclude": "water_.*",
                    "order": [
                        { "test_order": "asc" }
                    ]
                },
                "aggs": {
                    "test_sub_agg": {
                        "terms": {
                            "field": "test_field2",
                            "size": 3,
                            "missing": false,
                            "include": {
                                "partition": 0,
                                "num_partitions": 20
                            }
                        }
                    }
                }
            }),
        );

        assert_serialize_aggregation(
            Aggregation::terms_with_script(
                Script::source("if (!doc['field1'].isEmpty()) { return 'f2'; } if (!doc['field2'].isEmpty()) { return 'f1'; } return 'unknown';")
                    .lang("painless")
            ).size(10),
            json!({
                "terms": {
                    "script": {
                        "source": "if (!doc['field1'].isEmpty()) { return 'f2'; } if (!doc['field2'].isEmpty()) { return 'f1'; } return 'unknown';",
                        "lang": "painless"
                    },
                    "size": 10
                }
            }),
        );
    }
}
