use crate::search::*;
use crate::util::*;
use serde::{Serialize, Serializer };
use serde_json::Value;

/// A multi-bucket aggregation that creates composite buckets from different sources.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-bucket-composite-aggregation.html>
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CompositeAggregation {
    composite: CompositeAggregationInner,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    aggs: Aggregations,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct CompositeAggregationInner {
    sources: Vec<CompositeSource>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    size: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    after: Option<AfterKey>,
}

impl Aggregation {
    /// Creates an instance of [`CompositeAggregation`]
    ///
    /// - `sources` - A vector of `CompositeSource` which defines the sources for the composite aggregation.
    pub fn composite(sources: Vec<CompositeSource>) -> CompositeAggregation {
        CompositeAggregation {
            composite: CompositeAggregationInner {
                sources,
                size: None,
                after: None,
            },
            aggs: Aggregations::new(),
        }
    }
}

impl CompositeAggregation {
    /// The `size` parameter can be set to define how many composite buckets should be returned.
    ///
    /// - `size` - The maximum number of composite buckets to be returned.
    pub fn size(mut self, size: u64) -> Self {
        self.composite.size = Some(size);
        self
    }

    /// The `after` parameter can be set to paginate composite buckets.
    ///
    /// - `after` - The key to start after for pagination in composite aggregations.
    pub fn after<T>(mut self, after: T) -> Self
    where
        T: Into<AfterKey>,
    {
        self.composite.after = Some(after.into());
        self
    }

    add_aggregate!();
}

/// Represents the `after` key for pagination in composite aggregations.
///
/// The `AfterKey` is used to paginate through the composite aggregation results.
/// It is typically a JSON object containing the values of the composite keys.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct AfterKey(Value);

impl From<Value> for AfterKey {
    fn from(value: Value) -> Self {
        AfterKey(value)
    }
}

impl AfterKey {
    /// Creates a new `AfterKey` instance from a JSON value.
    ///
    /// - `value` - The JSON value representing the `after` key.
    pub fn new(value: Value) -> Self {
        AfterKey(value)
    }
}

/// Represents different types of sources for a composite aggregation.
#[derive(Debug, Clone, PartialEq)]
pub enum CompositeSource {
    /// Terms source for the composite aggregation.
    Terms {
        /// The unique identifier for the terms source.
        name: String,
        /// The terms composite source.
        terms: TermsCompositeSource,
    },
    /// Histogram source for the composite aggregation.
    Histogram {
        /// The unique identifier for the histogram source.
        name: String,
        /// The histogram composite source.
        histogram: HistogramCompositeSource,
    },
    /// Date histogram source for the composite aggregation.
    DateHistogram {
        /// The unique identifier for the date histogram source.
        name: String,
        /// The date histogram composite source.
        date_histogram: DateHistogramCompositeSource,
    },
}

impl Serialize for CompositeSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serde_json::Map::new();
        match self {
            CompositeSource::Terms { name, terms } => {
                let _ = map.insert(name.clone(), serde_json::json!({ "terms": terms }));
            }
            CompositeSource::Histogram { name, histogram } => {
                let _ = map.insert(name.clone(), serde_json::json!({ "histogram": histogram }));
            }
            CompositeSource::DateHistogram { name, date_histogram } => {
                let _ = map.insert(name.clone(), serde_json::json!({ "date_histogram": date_histogram }));
            }
        }
        map.serialize(serializer)
    }
}

/// Represents a terms source in a composite aggregation.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TermsCompositeSource {
    field: String,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing_bucket: Option<bool>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    order: Option<SortOrder>,
}

/// Represents a histogram source in a composite aggregation.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct HistogramCompositeSource {
    field: String,
    interval: f64,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing_bucket: Option<bool>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    order: Option<SortOrder>,
}

/// Represents a date histogram source in a composite aggregation.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DateHistogramCompositeSource {
    field: String,
    calendar_interval: String,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    missing_bucket: Option<bool>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    order: Option<SortOrder>,
}

