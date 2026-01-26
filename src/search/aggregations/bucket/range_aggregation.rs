use crate::search::*;
use crate::util::*;

/// A multi-bucket value source based aggregation that enables the user to define a set of ranges -
/// each representing a bucket. During the aggregation process, the values extracted from each
/// document will be checked against each bucket range and "bucket" the relevant/matching document.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-range-aggregation.html>
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RangeAggregation {
    range: RangeAggregationInner,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    aggs: Aggregations,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct RangeAggregationInner {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    keyed: Option<bool>,

    ranges: Vec<RangeBucket>,
}

/// A single range bucket definition with optional key, from, and to values.
#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct RangeBucket {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    key: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    from: Option<Number>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    to: Option<Number>,
}

impl RangeBucket {
    /// Creates a new range bucket with only an upper bound (exclusive)
    pub fn lt<T: Into<Number>>(to: T) -> Self {
        Self {
            to: Some(to.into()),
            ..Self::default()
        }
    }

    /// Creates a new range bucket with only a lower bound (inclusive)
    pub fn gte<T: Into<Number>>(from: T) -> Self {
        Self {
            from: Some(from.into()),
            ..Self::default()
        }
    }

    /// Creates a new range bucket with both lower (inclusive) and upper (exclusive) bounds
    pub fn between<T: Into<Number>>(from: T, to: T) -> Self {
        Self {
            from: Some(from.into()),
            to: Some(to.into()),
            ..Self::default()
        }
    }

    /// Sets the key for this range bucket
    pub fn key<T: ToString>(mut self, key: T) -> Self {
        self.key = Some(key.to_string());
        self
    }

    /// Sets the lower bound (inclusive) for this range bucket
    pub fn from<T: Into<Number>>(mut self, from: T) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Sets the upper bound (exclusive) for this range bucket
    pub fn to<T: Into<Number>>(mut self, to: T) -> Self {
        self.to = Some(to.into());
        self
    }
}

impl Aggregation {
    /// Creates an instance of [`RangeAggregation`]
    ///
    /// - `field` - field to aggregate on
    /// - `ranges` - range buckets. At least one [`RangeBucket`] is required, either
    ///   at the time of contruction or dynamically by using the [`RangeAggregation::range_bucket`]
    ///   method.
    pub fn range<T>(field: T, ranges: Vec<RangeBucket>) -> RangeAggregation
    where
        T: ToString,
    {
        RangeAggregation {
            range: RangeAggregationInner {
                field: field.to_string(),
                keyed: None,
                ranges,
            },
            aggs: Aggregations::new(),
        }
    }
}

impl RangeAggregation {
    /// Adds a range bucket to the aggregation
    pub fn range_bucket(mut self, bucket: RangeBucket) -> Self {
        self.range.ranges.push(bucket);
        self
    }

    /// Setting the `keyed` flag to `true` associates a unique string key with each bucket and
    /// returns the ranges as a hash rather than an array
    pub fn keyed(mut self, keyed: bool) -> Self {
        self.range.keyed = Some(keyed);
        self
    }

    add_aggregate!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_aggregation(
            Aggregation::range("price", vec![RangeBucket::lt(100)]),
            json!({ "range": { "field": "price", "ranges": [ { "to": 100} ] } }),
        );

        assert_serialize_aggregation(
            Aggregation::range(
                "price",
                vec![
                    RangeBucket::lt(-100),
                    RangeBucket::between(-100, 200),
                    RangeBucket::gte(200),
                ],
            ),
            json!({
                "range": {
                    "field": "price",
                    "ranges": [
                        { "to": -100 },
                        { "from": -100, "to": 200 },
                        { "from": 200 }
                    ]
                }
            }),
        );

        assert_serialize_aggregation(
            Aggregation::range(
                "price",
                vec![
                    RangeBucket::lt(100.0).key("cheap"),
                    RangeBucket::between(100.0, 200.0).key("average"),
                    RangeBucket::gte(200.0).key("expensive"),
                ],
            )
            .keyed(true),
            json!({
                "range": {
                    "field": "price",
                    "keyed": true,
                    "ranges": [
                        { "key": "cheap", "to": 100.0 },
                        { "key": "average", "from": 100.0, "to": 200.0 },
                        { "key": "expensive", "from": 200.0 }
                    ]
                }
            }),
        );

        assert_serialize_aggregation(
            Aggregation::range(
                "timestamp",
                vec![
                    RangeBucket::between(0, 1000).key("baseline"),
                    RangeBucket::between(1000, 2000).key("current"),
                ],
            )
            .keyed(true)
            .aggregate("avg_value", Aggregation::avg("value")),
            json!({
                "range": {
                    "field": "timestamp",
                    "keyed": true,
                    "ranges": [
                        { "key": "baseline", "from": 0, "to": 1000},
                        { "key": "current", "from": 1000, "to": 2000 }
                    ]
                },
                "aggs": {
                    "avg_value": {
                        "avg": { "field": "value" }
                    }
                }
            }),
        );
    }
}
