use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// Matches [geo_point](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html)
/// and [geo_shape](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-shape.html)
/// values that intersect a bounding box.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-geo-bounding-box-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct GeoBoundingBoxQuery {
    #[serde(skip)]
    field: String,

    #[serde(skip)]
    bounding_box: GeoBoundingBox,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    validation_method: Option<ValidationMethod>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}
impl Query {
    /// Creates an instance of [`GeoBoundingBoxQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `bounding_box` - A series of vertex coordinates of a geo bounding box
    pub fn geo_bounding_box<T, U>(field: T, bounding_box: U) -> GeoBoundingBoxQuery
    where
        T: ToString,
        U: Into<GeoBoundingBox>,
    {
        GeoBoundingBoxQuery {
            field: field.to_string(),
            bounding_box: bounding_box.into(),
            validation_method: None,
            boost: None,
            _name: None,
        }
    }
}

impl GeoBoundingBoxQuery {
    /// Set to `IGNORE_MALFORMED` to accept geo points with invalid latitude or longitude, set to
    /// `COERCE` to also try to infer correct latitude or longitude. (default is `STRICT`).
    pub fn validation_method(mut self, validation_method: ValidationMethod) -> Self {
        self.validation_method = Some(validation_method);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for GeoBoundingBoxQuery {}

serialize_with_root_key_value_pair!("geo_bounding_box": GeoBoundingBoxQuery, field, bounding_box);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::geo_bounding_box(
                "pin.location",
                GeoBoundingBox::WellKnownText {
                    wkt: "BBOX (-74.1, -71.12, 40.73, 40.01)".into(),
                },
            ),
            json!({
                "geo_bounding_box": {
                    "pin.location": {
                        "wkt": "BBOX (-74.1, -71.12, 40.73, 40.01)"
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::geo_bounding_box(
                "pin.location",
                GeoBoundingBox::MainDiagonal {
                    top_left: GeoLocation::new(40.73, -74.1),
                    bottom_right: GeoLocation::new(40.01, -71.12),
                },
            )
            .validation_method(ValidationMethod::Strict)
            .name("test_name"),
            json!({
                "geo_bounding_box": {
                    "validation_method": "STRICT",
                    "_name": "test_name",
                    "pin.location": {
                        "top_left": [ -74.1, 40.73 ],
                        "bottom_right": [ -71.12, 40.01 ]
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::geo_bounding_box(
                "pin.location",
                GeoBoundingBox::Vertices {
                    top: 40.73,
                    left: -74.1,
                    bottom: 40.01,
                    right: -71.12,
                },
            )
            .validation_method(ValidationMethod::Strict)
            .name("test_name")
            .boost(1),
            json!({
                "geo_bounding_box": {
                    "validation_method": "STRICT",
                    "_name": "test_name",
                    "boost": 1.0,
                    "pin.location": {
                        "top": 40.73,
                        "left": -74.1,
                        "bottom": 40.01,
                        "right": -71.12
                    }
                }
            }),
        )
    }
}
