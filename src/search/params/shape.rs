use crate::search::*;

/// The `shape` data type facilitates the indexing of and searching with
/// arbitrary `x, y` cartesian shapes such as rectangles and polygons. It can
/// be used to index and query geometries whose coordinates fall in a
/// 2-dimensional planar coordinate system.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Shape {
    /// A single `x, y` coordinate
    #[serde(rename = "point")]
    Point {
        /// Coordinates
        coordinates: Coordinate,
    },

    /// An arbitrary line given two or more points
    #[serde(rename = "linestring")]
    LineString {
        /// Coordinates
        coordinates: Vec<Coordinate>,
    },

    /// A closed polygon whose first and last point must match, thus requiring
    /// `n + 1` vertices to create an `n-sided` polygon and a minimum of `4`
    /// vertices
    #[serde(rename = "polygon")]
    Polygon {
        /// Coordinates
        coordinates: Vec<Vec<Coordinate>>,
    },

    /// An array of unconnected, but likely related points
    #[serde(rename = "multipoint")]
    MultiPoint {
        /// Coordinates
        coordinates: Vec<Coordinate>,
    },

    /// An array of separate linestrings
    #[serde(rename = "multilinestring")]
    MultiLineString {
        /// Coordinates
        coordinates: Vec<Vec<Coordinate>>,
    },

    /// An array of separate polygons
    #[serde(rename = "multipolygon")]
    MultiPolygon {
        /// Coordinates
        coordinates: Vec<Vec<Vec<Coordinate>>>,
    },

    /// A bounding rectangle, or envelope, specified by specifying only
    /// the top left and bottom right points.
    #[serde(rename = "envelope")]
    Envelope {
        /// Coordinates
        coordinates: (Coordinate, Coordinate),
    },

    /// A GeoJSON shape similar to the `multi*` shapes except that multiple
    /// types can coexist (e.g., a Point and a LineString)
    #[serde(rename = "geometrycollection")]
    GeometryCollection {
        /// A collection of shapes
        geometries: Vec<Shape>,
    },
}

impl Shape {
    /// Creates an instance of [`Shape::Point`]
    pub fn point<T>(coordinates: T) -> Self
    where
        T: Into<Coordinate>,
    {
        Self::Point {
            coordinates: coordinates.into(),
        }
    }

