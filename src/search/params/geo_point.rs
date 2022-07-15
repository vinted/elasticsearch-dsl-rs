use serde::ser::{Serialize, Serializer};

/// Fields of type geo_point accept latitude-longitude pairs.
///
/// It can be used:
/// - to find geo-points within a
/// [bounding box](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-geo-bounding-box-query.html)
/// , within a certain
/// [distance](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-geo-distance-query.html)
/// of a central point, or within a
/// [polygon](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-geo-polygon-query.html)
/// or within a
/// [geo_shape query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-geo-shape-query.html).
/// - to aggregate documents
/// [geographically](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-geohashgrid-aggregation.html)
/// or by
/// [distance](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-geodistance-aggregation.html)
/// from a central point.
/// - to integrate distance into a document’s
/// [relevance score](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-function-score-query.html).
/// - to [sort](https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#geo-sorting)
/// documents by distance.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html>
#[derive(Debug, PartialEq, Clone)]
pub enum GeoPoint {
    /// Geo-point expressed as an array with the format: [`lon`, `lat`]
    Coordinates {
        /// Latitudes measure an angle up from the equator
        /// (latitudes to the south are negative).
        latitude: f32,

        /// A longitude is an angle from the prime meridian,
        /// measured to the east (longitudes to the west are negative)
        longitude: f32,
    },
    /// Geo-point expressed as a geohash
    Geohash(String),
}

impl GeoPoint {
    /// Creates an instance of [GeoPoint](GeoPoint)
    pub fn coordinates(latitude: f32, longitude: f32) -> Self {
        Self::Coordinates {
            latitude,
            longitude,
        }
    }

    /// Creates an instance of [GeoPoint](GeoPoint)
    pub fn geohash<T>(geohash: T) -> Self
    where
        T: Into<String>,
    {
        Self::Geohash(geohash.into())
    }
}

impl Serialize for GeoPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Coordinates {
                latitude,
                longitude,
            } => (longitude, latitude).serialize(serializer),
            Self::Geohash(geohash) => geohash.serialize(serializer),
        }
    }
}

impl IntoIterator for GeoPoint {
    type Item = Self;

    type IntoIter = std::option::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Some(self).into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_coordinates_successfully() {
        let geo_point = GeoPoint::coordinates(-40f32, -70f32);

        let result = serde_json::to_string(&geo_point).unwrap();

        let expectation = "[-70.0,-40.0]";

        assert_eq!(result, expectation);
    }

    #[test]
    fn serializes_geohash_successfully() {
        let geo_point = GeoPoint::geohash("ww8p1r4t8");

        let result = serde_json::to_string(&geo_point).unwrap();

        let expectation = "\"ww8p1r4t8\"";

        assert_eq!(result, expectation);
    }
}
