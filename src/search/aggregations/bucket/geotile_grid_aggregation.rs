use crate::search::*;
use crate::util::*;

#[derive(Debug, Clone, Serialize, PartialEq)]
/// A multi-bucket aggregation that groups geo_point and geo_shape values into buckets that represent a grid.
/// The resulting grid can be sparse and only contains cells that have matching data. Each cell corresponds to
/// a map tile as used by many online map sites. Each cell is labeled using a "{zoom}/{x}/{y}" format, where
/// zoom is equal to the user-specified precision.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-geotilegrid-aggregation.html>
pub struct GeotileGridAggregation {
    geotile_grid: GeotileGridAggregationInner,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    aggs: Aggregations,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct GeotileGridAggregationInner {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    precision: Option<u8>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    bounds: Option<GeoBoundingBox>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    size: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    shard_size: Option<u64>,
}

impl Aggregation {
    /// Creates an instance of [`GeotileGridAggregation`]
    ///
    /// - `field` - field to group by
    pub fn geotile_grid<T>(field: T) -> GeotileGridAggregation
    where
        T: ToString,
    {
        GeotileGridAggregation {
            geotile_grid: GeotileGridAggregationInner {
                field: field.to_string(),
                precision: None,
                bounds: None,
                size: None,
                shard_size: None,
            },
            aggs: Aggregations::new(),
        }
    }
}

impl GeotileGridAggregation {
    /// The `size` parameter can be set to define the maximum number of buckets to return.Defaults to 10,000.
    /// When results are trimmed, buckets are prioritized based on the volume of documents they contain.
    pub fn size(mut self, size: u64) -> Self {
        self.geotile_grid.size = Some(size);
        self
    }

    /// The `shard_size` parameter limits the number of buckets returned from each shard.
    /// Defaults to max(10,(size x number-of-shards)) to allow for a more accurate count of
    /// the top cells in the final result.Since each shard could have a different top result order,
    /// using a larger number here reduces the risk of inaccurate counts, but incurs a performance cost.
    pub fn shard_size(mut self, shard_size: u64) -> Self {
        self.geotile_grid.shard_size = Some(shard_size);
        self
    }

    /// the `precision` parameter is used to define cells/buckets in the results. Defaults to 7.
    /// Values outside of \[0,29\] will be rejected.
    pub fn precision(mut self, precision: u8) -> Self {
        self.geotile_grid.precision = Some(precision);
        self
    }

    /// the `bounds` parameter defines the bounding box used to filter the geo-points or geo-shapes in each bucket.
    /// Accepts the same bounding box formats as the [`GeoBoundingBoxQuery`]
    pub fn bounds<T>(mut self, bounds: T) -> Self
    where
        T: Into<GeoBoundingBox>,
    {
        self.geotile_grid.bounds = Some(bounds.into());
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
            Aggregation::geotile_grid("test_field"),
            json!({ "geotile_grid": { "field": "test_field" } }),
        );

        assert_serialize_aggregation(
            Aggregation::geotile_grid("test_field")
                .size(5)
                .precision(18)
                .shard_size(100)
                .bounds(GeoBoundingBox::Vertices {
                    top: 5.0,
                    left: 52.4,
                    bottom: 4.9,
                    right: 52.3,
                }),
            json!({
                "geotile_grid": {
                    "field": "test_field",
                    "size": 5,
                    "precision": 18,
                    "shard_size": 100,
                    "bounds": {
                        "top": 5.0,
                        "left": 52.4,
                        "bottom": 4.9,
                        "right": 52.3,
                    }
                }
            }),
        );

        assert_serialize_aggregation(
            Aggregation::geotile_grid("test_field")
                .size(0)
                .precision(22)
                .shard_size(16)
                .bounds(GeoBoundingBox::Vertices {
                    top: 25.0,
                    left: 102.4,
                    bottom: 7.9,
                    right: 152.3,
                })
                .aggregate(
                    "test_sub_agg",
                    Aggregation::geotile_grid("test_field2")
                        .size(2)
                        .precision(12)
                        .shard_size(22)
                        .bounds(GeoBoundingBox::WellKnownText {
                            wkt: "BBOX (-74.1, -71.12, 40.73, 40.01)".to_string(),
                        }),
                ),
            json!({
                "geotile_grid": {
                    "field": "test_field",
                    "size": 0,
                    "precision": 22,
                    "shard_size": 16,
                    "bounds": {
                        "top": 25.0,
                        "left": 102.4,
                        "bottom": 7.9,
                        "right": 152.3,
                    }
                },
                "aggs": {
                    "test_sub_agg": {
                        "geotile_grid": {
                            "field": "test_field2",
                            "size": 2,
                            "precision": 12,
                            "shard_size": 22,
                            "bounds":{
                                "wkt": "BBOX (-74.1, -71.12, 40.73, 40.01)"
                            }
                        }
                    }
                }
            }),
        );
    }
}