    /// Creates an instance of [`Shape::LineString`]
    pub fn line_string<T>(coordinates: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Coordinate>,
    {
        Self::LineString {
            coordinates: coordinates.into_iter().map(Into::into).collect(),
        }
    }

    /// Creates an instance of [`Shape::Polygon`]
    pub fn polygon<T>(coordinates: T) -> Self
    where
        T: IntoIterator,
        T::Item: IntoIterator,
        <T::Item as IntoIterator>::Item: Into<Coordinate>,
    {
        Self::Polygon {
            coordinates: coordinates
                .into_iter()
                .map(|x| x.into_iter().map(Into::into).collect())
                .collect(),
        }
    }

    /// Creates an instance of [`Shape::MultiPoint`]
    pub fn multi_point<T>(coordinates: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Coordinate>,
    {
        Self::MultiPoint {
            coordinates: coordinates.into_iter().map(Into::into).collect(),
        }
    }

    /// Creates an instance of [`Shape::MultiLineString`]
    pub fn multi_line_string<T>(coordinates: T) -> Self
    where
        T: IntoIterator,
        T::Item: IntoIterator,
        <T::Item as IntoIterator>::Item: Into<Coordinate>,
    {
        Self::MultiLineString {
            coordinates: coordinates
                .into_iter()
                .map(|x| x.into_iter().map(Into::into).collect())
                .collect(),
        }
    }

    /// Creates an instance of [`Shape::MultiPolygon`]
    pub fn multi_polygon<T>(coordinates: T) -> Self
    where
        T: IntoIterator,
        T::Item: IntoIterator,
        <T::Item as IntoIterator>::Item: IntoIterator,
        <<T::Item as IntoIterator>::Item as IntoIterator>::Item: Into<Coordinate>,
    {
        Self::MultiPolygon {
            coordinates: coordinates
                .into_iter()
                .map(|x| {
                    x.into_iter()
                        .map(|y| y.into_iter().map(Into::into).collect())
                        .collect()
                })
                .collect(),
        }
    }

    /// Creates an instance of [`Shape::Envelope`]
    pub fn envelope<T>(top_left: T, bottom_right: T) -> Self
    where
        T: Into<Coordinate>,
    {
        Self::Envelope {
            coordinates: (top_left.into(), bottom_right.into()),
        }
    }

    /// Creates an instance of [`Shape::GeometryCollection`]
    pub fn geometry_collection<T>(geometries: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Self>,
    {
        Self::GeometryCollection {
            geometries: geometries.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Shape::point([-77.0, 38.0]),
            json!({
                "type": "point",
                "coordinates": [-77.0, 38.0]
            }),
        );

        assert_serialize(
            Shape::line_string([[-77.0, 38.0], [-77.0, 38.0]]),
            json!({
                "type": "linestring",
                "coordinates": [[-77.0, 38.0], [-77.0, 38.0]]
            }),
        );

        assert_serialize(
            Shape::polygon([
                vec![
                    [-17.0, 10.0],
                    [16.0, 15.0],
                    [12.0, 0.0],
                    [16.0, -15.0],
                    [-17.0, -10.0],
                    [-17.0, 10.0],
                ],
                vec![[18.2, 8.2], [-18.8, 8.2], [-10.8, -8.8], [18.2, 8.2]],
            ]),
            json!({
                "type": "polygon",
                "coordinates": [
                    [
                        [-17.0, 10.0],
                        [16.0, 15.0],
                        [12.0, 0.0],
                        [16.0, -15.0],
                        [-17.0, -10.0],
                        [-17.0, 10.0],
                    ],
                    [
                        [18.2, 8.2],
                        [-18.8, 8.2],
                        [-10.8, -8.8],
                        [18.2, 8.2],
                    ],
                ]
            }),
        );

        assert_serialize(
            Shape::multi_point([[-77.0, 38.0], [-77.0, 38.0]]),
            json!({
                "type": "multipoint",
                "coordinates": [[-77.0, 38.0], [-77.0, 38.0]]
            }),
        );

        assert_serialize(
            Shape::multi_line_string([
                [[12.0, 2.0], [13.0, 2.0], [13.0, 3.0], [12.0, 3.0]],
                [[10.0, 0.0], [11.0, 0.0], [11.0, 1.0], [10.0, 1.0]],
                [[10.2, 0.2], [10.8, 0.2], [10.8, 0.8], [12.0, 0.8]],
            ]),
            json!({
                "type": "multilinestring",
                "coordinates": [
                    [[12.0, 2.0], [13.0, 2.0], [13.0, 3.0], [12.0, 3.0]],
                    [[10.0, 0.0], [11.0, 0.0], [11.0, 1.0], [10.0, 1.0]],
                    [[10.2, 0.2], [10.8, 0.2], [10.8, 0.8], [12.0, 0.8]],
                ]
            }),
        );

        assert_serialize(
            Shape::multi_polygon([
                vec![
                    vec![
                        [-17.0, 10.0],
                        [16.0, 15.0],
                        [12.0, 0.0],
                        [16.0, -15.0],
                        [-17.0, -10.0],
                        [-17.0, 10.0],
                    ],
                    vec![[18.2, 8.2], [-18.8, 8.2], [-10.8, -8.8], [18.2, 8.2]],
                ],
                vec![vec![
                    [-15.0, 8.0],
                    [16.0, 15.0],
                    [12.0, 0.0],
                    [16.0, -15.0],
                    [-17.0, -10.0],
                    [-15.0, 8.0],
                ]],
            ]),
            json!({
                "type": "multipolygon",
                "coordinates": [
                    [
                        [
                            [-17.0, 10.0],
                            [16.0, 15.0],
                            [12.0, 0.0],
                            [16.0, -15.0],
                            [-17.0, -10.0],
                            [-17.0, 10.0],
                        ],
                        [
                            [18.2, 8.2],
                            [-18.8, 8.2],
                            [-10.8, -8.8],
                            [18.2, 8.2],
                        ],
                    ],
                    [
                        [
                            [-15.0, 8.0],
                            [16.0, 15.0],
                            [12.0, 0.0],
                            [16.0, -15.0],
                            [-17.0, -10.0],
                            [-15.0, 8.0],
                        ]
                    ],
                ]
            }),
        );

        assert_serialize(
            Shape::envelope([-77.0, 38.0], [-77.0, 38.0]),
            json!({
                "type": "envelope",
                "coordinates": [[-77.0, 38.0], [-77.0, 38.0]]
            }),
        );

        assert_serialize(
            Shape::geometry_collection([
                Shape::envelope([-77.0, 38.0], [-77.0, 38.0]),
                Shape::point([-77.0, 38.0]),
            ]),
            json!({
                "type": "geometrycollection",
                "geometries": [
                    {
                        "type": "envelope",
                        "coordinates": [[-77.0, 38.0], [-77.0, 38.0]]
                    },
                    {
                        "type": "point",
                        "coordinates": [-77.0, 38.0]
                    },
                ]
            }),
        );
    }
}
