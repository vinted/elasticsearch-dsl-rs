use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// Queries documents that contain fields indexed using the `shape` type.
///
/// Requires the [`shape` Mapping](https://www.elastic.co/guide/en/elasticsearch/reference/current/shape.html).
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-shape-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct ShapeLookupQuery {
    #[serde(skip)]
    field: String,

    #[serde(skip)]
    shape: Shape,

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
    /// Creates an instance of [`ShapeLookupQuery`]
    ///
    /// - `field` - Field you wish to search
    /// - `id` - The ID of the document that containing the pre-indexed shape
    pub fn shape_lookup<S, T>(field: S, id: T) -> ShapeLookupQuery
    where
        S: ToString,
        T: ToString,
    {
        ShapeLookupQuery {
            field: field.to_string(),
            shape: Shape {
                indexed_shape: IndexedShape {
                    id: id.to_string(),
                    index: None,
                    path: None,
                    routing: None,
                },
                relation: None,
            },
            ignore_unmapped: None,
            boost: None,
            _name: None,
        }
    }
}

impl ShapeLookupQuery {
    /// Name of the index where the pre-indexed shape is. Defaults to shapes
    pub fn index<S>(mut self, index: S) -> Self
    where
        S: ToString,
    {
        self.shape.indexed_shape.index = Some(index.to_string());
        self
    }

    /// The field specified as path containing the pre-indexed shape. Defaults to shape
    pub fn path<S>(mut self, path: S) -> Self
    where
        S: ToString,
    {
        self.shape.indexed_shape.path = Some(path.to_string());
        self
    }

    /// The routing of the shape document
    pub fn routing<S>(mut self, routing: S) -> Self
    where
        S: ToString,
    {
        self.shape.indexed_shape.routing = Some(routing.to_string());
        self
    }

    /// The [shape strategy](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-shape.html#spatial-strategy)
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

impl ShouldSkip for ShapeLookupQuery {}

serialize_with_root_key_value_pair!("shape": ShapeLookupQuery, field, shape);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        assert_serialize_query(
            Query::shape_lookup("pin.location", "id"),
            json!({
                "shape": {
                    "pin.location": {
                        "indexed_shape": {
                            "id": "id",
                        }
                    },
                }
            }),
        );

        assert_serialize_query(
            Query::shape_lookup("pin.location", "id")
                .boost(2)
                .name("test")
                .ignore_unmapped(true)
                .relation(SpatialRelation::Within)
                .routing("routing")
                .index("index")
                .path("path"),
            json!({
                "shape": {
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