impl CompositeSource {
    /// Creates a terms source for the composite aggregation.
    ///
    /// - `name` - The unique identifier for the terms source.
    /// - `field` - The field to perform the terms aggregation on.
    pub fn terms(name: &str, field: &str) -> CompositeSource {
        CompositeSource::Terms {
            name: name.to_string(),
            terms: TermsCompositeSource {
                field: field.to_string(),
                missing_bucket: None,
                order: None,
            },
        }
    }

    /// Creates a histogram source for the composite aggregation.
    ///
    /// - `name` - The unique identifier for the histogram source.
    /// - `field` - The field to perform the histogram aggregation on.
    /// - `interval` - The interval for the histogram buckets.
    pub fn histogram(name: &str, field: &str, interval: f64) -> CompositeSource {
        CompositeSource::Histogram {
            name: name.to_string(),
            histogram: HistogramCompositeSource {
                field: field.to_string(),
                interval,
                missing_bucket: None,
                order: None,
            },
        }
    }

    /// Creates a date histogram source for the composite aggregation.
    ///
    /// - `name` - The unique identifier for the date histogram source.
    /// - `field` - The field to perform the date histogram aggregation on.
    /// - `calendar_interval` - The calendar interval for the date histogram buckets.
    pub fn date_histogram(name: &str, field: &str, calendar_interval: &str) -> CompositeSource {
        CompositeSource::DateHistogram {
            name: name.to_string(),
            date_histogram: DateHistogramCompositeSource {
                field: field.to_string(),
                calendar_interval: calendar_interval.to_string(),
                missing_bucket: None,
                order: None,
            },
        }
    }
}

impl TermsCompositeSource {
    /// Sets the `missing_bucket` parameter for the terms source.
    ///
    /// - `missing_bucket` - Whether to include documents with missing values in the bucket.
    pub fn missing_bucket(mut self, missing_bucket: bool) -> Self {
        self.missing_bucket = Some(missing_bucket);
        self
    }

    /// Sets the `order` parameter for the terms source.
    ///
    /// - `order` - The order of the terms in the bucket.
    pub fn order(mut self, order: SortOrder) -> Self {
        self.order = Some(order);
        self
    }
}

impl HistogramCompositeSource {
    /// Sets the `missing_bucket` parameter for the histogram source.
    ///
    /// - `missing_bucket` - Whether to include documents with missing values in the bucket.
    pub fn missing_bucket(mut self, missing_bucket: bool) -> Self {
        self.missing_bucket = Some(missing_bucket);
        self
    }

    /// Sets the `order` parameter for the histogram source.
    ///
    /// - `order` - The order of the histogram buckets.
    pub fn order(mut self, order: SortOrder) -> Self {
        self.order = Some(order);
        self
    }
}

impl DateHistogramCompositeSource {
    /// Sets the `missing_bucket` parameter for the date histogram source.
    ///
    /// - `missing_bucket` - Whether to include documents with missing values in the bucket.
    pub fn missing_bucket(mut self, missing_bucket: bool) -> Self {
        self.missing_bucket = Some(missing_bucket);
        self
    }

    /// Sets the `order` parameter for the date histogram source.
    ///
    /// - `order` - The order of the date histogram buckets.
    pub fn order(mut self, order: SortOrder) -> Self {
        self.order = Some(order);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_aggregation(
            Aggregation::composite(vec![CompositeSource::terms("test_field", "field_name")]),
            json!({ "composite": { "sources": [{ "test_field": { "terms": { "field": "field_name" } } }] } }),
        );

        assert_serialize_aggregation(
            Aggregation::composite(vec![CompositeSource::terms("test_field", "field_name")])
                .size(10)
                .after(serde_json::json!({"test_field": "after_key"})),
            json!({
                "composite": {
                    "sources": [{ "test_field": { "terms": { "field": "field_name" } } }],
                    "size": 10,
                    "after": { "test_field": "after_key" }
                }
            }),
        );

        assert_serialize_aggregation(
            Aggregation::composite(vec![CompositeSource::terms("test_field", "field_name")]),
            json!({
                "composite": {
                    "sources": [{ "test_field": { "terms": { "field": "field_name" } } }],
                }
            }),
        );
    }
}
