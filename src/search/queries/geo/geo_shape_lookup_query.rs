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
pub struct GeoShapeLookupQuery {
    #[serde(flatten)]
    pair: KeyValuePair<String, Shape>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    ignore_unmapped: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Shape {
    indexed_shape: IndexedShape,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    relation: Option<SpatialRelation>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct IndexedShape {
    id: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    index: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    path: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    routing: Option<String>,
}

impl Query {
    /// Creates an instance of [`GeoShapeLookupQuery`]
    ///
    /// - `field` - Field you wish to search
    /// - `id` - The ID of the document that containing the pre-indexed shape
    pub fn geo_shape_lookup<S, T>(field: S, id: T) -> GeoShapeLookupQuery
    where
        S: ToString,
        T: ToString,
    {
        GeoShapeLookupQuery {
            pair: KeyValuePair::new(
                field.to_string(),
                Shape {
                    indexed_shape: IndexedShape {
                        id: id.to_string(),
                        index: None,
                        path: None,
                        routing: None,
                    },
                    relation: None,
                },
            ),
            ignore_unmapped: None,
            boost: None,
            _name: None,
        }
    }
}

impl GeoShapeLookupQuery {
    /// Name of the index where the pre-indexed shape is. Defaults to shapes
    pub fn index<S>(mut self, index: S) -> Self
    where
        S: ToString,
    {
        self.pair.value.indexed_shape.index = Some(index.to_string());
        self
    }

    /// The field specified as path containing the pre-indexed shape. Defaults to shape
    pub fn path<S>(mut self, path: S) -> Self
    where
        S: ToString,
    {
        self.pair.value.indexed_shape.path = Some(path.to_string());
        self
    }

    /// The routing of the shape document
    pub fn routing<S>(mut self, routing: S) -> Self
    where
        S: ToString,
    {
        self.pair.value.indexed_shape.routing = Some(routing.to_string());
        self
    }

    /// The [geo_shape strategy](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-shape.html#spatial-strategy)
    /// mapping parameter determines which spatial relation operators may be
    /// used at search time.
    pub fn relation(mut self, relation: SpatialRelation) -> Self {
        self.pair.value.relation = Some(relation);
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

impl ShouldSkip for GeoShapeLookupQuery {}

serialize_with_root!("geo_shape": GeoShapeLookupQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        assert_serialize_query(
            Query::geo_shape_lookup("pin.location", "id"),
            json!({
                "geo_shape": {
                    "pin.location": {
                        "indexed_shape": {
                            "id": "id",
                        }
                    },
                }
            }),
        );

        assert_serialize_query(
            Query::geo_shape_lookup("pin.location", "id")
                .boost(2)
                .name("test")
                .ignore_unmapped(true)
                .relation(SpatialRelation::Within)
                .routing("routing")
                .index("index")
                .path("path"),
            json!({
                "geo_shape": {
                    "_name": "test",
                    "boost": 2,
                    "ignore_unmapped": true,
                    "pin.location": {
                        "indexed_shape": {
                            "id": "id",
                            "index": "index",
                            "path": "path",
                            "routing": "routing"
                        },
                        "relation": "WITHIN"
                    },
                }
            }),
        );
    }
}
