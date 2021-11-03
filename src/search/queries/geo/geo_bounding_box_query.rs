use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// Matches [geo_point](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html)
/// and [geo_shape](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-shape.html)
/// values that intersect a bounding box.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-geo-bounding-box-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GeoBoundingBoxQuery {
    #[serde(rename = "geo_bounding_box")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    #[serde(flatten)]
    pair: KeyValuePair<String, GeoBoundingBox>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    validation_method: Option<ValidationMethod>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`GeoBoundingBoxQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `value` - A series of vertex coordinates of a geo bounding box
    pub fn geo_bounding_box(
        field: impl Into<String>,
        value: impl Into<GeoBoundingBox>,
    ) -> GeoBoundingBoxQuery {
        GeoBoundingBoxQuery {
            inner: Inner {
                pair: KeyValuePair::new(field.into(), value.into()),
                validation_method: None,
                boost: None,
                _name: None,
            },
        }
    }
}

impl GeoBoundingBoxQuery {
    /// Set to `IGNORE_MALFORMED` to accept geo points with invalid latitude or longitude, set to
    /// `COERCE` to also try to infer correct latitude or longitude. (default is `STRICT`).
    pub fn validation_method(mut self, validation_method: impl Into<ValidationMethod>) -> Self {
        self.inner.validation_method = Some(validation_method.into());
        self
    }

    add_boost_and_name!();
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        geo_bounding_box_wkt(
            Query::geo_bounding_box("pin.location", GeoBoundingBox::WellKnownText {
                wkt: "BBOX (-74.1, -71.12, 40.73, 40.01)".into()
            }), json!({
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
            }).validation_method(ValidationMethod::Strict).name("test_name"),
            json!({
                "geo_bounding_box": {
                    "validation_method": "STRICT",
                    "_name": "test_name",
                    "pin.location": {
                        "top_left": [ -74.1, 40.73 ],
                        "bottom_right": [ -71.12, 40.01 ]
                    }
                }
            })
        );

        geo_bounding_box_vertices(
            Query::geo_bounding_box("pin.location", GeoBoundingBox::Vertices {
                top: 40.73,
                left: -74.1,
                bottom: 40.01,
                right: -71.12
            }).validation_method(ValidationMethod::Strict)
            .name("test_name")
            .boost(1),
            json!({
                "geo_bounding_box": {
                    "validation_method": "STRICT",
                    "_name": "test_name",
                    "boost": 1,
                    "pin.location": {
                        "top": 40.73,
                        "left": -74.1,
                        "bottom": 40.01,
                        "right": -71.12
                    }
                }
            })
        )
    }
}
