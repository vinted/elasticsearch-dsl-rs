use crate::search::*;
use crate::util::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};

/// Returns documents that contain terms within a provided range.
///
/// To create a range query with numeric values:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::range("numeric_field")
///     .gt(1)
///     .lt(3)
///     .boost(2)
///     .name("range_query");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-range-query.html>
#[derive(Debug, Clone, PartialEq)]
pub struct RangeQuery {
    field: String,
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    gt: OptionalScalar,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    gte: OptionalScalar,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    lt: OptionalScalar,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    lte: OptionalScalar,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    format: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    relation: Option<RangeRelation>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    time_zone: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`RangeQuery`]
    ///
    /// - `field` - Field you wish to search.
    pub fn range(field: impl Into<String>) -> RangeQuery {
        RangeQuery {
            field: field.into(),
            inner: Inner {
                gt: Default::default(),
                gte: Default::default(),
                lt: Default::default(),
                lte: Default::default(),
                format: None,
                relation: None,
                time_zone: None,
                boost: None,
                _name: None,
            },
        }
    }
}

impl RangeQuery {
    /// Greater than.
    pub fn gt(mut self, gt: impl Into<OptionalScalar>) -> Self {
        self.inner.gt = gt.into();
        self
    }

    /// Greater than or equal to.
    pub fn gte(mut self, gte: impl Into<OptionalScalar>) -> Self {
        self.inner.gte = gte.into();
        self
    }

    /// Less than.
    pub fn lt(mut self, lt: impl Into<OptionalScalar>) -> Self {
        self.inner.lt = lt.into();
        self
    }

    /// Less than or equal to.
    pub fn lte(mut self, lte: impl Into<OptionalScalar>) -> Self {
        self.inner.lte = lte.into();
        self
    }

    /// Date format used to convert `date` values in the query.
    ///
    /// By default, Elasticsearch uses the
    /// [date `format`](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-date-format.html)
    /// provided in the `<field>`'s mapping. This value overrides that mapping format.
    ///
    /// For valid syntax, see
    /// [`format`](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-date-format.html).
    ///
    /// >If a format or date value is incomplete, the range query replaces
    /// any missing components with default values. See
    /// [Missing date components](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-range-query.html#missing-date-components).
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.inner.format = Some(format.into());
        self
    }

    /// Indicates how the range query matches values for range fields.
    pub fn relation(mut self, relation: RangeRelation) -> Self {
        self.inner.relation = Some(relation);
        self
    }

    /// [Coordinated Universal Time (UTC) offset](https://en.wikipedia.org/wiki/List_of_UTC_time_offsets)
    /// or [IANA time zone](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)
    /// used to convert `date` values in the query to UTC.
    ///
    /// Valid values are ISO 8601 UTC offsets, such as `+01:00` or `-08:00`, and IANA time zone IDs,
    /// such as `America/Los_Angeles`.
    ///
    /// For an example query using the `time_zone` parameter, see
    /// [Time zone in `range` queries](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-range-query.html#range-query-time-zone).
    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.inner.time_zone = Some(time_zone.into());
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for RangeQuery {
    fn should_skip(&self) -> bool {
        self.inner.gt.should_skip()
            && self.inner.gte.should_skip()
            && self.inner.lt.should_skip()
            && self.inner.lte.should_skip()
    }
}

impl Serialize for RangeQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hash = std::collections::HashMap::new();
        let _ = hash.insert(&self.field, &self.inner);

        let mut map = serializer.serialize_struct("RangeQuery", 1)?;
        map.serialize_field("range", &hash)?;
        map.end()
    }
}

#[cfg(test)]
#[allow(unused_qualifications)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    test_serialization! {
        with_required_fields(
            Query::range("test_field"),
            json!({
                "range": {
                    "test_field": {}
                }
            })
        );

        with_none_fields(
            Query::range("test_field")
                .gt(Option::<i32>::None)
                .lt(Option::<i32>::None)
                .gte(Option::<i32>::None)
                .lte(Option::<i32>::None),
            json!({
                "range": {
                    "test_field": {}
                }
            })
        );

        with_all_numeric_fields(
            Query::range("test_numeric_field")
                .gt(1)
                .gte(2)
                .lt(3)
                .lte(4)
                .relation(RangeRelation::Within)
                .boost(2)
                .name("range_query_test"),
            json!({
                "range": {
                    "test_numeric_field": {
                        "gt": 1,
                        "gte": 2,
                        "lt": 3,
                        "lte": 4,
                        "relation": "WITHIN",
                        "boost": 2.0,
                        "_name": "range_query_test"
                    }
                }
            })
        );

        with_all_date_fields(
            Query::range("test_date_field")
                .gt(Utc.ymd(2014, 11, 28).and_hms(12, 0, 1))
                .gte(Utc.ymd(2014, 11, 28).and_hms(12, 0, 2))
                .lt(Utc.ymd(2014, 11, 28).and_hms(12, 0, 3))
                .lte(Utc.ymd(2014, 11, 28).and_hms(12, 0, 4))
                .relation(RangeRelation::Contains)
                .format("yyyy-MM-dd")
                .time_zone("UTC")
                .boost(2)
                .name("range_query_test"),
            json!({
                "range": {
                    "test_date_field": {
                        "gt": "2014-11-28T12:00:01Z",
                        "gte": "2014-11-28T12:00:02Z",
                        "lt": "2014-11-28T12:00:03Z",
                        "lte": "2014-11-28T12:00:04Z",
                        "format": "yyyy-MM-dd",
                        "time_zone": "UTC",
                        "relation": "CONTAINS",
                        "boost": 2.0,
                        "_name": "range_query_test"
                    }
                }
            })
        );
    }
}
