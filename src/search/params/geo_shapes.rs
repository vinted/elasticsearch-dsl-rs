use crate::search::*;

/// A single geographic coordinate
///
/// Note: Elasticsearch uses WGS-84 coordinates only
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct PointGeoShape {
    /// Coordinates
    pub coordinates: GeoCoordinate,
}

/// An arbitrary line given two or more points
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LineStringGeoShape {
    /// Coordinates
    pub coordinates: Vec<GeoCoordinate>,
}

/// A closed polygon whose first and last point must match, thus requiring
/// `n + 1` vertices to create an `n-sided` polygon and a minimum of `4`
/// vertices
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PolygonGeoShape {
    /// Coordinates
    pub coordinates: Vec<Vec<GeoCoordinate>>,
}

/// An array of unconnected, but likely related points
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MultiPointGeoShape {
    /// Coordinates
    pub coordinates: Vec<GeoCoordinate>,
}

/// An array of separate linestrings
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MultiLineStringGeoShape {
    /// Coordinates
    pub coordinates: Vec<Vec<GeoCoordinate>>,
}

/// An array of separate polygons
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MultiPolygonGeoShape {
    /// Coordinates
    pub coordinates: Vec<Vec<Vec<GeoCoordinate>>>,
}

/// A bounding rectangle, or envelope, specified by specifying only
/// the top left and bottom right points.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EnvelopeGeoShape {
    /// Coordinates
    pub coordinates: Vec<GeoCoordinate>,
}

/// A circle specified by a center point and radius with units,
/// which default to `METERS`
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct CircleGeoShape {
    /// Coordinates
    pub coordinates: GeoCoordinate,

    /// Circle radius
    pub radius: Distance,
}

/// A GeoJSON shape similar to the `multi*` shapes except that multiple types
/// can coexist (e.g., a Point and a LineString)
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GeometryCollection {
    /// A collection of geo shapes
    pub geometries: Vec<GeoShape>,
}

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
    Point(PointGeoShape),

    /// An arbitrary line given two or more points
    #[serde(rename = "linestring")]
    LineString(LineStringGeoShape),

    /// A closed polygon whose first and last point must match, thus requiring
    /// `n + 1` vertices to create an `n-sided` polygon and a minimum of `4`
    /// vertices
    #[serde(rename = "polygon")]
    Polygon(PolygonGeoShape),

    /// An array of unconnected, but likely related points
    #[serde(rename = "multipoint")]
    MultiPoint(MultiPointGeoShape),

    /// An array of separate linestrings
    #[serde(rename = "multilinestring")]
    MultiLineString(MultiLineStringGeoShape),

    /// An array of separate polygons
    #[serde(rename = "multipolygon")]
    MultiPolygon(MultiPolygonGeoShape),

    /// A bounding rectangle, or envelope, specified by specifying only
    /// the top left and bottom right points.
    #[serde(rename = "envelope")]
    Envelope(EnvelopeGeoShape),

    /// A circle specified by a center point and radius with units,
    /// which default to `METERS`
    #[serde(rename = "circle")]
    Circle(CircleGeoShape),

    /// A GeoJSON shape similar to the `multi*` shapes except that multiple
    /// types can coexist (e.g., a Point and a LineString)
    #[serde(rename = "geometrycollection")]
    GeometryCollection(GeometryCollection),
}
