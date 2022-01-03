use crate::search::*;

/// The `geo_shape` data type facilitates the indexing of and searching with
/// arbitrary geo shapes such as rectangles and polygons. It should be used
/// when either the data being indexed or the queries being executed contain
/// shapes other than just points.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum GeoShape {
    /// A single geographic coordinate
    ///
    /// Note: Elasticsearch uses WGS-84 coordinates only
    #[serde(rename = "point")]
    Point {
        /// Coordinates
        coordinates: GeoCoordinate,
    },

    /// An arbitrary line given two or more points
    #[serde(rename = "linestring")]
    LineString {
        /// Coordinates
        coordinates: Vec<GeoCoordinate>,
    },

    /// A closed polygon whose first and last point must match, thus requiring
    /// `n + 1` vertices to create an `n-sided` polygon and a minimum of `4`
    /// vertices
    #[serde(rename = "polygon")]
    Polygon {
        /// Coordinates
        coordinates: Vec<Vec<GeoCoordinate>>,
    },

    /// An array of unconnected, but likely related points
    #[serde(rename = "multipoint")]
    MultiPoint {
        /// Coordinates
        coordinates: Vec<GeoCoordinate>,
    },

    /// An array of separate linestrings
    #[serde(rename = "multilinestring")]
    MultiLineString {
        /// Coordinates
        coordinates: Vec<Vec<GeoCoordinate>>,
    },

    /// An array of separate polygons
    #[serde(rename = "multipolygon")]
    MultiPolygon {
        /// Coordinates
        coordinates: Vec<Vec<Vec<GeoCoordinate>>>,
    },

    /// A bounding rectangle, or envelope, specified by specifying only
    /// the top left and bottom right points.
    #[serde(rename = "envelope")]
    Envelope {
        /// Coordinates
        coordinates: (GeoCoordinate, GeoCoordinate),
    },

    /// A circle specified by a center point and radius with units,
    /// which default to `METERS`
    #[serde(rename = "circle")]
    Circle {
        /// Coordinates
        coordinates: GeoCoordinate,

        /// Circle radius
        radius: Distance,
    },

    /// A GeoJSON shape similar to the `multi*` shapes except that multiple
    /// types can coexist (e.g., a Point and a LineString)
    #[serde(rename = "geometrycollection")]
    GeometryCollection {
        /// A collection of geo shapes
        geometries: Vec<GeoShape>,
    },
}

impl GeoShape {
    /// Creates an instance of [`GeoShape::Point`]
    pub fn point<T>(coordinates: T) -> Self
    where
        T: Into<GeoCoordinate>,
    {
        Self::Point {
            coordinates: coordinates.into(),
        }
    }

    /// Creates an instance of [`GeoShape::LineString`]
    pub fn line_string<T>(coordinates: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<GeoCoordinate>,
    {
        Self::LineString {
            coordinates: coordinates.into_iter().map(Into::into).collect(),
        }
    }

    /// Creates an instance of [`GeoShape::Polygon`]
    pub fn polygon<T>(coordinates: T) -> Self
    where
        T: IntoIterator,
        T::Item: IntoIterator,
        <T::Item as IntoIterator>::Item: Into<GeoCoordinate>,
    {
        Self::Polygon {
            coordinates: coordinates
                .into_iter()
                .map(|x| x.into_iter().map(Into::into).collect())
                .collect(),
        }
    }

    /// Creates an instance of [`GeoShape::MultiPoint`]
    pub fn multi_point<T>(coordinates: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<GeoCoordinate>,
    {
        Self::MultiPoint {
            coordinates: coordinates.into_iter().map(Into::into).collect(),
        }
    }

    /// Creates an instance of [`GeoShape::MultiLineString`]
    pub fn multi_line_string<T>(coordinates: T) -> Self
    where
        T: IntoIterator,
        T::Item: IntoIterator,
        <T::Item as IntoIterator>::Item: Into<GeoCoordinate>,
    {
        Self::MultiLineString {
            coordinates: coordinates
                .into_iter()
                .map(|x| x.into_iter().map(Into::into).collect())
                .collect(),
        }
    }

    /// Creates an instance of [`GeoShape::MultiPolygon`]
    pub fn multi_polygon<T>(coordinates: T) -> Self
    where
        T: IntoIterator,
        T::Item: IntoIterator,
        <T::Item as IntoIterator>::Item: IntoIterator,
        <<T::Item as IntoIterator>::Item as IntoIterator>::Item: Into<GeoCoordinate>,
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

    /// Creates an instance of [`GeoShape::Envelope`]
    pub fn envelope<T>(top_left: T, bottom_right: T) -> Self
    where
        T: Into<GeoCoordinate>,
    {
        Self::Envelope {
            coordinates: (top_left.into(), bottom_right.into()),
        }
    }

    /// Creates an instance of [`GeoShape::Circle`]
    pub fn circle<T, R>(coordinates: T, radius: R) -> Self
    where
        T: Into<GeoCoordinate>,
        R: Into<Distance>,
    {
        Self::Circle {
            coordinates: coordinates.into(),
            radius: radius.into(),
        }
    }

    /// Creates an instance of [`GeoShape::GeometryCollection`]
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
            GeoShape::point([-77.0, 38.0]),
            json!({
                "type": "point",
                "coordinates": [-77.0, 38.0]
            }),
        );

        assert_serialize(
            GeoShape::line_string([[-77.0, 38.0], [-77.0, 38.0]]),
            json!({
                "type": "linestring",
                "coordinates": [[-77.0, 38.0], [-77.0, 38.0]]
            }),
        );

        assert_serialize(
            GeoShape::polygon([
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
            GeoShape::multi_point([[-77.0, 38.0], [-77.0, 38.0]]),
            json!({
                "type": "multipoint",
                "coordinates": [[-77.0, 38.0], [-77.0, 38.0]]
            }),
        );

        assert_serialize(
            GeoShape::multi_line_string([
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
            GeoShape::multi_polygon([
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
            GeoShape::envelope([-77.0, 38.0], [-77.0, 38.0]),
            json!({
                "type": "envelope",
                "coordinates": [[-77.0, 38.0], [-77.0, 38.0]]
            }),
        );

        assert_serialize(
            GeoShape::circle([-45.0, 45.0], Distance::Meters(100)),
            json!({
                "type": "circle",
                "radius": "100m",
                "coordinates": [-45.0, 45.0]
            }),
        );

        assert_serialize(
            GeoShape::geometry_collection([
                GeoShape::envelope([-77.0, 38.0], [-77.0, 38.0]),
                GeoShape::point([-77.0, 38.0]),
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
