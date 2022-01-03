use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// Queries documents that contain fields indexed using the `shape` type.
///
/// Requires the [`shape` Mapping](https://www.elastic.co/guide/en/elasticsearch/reference/current/shape.html).
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-shape-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ShapeQuery {
    #[serde(rename = "shape")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    #[serde(flatten)]
    pair: KeyValuePair<String, InlineShape>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    ignore_unmapped: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct InlineShape {
    shape: Shape,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    relation: Option<SpatialRelation>,
}

impl Query {
    /// Creates an instance of [`ShapeQuery`]
    ///
    /// - `field` - Field you wish to search
    /// - `shape` - Shape you with to search
    pub fn shape<S, T>(field: S, shape: T) -> ShapeQuery
    where
        S: ToString,
        T: Into<Shape>,
    {
        ShapeQuery {
            inner: Inner {
                pair: KeyValuePair::new(
                    field.to_string(),
                    InlineShape {
                        shape: shape.into(),
                        relation: None,
                    },
                ),
                ignore_unmapped: None,
                boost: None,
                _name: None,
            },
        }
    }
}

impl ShapeQuery {
    /// The [shape strategy](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-shape.html#spatial-strategy)
    /// mapping parameter determines which spatial relation operators may be
    /// used at search time.
    pub fn relation(mut self, relation: SpatialRelation) -> Self {
        self.inner.pair.value.relation = Some(relation);
        self
    }

    /// When set to true the `ignore_unmapped` option will ignore an unmapped
    /// field and will not match any documents for this query. This can be
    /// useful when querying multiple indexes which might have different
    /// mappings. When set to `false` (the default value) the query will throw
    /// an exception if the field is not mapped.
    pub fn ignore_unmapped(mut self, ignore_unmapped: bool) -> Self {
        self.inner.ignore_unmapped = Some(ignore_unmapped);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for ShapeQuery {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        assert_serialize(
            Query::shape("pin.location", Shape::point([2.2, 1.1])),
            json!({
                "shape": {
                    "pin.location": {
                        "shape": {
                            "type": "point",
                            "coordinates": [2.2, 1.1]
                        }
                    },
                }
            }),
        );

        assert_serialize(
            Query::shape("pin.location", Shape::point([2.2, 1.1]))
                .boost(2)
                .name("test")
                .ignore_unmapped(true)
                .relation(SpatialRelation::Within),
            json!({
                "shape": {
                    "_name": "test",
                    "boost": 2,
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
