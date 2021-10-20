use crate::{search::*, util::*};
use serde::Serialize;

/// Boosts the relevance score of documents based on the numeric value of a `rank_feature` or
/// `rank_features` field.
///
/// To create a rank feature query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// RankFeatureQuery::new("test");
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::rank_feature("test");
/// ```
/// To apply mathematical functions:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::rank_feature("test").saturation();
/// # let query =
/// Query::rank_feature("test").saturation().pivot(2.2);
/// # let query =
/// Query::rank_feature("test").logarithm(3.0);
/// # let query =
/// Query::rank_feature("test").sigmoid(1.0, 2.0);
/// # let query =
/// Query::rank_feature("test").linear();
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-rank-feature-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RankFeatureQuery {
    #[serde(rename = "rank_feature")]
    inner: Inner,
}

/// Boosts the relevance score of documents based on the numeric value of a `rank_feature` or
/// `rank_features` field.
///
/// To create a rank feature query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// RankFeatureQuery::new("test");
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::rank_feature("test");
/// ```
/// To apply mathematical functions:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::rank_feature("test").saturation();
/// # let query =
/// Query::rank_feature("test").saturation().pivot(2.2);
/// # let query =
/// Query::rank_feature("test").logarithm(3.0);
/// # let query =
/// Query::rank_feature("test").sigmoid(1.0, 2.0);
/// # let query =
/// Query::rank_feature("test").linear();
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-rank-feature-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RankFeatureSaturationQuery {
    #[serde(rename = "rank_feature")]
    inner: InnerSaturation,
}

/// Boosts the relevance score of documents based on the numeric value of a `rank_feature` or
/// `rank_features` field.
///
/// To create a rank feature query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// RankFeatureQuery::new("test");
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::rank_feature("test");
/// ```
/// To apply mathematical functions:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::rank_feature("test").saturation();
/// # let query =
/// Query::rank_feature("test").saturation().pivot(2.2);
/// # let query =
/// Query::rank_feature("test").logarithm(3.0);
/// # let query =
/// Query::rank_feature("test").sigmoid(1.0, 2.0);
/// # let query =
/// Query::rank_feature("test").linear();
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-rank-feature-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RankFeatureLogarithmQuery {
    #[serde(rename = "rank_feature")]
    inner: InnerLogarithm,
}

/// Boosts the relevance score of documents based on the numeric value of a `rank_feature` or
/// `rank_features` field.
///
/// To create a rank feature query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// RankFeatureQuery::new("test");
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::rank_feature("test");
/// ```
/// To apply mathematical functions:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::rank_feature("test").saturation();
/// # let query =
/// Query::rank_feature("test").saturation().pivot(2.2);
/// # let query =
/// Query::rank_feature("test").logarithm(3.0);
/// # let query =
/// Query::rank_feature("test").sigmoid(1.0, 2.0);
/// # let query =
/// Query::rank_feature("test").linear();
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-rank-feature-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RankFeatureSigmoidQuery {
    #[serde(rename = "rank_feature")]
    inner: InnerSigmoid,
}

