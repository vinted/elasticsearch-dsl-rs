use crate::search::*;
use crate::util::*;

/// Like the sampler aggregation this is a filtering aggregation used to limit any sub aggregations' processing
/// to a sample of the top-scoring documents. The diversified_sampler aggregation adds the ability to limit
/// the number of matches that share a common value such as an "author".
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-diversified-sampler-aggregation.html>
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DiversifiedSamplerAggregation {
    diversified_sampler: DiversifiedSamplerAggregationInner,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    aggs: Aggregations,
}

/// `execution_hint` field values.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionHint {
    /// Hold field values directly
    Map,

    /// Hold ordinals of the field as determined by the Lucene index
    BytesHash,

    /// Hold hashes of the field values - with potential for hash collisions
    GlobalOrdinals,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct DiversifiedSamplerAggregationInner {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    shard_size: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    max_docs_per_value: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    execution_hint: Option<ExecutionHint>,
}

impl Aggregation {
    /// Creates an instance of [`DiversifiedSamplerAggregation`]
    pub fn diversified_sampler<T>(field: T) -> DiversifiedSamplerAggregation
    where
        T: ToString,
    {
        DiversifiedSamplerAggregation {
            diversified_sampler: DiversifiedSamplerAggregationInner {
                field: field.to_string(),
                shard_size: None,
                max_docs_per_value: None,
                execution_hint: None,
            },
            aggs: Aggregations::new(),
        }
    }
}

impl DiversifiedSamplerAggregation {
    /// The `shard_size` parameter limits how many top-scoring documents are
    /// collected in the sample processed on each shard. The default value is 100.
    pub fn shard_size(mut self, shard_size: u64) -> Self {
        self.diversified_sampler.shard_size = Some(shard_size);
        self
    }

    /// The `max_docs_per_value` is an optional parameter and limits how many documents
    /// are permitted per choice of de-duplicating value. The default setting is "1".
    pub fn max_docs_per_value(mut self, max_docs_per_value: u64) -> Self {
        self.diversified_sampler.max_docs_per_value = Some(max_docs_per_value);
        self
    }

    /// The optional `execution_hint` setting can influence the management of the values
    /// used for de-duplication. Each option will hold up to `shard_size` values in memory
    /// while performing de-duplication but the type of value held can be controlled as follows:
    /// - hold field values directly (`map`)
    /// - hold ordinals of the field as determined by the Lucene index (`global_ordinals`)
    /// - hold hashes of the field values - with potential for hash collisions (`bytes_hash`)
    pub fn execution_hint(mut self, execution_hint: ExecutionHint) -> Self {
        self.diversified_sampler.execution_hint = Some(execution_hint);
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
            Aggregation::diversified_sampler("catalog_id").shard_size(50),
            json!({
                "diversified_sampler": {
                    "field": "catalog_id",
                    "shard_size": 50
                }
            }),
        );

        assert_serialize_aggregation(
            Aggregation::diversified_sampler("catalog_id")
                .shard_size(50)
                .max_docs_per_value(2)
                .execution_hint(ExecutionHint::GlobalOrdinals)
                .aggregate("catalog", Aggregation::terms("catalog_id"))
                .aggregate("brand", Aggregation::terms("brand_id")),
            json!({
                "diversified_sampler": {
                    "field": "catalog_id",
                    "shard_size": 50,
                    "max_docs_per_value": 2,
                    "execution_hint": "global_ordinals"
                },
                "aggs": {
                    "catalog": {
                        "terms": {
                            "field": "catalog_id"
                        }
                    },
                    "brand": {
                        "terms": {
                            "field": "brand_id"
                        }
                    }
                }
            }),
        );
    }
}
