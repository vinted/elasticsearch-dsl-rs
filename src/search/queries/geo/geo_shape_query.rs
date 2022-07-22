use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// Filter documents indexed using the `geo_shape` or `geo_point` type.
///
/// Requires the
/// [`geo_shape` mapping](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-shape.html)
/// or the
/// [`geo_point` mapping](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html).
///
/// The `geo_shape` query uses the same grid square representation as the
/// `geo_shape` mapping to find documents that have a shape that intersects
/// with the query shape. It will also use the same Prefix Tree configuration
/// as defined for the field mapping.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-geo-shape-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct GeoShapeQuery {
    #[serde(skip)]
    field: String,

    #[serde(skip)]
    shape: InlineShape,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    ignore_unmapped: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct InlineShape {
    shape: GeoShape,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    relation: Option<SpatialRelation>,
}

impl Query {
    /// Creates an instance of [`GeoShapeQuery`]
    ///
    /// - `field` - Field you wish to search
    /// - `shape` - Shape you with to search
    pub fn geo_shape<S, T>(field: S, shape: T) -> GeoShapeQuery
    where
        S: ToString,
        T: Into<GeoShape>,
    {
        GeoShapeQuery {
            field: field.to_string(),
            shape: InlineShape {
                shape: shape.into(),
                relation: None,
            },
            ignore_unmapped: None,
            boost: None,
            _name: None,
        }
    }
}

impl GeoShapeQuery {
    /// The [geo_shape strategy](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-shape.html#spatial-strategy)
    /// mapping parameter determines which spatial relation operators may be
    /// used at search time.
    pub fn relation(mut self, relation: SpatialRelation) -> Self {
        self.shape.relation = Some(relation);
        self
    }

    /// When set to true the `ignore_unmapped` option will ignore an unmapped
    /// field and will not match any documents for this query. This can be
    /// useful when querying multiple indexes which might have different
    /// mappings. When set to `false` (the default value) the query will throw
    /// an exception if the field is not mapped.
    pub fn ignore_unmapped(mut self, ignore_unmapped: bool) -> Self {
        self.ignore_unmapped = Some(ignore_unmapped);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for GeoShapeQuery {}

serialize_with_root_key_value_pair!("geo_shape": GeoShapeQuery, field, shape);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        assert_serialize_query(
            Query::geo_shape("pin.location", GeoShape::point([2.2, 1.1])),
            json!({
                "geo_shape": {
                    "pin.location": {
                        "shape": {
                            "type": "point",
                            "coordinates": [2.2, 1.1]
                        }
                    },
                }
            }),
        );

        assert_serialize_query(
            Query::geo_shape("pin.location", GeoShape::point([2.2, 1.1]))
                .boost(2)
                .name("test")
                .ignore_unmapped(true)
                .relation(SpatialRelation::Within),
            json!({
                "geo_shape": {
                    "_name": "test",
                    "boost": 2.0,
                    "ignore_unmapped": true,
                    "pin.location": {
                        "shape": {
                            "type": "point",
                            "coordinates": [2.2, 1.1]
                        },
                        "relation": "WITHIN"
                    },
                }
            }),
        );
    }
}
