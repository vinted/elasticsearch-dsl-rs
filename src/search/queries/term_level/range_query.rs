use crate::search::*;
use crate::util::*;
use serde::Serialize;

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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct RangeQuery {
    #[serde(skip)]
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    gt: Option<Term>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    gte: Option<Term>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    lt: Option<Term>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    lte: Option<Term>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    format: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    relation: Option<RangeRelation>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    time_zone: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`RangeQuery`]
    ///
    /// - `field` - Field you wish to search.
    pub fn range<T>(field: T) -> RangeQuery
    where
        T: ToString,
    {
        RangeQuery {
            field: field.to_string(),
            gt: Default::default(),
            gte: Default::default(),
            lt: Default::default(),
            lte: Default::default(),
            format: None,
            relation: None,
            time_zone: None,
            boost: None,
            _name: None,
        }
    }
}

impl RangeQuery {
    /// Greater than.
    pub fn gt<T>(mut self, gt: T) -> Self
    where
        T: Serialize,
    {
        self.gt = Term::new(gt);
        self
    }

    /// Greater than or equal to.
    pub fn gte<T>(mut self, gte: T) -> Self
    where
        T: Serialize,
    {
        self.gte = Term::new(gte);
        self
    }

    /// Less than.
    pub fn lt<T>(mut self, lt: T) -> Self
    where
        T: Serialize,
    {
        self.lt = Term::new(lt);
        self
    }

    /// Less than or equal to.
    pub fn lte<T>(mut self, lte: T) -> Self
    where
        T: Serialize,
    {
        self.lte = Term::new(lte);
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
    pub fn format<T>(mut self, format: T) -> Self
    where
        T: ToString,
    {
        self.format = Some(format.to_string());
        self
    }

    /// Indicates how the range query matches values for range fields.
    pub fn relation(mut self, relation: RangeRelation) -> Self {
        self.relation = Some(relation);
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
    pub fn time_zone<T>(mut self, time_zone: T) -> Self
    where
        T: ToString,
    {
        self.time_zone = Some(time_zone.to_string());
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for RangeQuery {
    fn should_skip(&self) -> bool {
        self.gt.should_skip()
            && self.gte.should_skip()
            && self.lt.should_skip()
            && self.lte.should_skip()
    }
}

serialize_with_root_keyed!("range": RangeQuery);

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::range("test_field"),
            json!({
                "range": {
                    "test_field": {}
                }
            }),
        );

        assert_serialize_query(
            Query::range("test_field")
                .gt(None::<i32>)
                .lt(None::<i32>)
                .gte(None::<i32>)
                .lte(None::<i32>),
            json!({
                "range": {
                    "test_field": {}
                }
            }),
        );

        assert_serialize_query(
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
            }),
        );

        assert_serialize_query(
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
            }),
        );
    }
}
