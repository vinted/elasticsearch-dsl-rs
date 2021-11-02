use crate::GeoPoint;
use serde::Serialize;

/// Strategies to verify the correctness of coordinates
#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValidationMethod {
    /// accept geo points with invalid latitude or longitude
    IgnoreMalformed,
    /// try to infer correct latitude or longitude
    Coerce,
    /// strict mode
    Strict,
}

/// Different representations of geo bounding box
#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(untagged)]
pub enum GeoBoundingBox {
    /// MainDiagonal vertices of geo bounding box
    MainDiagonal {
        /// The coordinates of the upper left vertex
        top_left: GeoPoint,
        /// The coordinates of the lower right vertex
        bottom_right: GeoPoint,
    },
    /// SubDiagonal vertices of geo bounding box
    SubDiagonal {
        /// The coordinates of the upper right vertex
        top_right: GeoPoint,
        /// The coordinates of the lower left vertex
        bottom_left: GeoPoint,
    },
    /// Well-Known Text (WKT).
    WellKnownText {
        /// e.g. `BBOX (-74.1, -71.12, 40.73, 40.01)`
        wkt: String,
    },
    /// The vertices of the bounding box can either be set by `top_left` and `bottom_right` or by
    /// `top_right` and `bottom_left` parameters. More over the names `topLeft`, `bottomRight`, `topRight`
    /// and `bottomLeft` are supported. Instead of setting the values pairwise, one can use the simple
    /// names `top`, `left`, `bottom` and `right` to set the values separately.
    Vertices {
        /// Set top separately
        top: f32,
        /// Set left separately
        left: f32,
        /// Set bottom separately
        bottom: f32,
        /// Set right separately
        right: f32,
    },
}

#[cfg(test)]
mod tests {
    use crate::{GeoBoundingBox, GeoPoint};

    test_serialization! {
        serializes_as_geo_bounding_box_geopoint(GeoBoundingBox::MainDiagonal {
            top_left: GeoPoint::Coordinates {longitude: -74.1, latitude: 40.73},
            bottom_right: GeoPoint::Coordinates {longitude: -71.12, latitude: 40.01}
        }, json!({
            "top_left": [-74.1, 40.73],
            "bottom_right": [-71.12, 40.01]
        }));
        serializes_as_geo_bounding_box_geohash(GeoBoundingBox::SubDiagonal {
            top_right: GeoPoint::Geohash("dr5r9ydj2y73".into()),
            bottom_left: GeoPoint::Geohash("drj7teegpus6".into())
        }, json!({
            "top_right": "dr5r9ydj2y73",
            "bottom_left": "drj7teegpus6"
        }));
        serializes_as_geo_bounding_box_wkt(GeoBoundingBox::WellKnownText {
            wkt: "BBOX (-74.1, -71.12, 40.73, 40.01)".into()
        }, json!({
            "wkt": "BBOX (-74.1, -71.12, 40.73, 40.01)"
        }));
        serializes_as_geo_bounding_box_vertices(GeoBoundingBox::Vertices {
            top: 40.73,
            left: -74.1,
            bottom: 40.01,
            right: -71.12,
        }, json!({
            "top": 40.73,
            "left": -74.1,
            "bottom": 40.01,
            "right": -71.12
        }));
    }
}
