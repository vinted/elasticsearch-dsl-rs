use crate::search::*;
use crate::util::*;

/// Matches [geo_point](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html)
/// and [geo_shape](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-shape.html)
/// values within a given distance of a geopoint.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-geo-distance-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct GeoDistanceQuery {
    #[serde(skip)]
    field: String,

    #[serde(skip)]
    location: GeoLocation,

    distance: Distance,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    distance_type: Option<GeoDistanceType>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    validation_method: Option<ValidationMethod>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`GeoDistanceQuery`]
    ///
    /// - `field` - Field you wish to search
    /// - `origin` - GeoPoint to measure distance to
    /// - `distance` - Distance threshold
    pub fn geo_distance<T, U, V>(field: T, origin: U, distance: V) -> GeoDistanceQuery
    where
        T: ToString,
        U: Into<GeoLocation>,
        V: Into<Distance>,
    {
        GeoDistanceQuery {
            field: field.to_string(),
            location: origin.into(),
            distance: distance.into(),
            distance_type: None,
            validation_method: None,
            boost: None,
            _name: None,
        }
    }
}

impl GeoDistanceQuery {
    /// Set to `IGNORE_MALFORMED` to accept geo points with invalid latitude or longitude, set to
    /// `COERCE` to also try to infer correct latitude or longitude. (default is `STRICT`).
    pub fn validation_method(mut self, validation_method: ValidationMethod) -> Self {
        self.validation_method = Some(validation_method);
        self
    }

    /// How to compute the distance. Can either be [Arc](GeoDistanceType::Arc) (default), or
    /// [Plane](GeoDistanceType::Plane) (faster, but inaccurate on long distances and close to the
    /// poles).
    pub fn distance_type(mut self, distance_type: GeoDistanceType) -> Self {
        self.distance_type = Some(distance_type);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for GeoDistanceQuery {}

serialize_with_root_key_value_pair!("geo_distance": GeoDistanceQuery, field, location);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::geo_distance(
                "pin.location",
                GeoLocation::new(40.12, -71.34),
                Distance::Kilometers(300),
            ),
            json!({
                "geo_distance": {
                    "distance": "300km",
                    "pin.location": [-71.34, 40.12],
                }
            }),
        );

        assert_serialize_query(
            Query::geo_distance(
                "pin.location",
                GeoLocation::new(40.12, -71.34),
                Distance::Kilometers(300),
            )
            .distance_type(GeoDistanceType::Plane)
            .validation_method(ValidationMethod::Strict)
            .name("test_name")
            .boost(1),
            json!({
                "geo_distance": {
                    "distance": "300km",
                    "distance_type": "plane",
                    "pin.location": [-71.34, 40.12],
                    "validation_method": "STRICT",
                    "_name": "test_name",
                    "boost": 1.0,
                }
            }),
        );
    }
}
