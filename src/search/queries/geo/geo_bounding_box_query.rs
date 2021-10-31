use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use crate::{GeoBoundingBox, Query, ValidationMethod};

/// Matches [geo_point](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html)
/// and [geo_shape](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-shape.html)
/// values that intersect a bounding box.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-geo-bounding-box-query.html>
#[derive(Debug, Clone, PartialEq)]
pub struct GeoBoundingBoxQuery {
    field: String,
    validation_method: Option<ValidationMethod>,
    _name: Option<String>,
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    #[serde(flatten)]
    value: GeoBoundingBox,
}

impl Query {
    pub fn geo_bounding_box(field: impl Into<String>, geo: impl Into<GeoBoundingBox>) -> GeoBoundingBoxQuery {
        GeoBoundingBoxQuery {
            field: field.into(),
            validation_method: None,
            _name: None,
            inner: Inner {
                value: geo.into(),
            },
        }
    }
}

impl GeoBoundingBoxQuery {
    /// Set to `IGNORE_MALFORMED` to accept geo points with invalid latitude or longitude, set to
    /// `COERCE` to also try to infer correct latitude or longitude. (default is `STRICT`).
    pub fn validation_method(mut self, validation_method: impl Into<ValidationMethod>) -> GeoBoundingBoxQuery {
        self.validation_method = Some(validation_method.into());
        self
    }

    /// Optional name field to identify the filter
    pub fn name(mut self, name: impl Into<String>) -> GeoBoundingBoxQuery {
        self._name = Some(name.into());
        self
    }
}

impl Serialize for GeoBoundingBoxQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut hash = std::collections::HashMap::new();
        let _ = hash.insert(&self.field, &self.inner);
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("geo_bounding_box", &hash)?;
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::GeoPoint;
    use super::*;

    test_serialization! {
        geo_bounding_box_wkt(
            Query::geo_bounding_box("pin.location", GeoBoundingBox::Wkt("BBOX (-74.1, -71.12, 40.73, 40.01)".into())),
            json!({
                "geo_bounding_box": {
                    "pin.location": {
                        "wkt": "BBOX (-74.1, -71.12, 40.73, 40.01)"
                    }
                }
            })
        );

        geo_bounding_box_geopoint(
            Query::geo_bounding_box("pin.location", GeoBoundingBox::MainDiagonal {
                top_left: GeoPoint::Coordinates { longitude: -74.1, latitude: 40.73 },
                bottom_right: GeoPoint::Coordinates { longitude: -71.12, latitude: 40.01 }
            })
            .validation_method(ValidationMethod::Strict),
            json!({
                "geo_bounding_box": {
                    "validation_method": "STRICT",
                    "pin.location": {
                        "top_left": [ -74.1, 40.73 ],
                        "bottom_right": [ -71.12, 40.01 ]
                    }
                }
            })
        )
    }
}