/// Boosts the relevance score of documents based on the numeric value of a `rank_feature` or
/// `rank_features` field.
///
/// To create a rank feature query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// RankFeatureQuery::new("test");
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::rank_feature("test");
/// ```
/// To apply mathematical functions:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::rank_feature("test").saturation();
/// # let query =
/// Query::rank_feature("test").saturation().pivot(2.2);
/// # let query =
/// Query::rank_feature("test").logarithm(3.0);
/// # let query =
/// Query::rank_feature("test").sigmoid(1.0, 2.0);
/// # let query =
/// Query::rank_feature("test").linear();
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-rank-feature-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RankFeatureLinearQuery {
    #[serde(rename = "rank_feature")]
    inner: InnerLinear,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Saturation {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pivot: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Logarithm {
    scaling_factor: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Sigmoid {
    pivot: f64,
    exponent: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Linear {}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct InnerSaturation {
    field: String,

    saturation: Saturation,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct InnerLogarithm {
    field: String,

    log: Logarithm,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct InnerSigmoid {
    field: String,

    sigmoid: Sigmoid,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct InnerLinear {
    field: String,

    linear: Linear,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`RankFeatureQuery`]
    ///
    /// - `field` - `rank_feature` or `rank_features` field used to boost relevance scores
    pub fn rank_feature(field: impl Into<String>) -> RankFeatureQuery {
        RankFeatureQuery::new(field)
    }
}

impl RankFeatureQuery {
    /// Creates an instance of [`RankFeatureQuery`]
    ///
    /// - `field` - `rank_feature` or `rank_features` field used to boost relevance scores
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            inner: Inner {
                field: field.into(),
                boost: None,
                _name: None,
            },
        }
    }

    /// The `saturation` function gives a score equal to `S / (S + pivot)`, where `S` is the value
    /// of the rank feature field and `pivot` is a configurable pivot value so that the result will
    /// be less than `0.5` if `S` is less than pivot and greater than `0.5` otherwise.
    /// Scores are always `(0,1)`.
    ///
    /// If the rank feature has a negative score impact then the function will be computed as
    /// `pivot / (S + pivot)`, which decreases when `S` increases.
    ///
    /// If a `pivot` value is not provided, Elasticsearch computes a default value equal to the
    /// approximate geometric mean of all rank feature values in the index. We recommend using this
    /// default value if you haven’t had the opportunity to train a good pivot value.
    pub fn saturation(self) -> RankFeatureSaturationQuery {
        RankFeatureSaturationQuery {
            inner: InnerSaturation {
                field: self.inner.field,
                boost: self.inner.boost,
                _name: self.inner._name,
                saturation: Saturation { pivot: None },
            },
        }
    }

    /// The `log` function gives a score equal to `log(scaling_factor + S)`, where `S` is the value
    /// of the rank feature field and `scaling_factor` is a configurable scaling factor.
    /// Scores are unbounded.
    ///
    /// This function only supports rank features that have a positive score impact.
    pub fn logarithm(self, scaling_factor: f64) -> RankFeatureLogarithmQuery {
        RankFeatureLogarithmQuery {
            inner: InnerLogarithm {
                field: self.inner.field,
                boost: self.inner.boost,
                _name: self.inner._name,
                log: Logarithm { scaling_factor },
            },
        }
    }

    /// The `sigmoid` function is an extension of `saturation` which adds a configurable exponent.
    /// Scores are computed as `S^exp^ / (S^exp^ + pivot^exp^)`. Like for the `saturation` function,
    /// `pivot` is the value of `S` that gives a score of `0.5` and scores are `(0,1)`.
    ///
    /// The `exponent` must be positive and is typically in `[0.5, 1]`. A good value should be
    /// computed via training. If you don’t have the opportunity to do so, we recommend you use the
    /// `saturation` function instead.
    pub fn sigmoid(self, pivot: f64, exponent: f64) -> RankFeatureSigmoidQuery {
        RankFeatureSigmoidQuery {
            inner: InnerSigmoid {
                field: self.inner.field,
                boost: self.inner.boost,
                _name: self.inner._name,
                sigmoid: Sigmoid { pivot, exponent },
            },
        }
    }

    /// The `linear` function is the simplest function, and gives a score equal to the indexed
    /// value of `S`, where `S` is the value of the rank feature field. If a rank feature field is
    /// indexed with `"positive_score_impact": true`, its indexed value is equal to `S` and rounded
    /// to preserve only 9 significant bits for the precision. If a rank feature field is indexed
    /// with `"positive_score_impact": false`, its indexed value is equal to `1/S` and rounded to
    /// preserve only 9 significant bits for the precision.
    pub fn linear(self) -> RankFeatureLinearQuery {
        RankFeatureLinearQuery {
            inner: InnerLinear {
                field: self.inner.field,
                boost: self.inner.boost,
                _name: self.inner._name,
                linear: Linear {},
            },
        }
    }

    add_boost_and_name!();
}

impl RankFeatureSaturationQuery {
    /// Sets pivot value
    pub fn pivot(mut self, pivot: impl Into<Option<f64>>) -> Self {
        self.inner.saturation.pivot = pivot.into();
        self
    }

    add_boost_and_name!();
}

impl RankFeatureLogarithmQuery {
    add_boost_and_name!();
}

impl RankFeatureSigmoidQuery {
    add_boost_and_name!();
}

impl RankFeatureLinearQuery {
    add_boost_and_name!();
}

impl ShouldSkip for RankFeatureQuery {}
impl ShouldSkip for RankFeatureSaturationQuery {}
impl ShouldSkip for RankFeatureLogarithmQuery {}
impl ShouldSkip for RankFeatureSigmoidQuery {}
impl ShouldSkip for RankFeatureLinearQuery {}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            Query::rank_feature("test"),
            json!({
                "rank_feature": {
                    "field": "test",
                }
            })
        );

        with_all_fields(
            Query::rank_feature("test").boost(2).name("query"),
            json!({
                "rank_feature": {
                    "field": "test",
                    "boost": 2.0,
                    "_name": "query",
                }
            })
        );

        with_saturation(
            Query::rank_feature("test").saturation().boost(2).name("query"),
            json!({
                "rank_feature": {
                    "field": "test",
                    "boost": 2.0,
                    "_name": "query",
                    "saturation": {},
                }
            })
        );

        with_saturation_and_pivot_value(
            Query::rank_feature("test").saturation().pivot(2.2).boost(2).name("query"),
            json!({
                "rank_feature": {
                    "field": "test",
                    "boost": 2.0,
                    "_name": "query",
                    "saturation": {
                        "pivot": 2.2,
                    },
                }
            })
        );

        with_logarithm(
            Query::rank_feature("test").logarithm(2.2).boost(2).name("query"),
            json!({
                "rank_feature": {
                    "field": "test",
                    "boost": 2.0,
                    "_name": "query",
                    "log": {
                        "scaling_factor": 2.2
                    },
                }
            })
        );

        with_sigmoid(
            Query::rank_feature("test").sigmoid(2.2, 3.3).boost(2).name("query"),
            json!({
                "rank_feature": {
                    "field": "test",
                    "boost": 2.0,
                    "_name": "query",
                    "sigmoid": {
                        "pivot": 2.2,
                        "exponent": 3.3,
                    },
                }
            })
        );

        with_linear(
            Query::rank_feature("test").linear().boost(2).name("query"),
            json!({
                "rank_feature": {
                    "field": "test",
                    "boost": 2.0,
                    "_name": "query",
                    "linear": {},
                }
            })
        );
    }
}
