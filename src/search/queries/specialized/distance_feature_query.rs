use crate::search::*;
use crate::util::*;
use chrono::{DateTime, Utc};
use serde::ser::Serialize;
use std::fmt::Debug;

#[doc(hidden)]
pub trait Origin: Debug + PartialEq + Serialize + Clone {
    type Pivot: Debug + PartialEq + Serialize + Clone;
}

impl Origin for DateTime<Utc> {
    type Pivot = Time;
}

impl Origin for GeoPoint {
    type Pivot = Distance;
}

/// Boosts the [relevance score](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores)
/// of documents closer to a provided `origin` date or point.
/// For example, you can use this query to give more weight to documents
/// closer to a certain date or location.
///
/// You can use the `distance_feature` query to find the nearest neighbors to a location.
/// You can also use the query in a [bool](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-bool-query.html)
/// search’s `should` filter to add boosted relevance scores to the `bool` query’s scores.
///
/// **How the `distance_feature` query calculates relevance scores**
///
/// The `distance_feature` query dynamically calculates the distance between the
/// `origin` value and a document's field values. It then uses this distance as a
/// feature to boost the
/// [relevance-scores](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores)
/// of closer documents.
///
/// The `distance_feature` query calculates a document's
/// [relevance score](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores)
/// as follows:
///
/// ```text
/// relevance score = boost * pivot / (pivot + distance)
/// ```
///
/// The `distance` is the absolute difference between the `origin` value and a
/// document's field value.
///
/// **Skip non-competitive hits**
///
/// Unlike the
/// [`function_score`](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-function-score-query.html)
/// query or other ways to change
/// [relevance scores](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores)
/// , the `distance_feature` query efficiently skips non-competitive hits when the
/// [`track_total_hits`](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-uri-request.html)
/// parameter is **not** `true`.
///
/// To create distance feature query date query:
/// ```
/// # use elasticsearch_dsl::Time;
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # use chrono::prelude::*;
/// # let query =
/// Query::distance_feature("test", Utc.ymd(2014, 7, 8).and_hms(9, 1, 0), Time::Days(7))
///     .boost(1.5)
///     .name("test");
/// ```
/// To create distance feature query geo query:
/// ```
/// # use elasticsearch_dsl::{Distance, GeoPoint};
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::distance_feature("test", GeoPoint::coordinates(12.0, 13.0), Distance::Kilometers(15))
///     .boost(1.5)
///     .name("test");
/// ```
/// Distance Feature is built to allow only valid origin and pivot values,
/// the following won't compile:
/// ```compile_fail
/// # use elasticsearch_dsl::Distance;
/// # use chrono::prelude::*;
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::distance_feature("test", Utc.ymd(2014, 7, 8).and_hms(9, 1, 0), Distance::Kilometers(15))
///     .boost(1.5)
///     .name("test");
/// ```
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-distance-feature-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DistanceFeatureQuery<O: Origin> {
    #[serde(rename = "distance_feature")]
    inner: Inner<O>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner<O: Origin> {
    field: String,

    origin: O,

    pivot: <O as Origin>::Pivot,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [DistanceFeatureQuery](DistanceFeatureQuery)
    ///
    /// - `field` - Name of the field used to calculate distances. This field must meet the following criteria:<br>
    ///   - Be a [`date`](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html),
    /// [`date_nanos`](https://www.elastic.co/guide/en/elasticsearch/reference/current/date_nanos.html) or
    /// [`geo_point`](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html) field
    ///   - Have an [index](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-index.html)
    /// mapping parameter value of `true`, which is the default
    ///   - Have an [`doc_values`](https://www.elastic.co/guide/en/elasticsearch/reference/current/doc-values.html)
    /// mapping parameter value of `true`, which is the default
    /// - `origin` - Date or point of origin used to calculate distances.<br>
    /// If the `field` value is a
    /// [`date`](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html) or
    /// [`date_nanos`](https://www.elastic.co/guide/en/elasticsearch/reference/current/date_nanos.html)
    /// field, the `origin` value must be a
    /// [date](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-daterange-aggregation.html#date-format-pattern).
    /// [Date Math](https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#date-math),
    /// such as `now-1h`, is supported.<br>
    /// If the `field` value is a
    /// [`geo_point`](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html)
    /// field, the `origin` value must be a geopoint.
    /// - `pivot` - Distance from the `origin` at which relevance scores receive half of the boost value.<br>
    /// If the field value is a
    /// [`date`](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html) or
    /// [`date_nanos`](https://www.elastic.co/guide/en/elasticsearch/reference/current/date_nanos.html)
    /// field, the `pivot` value must be a
    /// [`time unit`](https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#time-units)
    /// , such as `1h` or `10d`.<br>
    /// If the `field` value is a
    /// [`geo_point`](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html)
    /// field, the `pivot` value must be a
    /// [distance unit](https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#distance-units)
    /// , such as `1km` or `12m`.
    pub fn distance_feature<O: Origin>(
        field: impl Into<String>,
        origin: O,
        pivot: <O as Origin>::Pivot,
    ) -> DistanceFeatureQuery<O> {
        DistanceFeatureQuery {
            inner: Inner {
                field: field.into(),
                origin,
                pivot,
                boost: None,
                _name: None,
            },
        }
    }
}

impl<O: Origin> DistanceFeatureQuery<O> {
    add_boost_and_name!();
}

impl<O: Origin> ShouldSkip for DistanceFeatureQuery<O> {}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Query::distance_feature("test", Utc.ymd(2014, 7, 8).and_hms(9, 1, 0), Time::Days(7)),
            json!({
                "distance_feature": {
                    "field": "test",
                    "origin": "2014-07-08T09:01:00Z",
                    "pivot": "7d",
                }
            }),
        );

        assert_serialize(
            Query::distance_feature("test", Utc.ymd(2014, 7, 8).and_hms(9, 1, 0), Time::Days(7))
                .boost(1.5)
                .name("test"),
            json!({
                "distance_feature": {
                    "field": "test",
                    "origin": "2014-07-08T09:01:00Z",
                    "pivot": "7d",
                    "boost": 1.5,
                    "_name": "test",
                }
            }),
        );
        assert_serialize(
            Query::distance_feature(
                "test",
                GeoPoint::coordinates(12.0, 13.0),
                Distance::Kilometers(15),
            ),
            json!({
                "distance_feature": {
                    "field": "test",
                    "origin": [13.0, 12.0],
                    "pivot": "15km",
                }
            }),
        );

        assert_serialize(
            Query::distance_feature(
                "test",
                GeoPoint::coordinates(12.0, 13.0),
                Distance::Kilometers(15),
            )
            .boost(2)
            .name("test"),
            json!({
                "distance_feature": {
                    "field": "test",
                    "origin": [13.0, 12.0],
                    "pivot": "15km",
                    "boost": 2,
                    "_name": "test",
                }
            }),
        );
    }
}
