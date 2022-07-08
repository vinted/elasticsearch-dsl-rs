use crate::search::*;
use crate::util::*;

/// A filtering aggregation used to limit any sub aggregations' processing to a sample of the top-scoring documents.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-sampler-aggregation.html>
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SamplerAggregation {
    sampler: SamplerAggregationInner,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    aggs: Aggregations,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct SamplerAggregationInner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    shard_size: Option<u64>,
}

impl Aggregation {
    /// Creates an instance of [`SamplerAggregation`]
    pub fn sampler() -> SamplerAggregation {
        SamplerAggregation {
            sampler: SamplerAggregationInner { shard_size: None },
            aggs: Aggregations::new(),
        }
    }
}

impl SamplerAggregation {
    /// The shard_size parameter limits how many top-scoring documents are
    /// collected in the sample processed on each shard. The default value is 100.
    pub fn shard_size(mut self, shard_size: u64) -> Self {
        self.sampler.shard_size = Some(shard_size);
        self
    }

    add_aggregate!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_aggregation(Aggregation::sampler(), json!({ "sampler": {} }));

        assert_serialize_aggregation(
            Aggregation::sampler().shard_size(100),
            json!({ "sampler": { "shard_size": 100 } }),
        );

        assert_serialize_aggregation(
            Aggregation::sampler()
                .shard_size(50)
                .aggregate("catalog", Aggregation::terms("catalog_id"))
                .aggregate("brand", Aggregation::terms("brand_id")),
            json!({
                "sampler": { "shard_size": 50 },
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
