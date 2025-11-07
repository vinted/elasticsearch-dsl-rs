use crate::search::*;
use crate::util::*;
use chrono::{DateTime, Utc};

/// This multi-bucket aggregation is similar to the normal histogram, but it can only be used with date or date range
/// values. Because dates are represented internally in Elasticsearch as long values, it is possible, but not as
/// accurate, to use the normal histogram on dates as well. The main difference in the two APIs is that here the
/// interval can be specified using date/time expressions. Time-based data requires special support because time-based
/// intervals are not always a fixed length.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-datehistogram-aggregation.html>
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DateHistogramAggregation {
    date_histogram: DateHistogramAggregationInner,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    aggs: Aggregations,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct DateHistogramAggregationInner {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    calendar_interval: Option<CalendarInterval>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fixed_interval: Option<Time>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    min_doc_count: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    offset: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    format: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    time_zone: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    order: TermsOrderCollection,
}

impl Aggregation {
    /// Creates an instance of [`DateHistogramAggregation`]
    ///
    /// - `field` - field to group by
    pub fn date_histogram<T>(field: T) -> DateHistogramAggregation
    where
        T: ToString,
    {
        DateHistogramAggregation {
            date_histogram: DateHistogramAggregationInner {
                field: field.to_string(),
                calendar_interval: None,
                fixed_interval: None,
                min_doc_count: None,
                missing: None,
                offset: None,
                format: None,
                time_zone: None,
                order: Default::default(),
            },
            aggs: Aggregations::new(),
        }
    }
}

impl DateHistogramAggregation {
    /// Calendar-aware intervals are configured with the calendar_interval parameter
    pub fn calendar_interval(mut self, calendar_interval: CalendarInterval) -> Self {
        self.date_histogram.calendar_interval = Some(calendar_interval);
        self
    }

    /// In contrast to calendar-aware intervals, fixed intervals are a fixed number of SI units and never deviate,
    /// regardless of where they fall on the calendar. One second is always composed of 1000ms. This allows fixed
    /// intervals to be specified in any multiple of the supported units.
    pub fn fixed_interval(mut self, fixed_interval: Time) -> Self {
        self.date_histogram.fixed_interval = Some(fixed_interval);
        self
    }

    /// Only returns terms that match more than a configured number of hits using the `min_doc_count`
    ///
    /// Default value is `1`
    pub fn min_doc_count(mut self, min_doc_count: u32) -> Self {
        self.date_histogram.min_doc_count = Some(min_doc_count);
        self
    }

    /// The missing parameter defines how documents that are missing a value should be treated.
    /// By default they will be ignored but it is also possible to treat them as if they had a value.
    pub fn missing(mut self, missing: DateTime<Utc>) -> Self {
        self.date_histogram.missing = Some(missing);
        self
    }

    /// Use the offset parameter to change the start value of each bucket by the specified positive (+) or negative
    /// offset (-) duration, such as 1h for an hour, or 1d for a day. See Time units for more possible time duration
    /// options.
    pub fn offset<T>(mut self, offset: T) -> Self
    where
        T: ToString,
    {
        self.date_histogram.offset = Some(offset.to_string());
        self
    }

    /// Sets the format for the date keys returned in the aggregation response.
    ///
    /// The `key` for each bucket is returned as a millisecond-since-the-epoch string.
    /// The `format` parameter can be used to convert this key into a formatted date string
    /// using the same date format patterns as the `date` field mapping.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-datehistogram-aggregation.html#datehistogram-aggregation-keys>
    pub fn format<T>(mut self, format: T) -> Self
    where
        T: ToString,
    {
        self.date_histogram.format = Some(format.to_string());
        self
    }

    /// Elasticsearch stores date-times in Coordinated Universal Time (UTC). By default, all bucketing and rounding is
    /// also done in UTC. Use the time_zone parameter to indicate that bucketing should use a different time zone.
    pub fn time_zone<T>(mut self, time_zone: T) -> Self
    where
        T: ToString,
    {
        self.date_histogram.time_zone = Some(time_zone.to_string());
        self
    }

    /// The order of the buckets can be customized by setting the order parameter.
    /// By default, the buckets are ordered by their doc_count descending.
    /// Order field allows changing this behavior.
    ///
    /// > Sorting by ascending `_count` or by sub aggregation is discouraged as it increases the
    /// > [error](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-terms-aggregation.html#search-aggregations-bucket-terms-aggregation-approximate-counts)
    /// > on document counts. It is fine when a single shard is queried, or when the field that is
    /// > being aggregated was used as a routing key at index time: in these cases results will be
    /// > accurate since shards have disjoint values. However otherwise, errors are unbounded.
    /// > One particular case that could still be useful is sorting by
    /// > [min](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-min-aggregation.html) or
    /// > [max](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-max-aggregation.html)
    /// > aggregation: counts will not be accurate but at least the top buckets will be correctly picked.
    pub fn order<T>(mut self, order: T) -> Self
    where
        T: Into<TermsOrderCollection>,
    {
        self.date_histogram.order = order.into();
        self
    }

    add_aggregate!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn serialization() {
        assert_serialize_aggregation(
            Aggregation::date_histogram("test_field"),
            json!({ "date_histogram": { "field": "test_field" } }),
        );

        assert_serialize_aggregation(
            Aggregation::date_histogram("test_field")
                .calendar_interval(CalendarInterval::Day)
                .fixed_interval(Time::Hours(1))
                .min_doc_count(2)
                .missing(
                    Utc.with_ymd_and_hms(2014, 11, 28, 12, 0, 4)
                        .single()
                        .unwrap(),
                )
                .order(TermsOrder::new("test_order", SortOrder::Asc))
                .offset("+6h")
                .format("yyyy-MM-dd")
                .time_zone("-01:00"),
            json!({
                "date_histogram": {
                    "field": "test_field",
                    "calendar_interval": "day",
                    "fixed_interval": "1h",
                    "min_doc_count": 2,
                    "missing": "2014-11-28T12:00:04Z",
                    "order": [
                        { "test_order": "asc" }
                    ],
                    "offset": "+6h",
                    "format": "yyyy-MM-dd",
                    "time_zone": "-01:00"
                }
            }),
        );

        assert_serialize_aggregation(
            Aggregation::date_histogram("test_field")
                .aggregate("test_sub_agg", Aggregation::terms("test_field2")),
            json!({
                "date_histogram": {
                    "field": "test_field",
                },
                "aggs": {
                    "test_sub_agg": {
                        "terms": {
                            "field": "test_field2"
                        }
                    }
                }
            }),
        );
    }
}
