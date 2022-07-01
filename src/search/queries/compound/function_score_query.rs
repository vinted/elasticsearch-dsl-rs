use crate::search::*;
use crate::util::*;

/// The `function_score` allows you to modify the score of documents that are retrieved by a query.
///
/// This can be useful if, for example, a score function is computationally expensive and it is
/// sufficient to compute the score on a filtered set of documents.
///
/// To use `function_score`, the user has to define a query and one or more functions, that compute
/// a new score for each document returned by the query.
///
/// To create function_score query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::function_score(Query::term("test", 1))
///     .function(RandomScore::new())
///     .function(Weight::new(2.0))
///     .max_boost(2.2)
///     .min_score(2.3)
///     .score_mode(FunctionScoreMode::Avg)
///     .boost_mode(FunctionScoreBoostMode::Max)
///     .boost(1.1)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-function-score-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FunctionScoreQuery {
    #[serde(rename = "function_score")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    query: Box<Query>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    functions: Vec<Function>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    max_boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    min_score: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    score_mode: Option<FunctionScoreMode>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost_mode: Option<FunctionScoreBoostMode>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`FunctionScoreQuery`]
    pub fn function_score(query: impl Into<Query>) -> FunctionScoreQuery {
        FunctionScoreQuery {
            inner: Inner {
                query: Box::new(query.into()),
                functions: Default::default(),
                max_boost: None,
                min_score: None,
                score_mode: None,
                boost_mode: None,
                boost: None,
                _name: None,
            },
        }
    }
}

impl FunctionScoreQuery {
    /// Push function to the list
    pub fn function(mut self, function: impl Into<Option<Function>>) -> Self {
        let function = function.into();

        if let Some(function) = function {
            self.inner.functions.push(function);
        }

        self
    }

    /// Maximum score value after applying all the functions
    pub fn max_boost(mut self, max_boost: impl std::convert::TryInto<Boost>) -> Self {
        if let Ok(max_boost) = max_boost.try_into() {
            self.inner.max_boost = Some(max_boost);
        }
        self
    }

    /// By default, modifying the score does not change which documents match. To exclude documents

    /// that do not meet a certain score threshold the `min_score` parameter can be set to the
    /// desired score threshold.
    pub fn min_score(mut self, min_score: impl Into<f32>) -> Self {
        self.inner.min_score = Some(min_score.into());
        self
    }

    /// Each document is scored by the defined functions. The parameter `score_mode` specifies how
    /// the computed scores are combined
    pub fn score_mode(mut self, score_mode: FunctionScoreMode) -> Self {
        self.inner.score_mode = Some(score_mode);
        self
    }

    /// The newly computed score is combined with the score of the query. The parameter
    /// `boost_mode` defines how.
    pub fn boost_mode(mut self, boost_mode: FunctionScoreBoostMode) -> Self {
        self.inner.boost_mode = Some(boost_mode);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for FunctionScoreQuery {
    fn should_skip(&self) -> bool {
        self.inner.query.should_skip() || self.inner.functions.should_skip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Query::function_score(Query::term("test", 1)).function(RandomScore::new()),
            json!({
                "function_score": {
                    "query": {
                        "term": {
                            "test": {
                                "value": 1
                            }
                        }
                    },
                    "functions": [
                        {
                            "random_score": {}
                        }
                    ]
                }
            }),
        );

        assert_serialize(
            Query::function_score(Query::term("test", 1))
                .function(RandomScore::new())
                .function(Weight::new(2.0))
                .max_boost(2.2)
                .min_score(2.3)
                .score_mode(FunctionScoreMode::Avg)
                .boost_mode(FunctionScoreBoostMode::Max)
                .boost(1.1)
                .name("test"),
            json!({
                "function_score": {
                    "query": {
                        "term": {
                            "test": {
                                "value": 1
                            }
                        }
                    },
                    "functions": [
                        {
                            "random_score": {}
                        },
                        {
                            "weight": 2.0
                        }
                    ],
                    "max_boost": 2.2,
                    "min_score": 2.3,
                    "score_mode": "avg",
                    "boost_mode": "max",
                    "boost": 1.1,
                    "_name": "test"
                }
            }),
        );
    }
}
