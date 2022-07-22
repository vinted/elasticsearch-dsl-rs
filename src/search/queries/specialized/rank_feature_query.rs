use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// Boosts the relevance score of documents based on the numeric value of a `rank_feature` or
/// `rank_features` field.
///
/// To create a rank feature query:
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
#[serde(remote = "Self")]
pub struct RankFeatureQuery {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

/// Boosts the relevance score of documents based on the numeric value of a `rank_feature` or
/// `rank_features` field.
///
/// To create a rank feature query:
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
#[serde(remote = "Self")]
pub struct RankFeatureSaturationQuery {
    field: String,

    saturation: Saturation,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

/// Boosts the relevance score of documents based on the numeric value of a `rank_feature` or
/// `rank_features` field.
///
/// To create a rank feature query:
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
#[serde(remote = "Self")]
pub struct RankFeatureLogarithmQuery {
    field: String,

    log: Logarithm,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

/// Boosts the relevance score of documents based on the numeric value of a `rank_feature` or
/// `rank_features` field.
///
/// To create a rank feature query:
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
#[serde(remote = "Self")]
pub struct RankFeatureSigmoidQuery {
    field: String,

    sigmoid: Sigmoid,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

/// Boosts the relevance score of documents based on the numeric value of a `rank_feature` or
/// `rank_features` field.
///
/// To create a rank feature query:
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
#[serde(remote = "Self")]
pub struct RankFeatureLinearQuery {
    field: String,

    linear: Linear,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
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

impl Query {
    /// Creates an instance of [`RankFeatureQuery`]
    ///
    /// - `field` - `rank_feature` or `rank_features` field used to boost relevance scores
    pub fn rank_feature<T>(field: T) -> RankFeatureQuery
    where
        T: ToString,
    {
        RankFeatureQuery {
            field: field.to_string(),
            boost: None,
            _name: None,
        }
    }
}

impl RankFeatureQuery {
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
            field: self.field,
            boost: self.boost,
            _name: self._name,
            saturation: Saturation { pivot: None },
        }
    }

    /// The `log` function gives a score equal to `log(scaling_factor + S)`, where `S` is the value
    /// of the rank feature field and `scaling_factor` is a configurable scaling factor.
    /// Scores are unbounded.
    ///
    /// This function only supports rank features that have a positive score impact.
    pub fn logarithm(self, scaling_factor: f64) -> RankFeatureLogarithmQuery {
        RankFeatureLogarithmQuery {
            field: self.field,
            boost: self.boost,
            _name: self._name,
            log: Logarithm { scaling_factor },
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
            field: self.field,
            boost: self.boost,
            _name: self._name,
            sigmoid: Sigmoid { pivot, exponent },
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
            field: self.field,
            boost: self.boost,
            _name: self._name,
            linear: Linear {},
        }
    }

    add_boost_and_name!();
}

impl RankFeatureSaturationQuery {
    /// Sets pivot value
    pub fn pivot<T>(mut self, pivot: T) -> Self
    where
        T: Into<f64>,
    {
        self.saturation.pivot = Some(pivot.into());
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

serialize_with_root!("rank_feature": RankFeatureQuery);
serialize_with_root!("rank_feature": RankFeatureSaturationQuery);
serialize_with_root!("rank_feature": RankFeatureLogarithmQuery);
serialize_with_root!("rank_feature": RankFeatureSigmoidQuery);
serialize_with_root!("rank_feature": RankFeatureLinearQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::rank_feature("test"),
            json!({
                "rank_feature": {
                    "field": "test",
                }
            }),
        );

        assert_serialize_query(
            Query::rank_feature("test").boost(2).name("query"),
            json!({
                "rank_feature": {
                    "field": "test",
                    "boost": 2.0,
                    "_name": "query",
                }
            }),
        );

        assert_serialize_query(
            Query::rank_feature("test")
                .saturation()
                .boost(2)
                .name("query"),
            json!({
                "rank_feature": {
                    "field": "test",
                    "boost": 2.0,
                    "_name": "query",
                    "saturation": {},
                }
            }),
        );

        assert_serialize_query(
            Query::rank_feature("test")
                .saturation()
                .pivot(2.2)
                .boost(2)
                .name("query"),
            json!({
                "rank_feature": {
                    "field": "test",
                    "boost": 2.0,
                    "_name": "query",
                    "saturation": {
                        "pivot": 2.2,
                    },
                }
            }),
        );

        assert_serialize_query(
            Query::rank_feature("test")
                .logarithm(2.2)
                .boost(2)
                .name("query"),
            json!({
                "rank_feature": {
                    "field": "test",
                    "boost": 2.0,
                    "_name": "query",
                    "log": {
                        "scaling_factor": 2.2
                    },
                }
            }),
        );

        assert_serialize_query(
            Query::rank_feature("test")
                .sigmoid(2.2, 3.3)
                .boost(2)
                .name("query"),
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
            }),
        );

        assert_serialize_query(
            Query::rank_feature("test").linear().boost(2).name("query"),
            json!({
                "rank_feature": {
                    "field": "test",
                    "boost": 2.0,
                    "_name": "query",
                    "linear": {},
                }
            }),
        );
    }
}
