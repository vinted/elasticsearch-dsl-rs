use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// Matches [geo_point](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html)
/// and [geo_shape](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-shape.html)
/// values within a given distance of a geopoint.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-geo-distance-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GeoDistanceQuery {
    #[serde(rename = "geo_distance")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    #[serde(flatten)]
    pair: KeyValuePair<String, GeoPoint>,

    distance: Distance,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    distance_type: Option<DistanceType>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    validation_method: Option<ValidationMethod>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

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
        T: Into<String>,
        U: Into<GeoPoint>,
        V: Into<Distance>,
    {
        GeoDistanceQuery {
            inner: Inner {
                pair: KeyValuePair::new(field.into(), origin.into()),
                distance: distance.into(),
                distance_type: None,
                validation_method: None,
                boost: None,
                _name: None,
            },
        }
    }
}

impl GeoDistanceQuery {
    /// Set to `IGNORE_MALFORMED` to accept geo points with invalid latitude or longitude, set to
    /// `COERCE` to also try to infer correct latitude or longitude. (default is `STRICT`).
    pub fn validation_method(mut self, validation_method: ValidationMethod) -> Self {
        self.inner.validation_method = Some(validation_method);
        self
    }

    /// How to compute the distance. Can either be `Arc` (default),
    /// or `Plane` (faster, but inaccurate on long distances and close to the poles).
    pub fn distance_type(mut self, distance_type: DistanceType) -> Self {
        self.inner.distance_type = Some(distance_type);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for GeoDistanceQuery {
    fn should_skip(&self) -> bool {
        self.inner.pair.key.should_skip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::geo_distance(
                "pin.location",
                GeoPoint::Coordinates {
                    latitude: 40.12,
                    longitude: -71.34,
                },
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
                GeoPoint::Geohash("drm3btev3e86".into()),
                Distance::Miles(200),
            ),
            json!({
                "geo_distance": {
                    "distance": "200mi",
                    "pin.location": "drm3btev3e86",
                }
            }),
        );

        assert_serialize_query(
            Query::geo_distance(
                "pin.location",
                GeoPoint::Coordinates {
                    latitude: 40.12,
                    longitude: -71.34,
                },
                Distance::Kilometers(300),
            )
            .distance_type(DistanceType::Plane)
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
                    "boost": 1,
                }
            }),
        );
    }
}
