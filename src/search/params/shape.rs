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
        coordinates: [Coordinate; 2],
    },

    /// A GeoJSON shape similar to the `multi*` shapes except that multiple
    /// types can coexist (e.g., a Point and a LineString)
    #[serde(rename = "geometrycollection")]
    GeometryCollection {
        /// A collection of geo shapes
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
            coordinates: [top_left.into(), bottom_right.into()],
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
