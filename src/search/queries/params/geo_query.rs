use crate::GeoPoint;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

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
#[derive(Debug, PartialEq, Clone)]
pub enum GeoBoundingBox {
    /// Diagonal vertices of geo bounding box
    MainDiagonal {
        top_left: GeoPoint,
        bottom_right: GeoPoint,
    },
    SubDiagonal {
        top_right: GeoPoint,
        bottom_left: GeoPoint,
    },
    /// Well-Known Text (WKT). e.g. `BBOX (-74.1, -71.12, 40.73, 40.01)`
    Wkt(String),
    /// The vertices of the bounding box can either be set by `top_left` and `bottom_right` or by
    /// `top_right` and `bottom_left` parameters. More over the names `topLeft`, `bottomRight`, `topRight`
    /// and `bottomLeft` are supported. Instead of setting the values pairwise, one can use the simple
    /// names `top`, `left`, `bottom` and `right` to set the values separately.
    Vertices {
        top: f32,
        left: f32,
        bottom: f32,
        right: f32,
    },
}

impl Serialize for GeoBoundingBox {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::MainDiagonal {
                top_left,
                bottom_right,
            } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("top_left", top_left)?;
                map.serialize_entry("bottom_right", bottom_right)?;
                map.end()
            }
            Self::SubDiagonal {
                top_right,
                bottom_left,
            } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("top_right", top_right)?;
                map.serialize_entry("bottom_left", bottom_left)?;
                map.end()
            }
            Self::Wkt(wkt) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("wkt", wkt)?;
                map.end()
            }
            Self::Vertices {
                top,
                left,
                bottom,
                right,
            } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("top", top)?;
                map.serialize_entry("left", left)?;
                map.serialize_entry("bottom", bottom)?;
                map.serialize_entry("right", right)?;
                map.end()
            }
        }
    }
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
        serializes_as_geo_bounding_box_wkt(GeoBoundingBox::Wkt("BBOX (-74.1, -71.12, 40.73, 40.01)".into()),
           json!({
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
