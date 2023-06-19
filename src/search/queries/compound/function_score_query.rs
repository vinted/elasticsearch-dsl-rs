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
/// Query::function_score()
///     .query(Query::term("test", 1))
///     .function(RandomScore::new().filter(Query::term("test", 1)).weight(2.0))
///     .function(Weight::new(2.0))
///     .max_boost(2.2)
///     .min_score(2.3)
///     .score_mode(FunctionScoreMode::Avg)
///     .boost_mode(FunctionBoostMode::Max)
///     .boost(1.1)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-function-score-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct FunctionScoreQuery {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    query: Option<Box<Query>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    functions: Vec<Function>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    max_boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    min_score: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    score_mode: Option<FunctionScoreMode>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost_mode: Option<FunctionBoostMode>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`FunctionScoreQuery`]
    pub fn function_score() -> FunctionScoreQuery {
        FunctionScoreQuery {
            query: None,
            functions: Default::default(),
            max_boost: None,
            min_score: None,
            score_mode: None,
            boost_mode: None,
            boost: None,
            _name: None,
        }
    }
}

impl FunctionScoreQuery {
    /// Base function score query
    pub fn query<T>(mut self, query: T) -> Self
    where
        T: Into<Option<Query>>,
    {
        self.query = query.into().map(Box::new);
        self
    }

    /// Push function to the list
    pub fn function<T>(mut self, function: T) -> Self
    where
        T: Into<Option<Function>>,
    {
        let function = function.into();

        if let Some(function) = function {
            self.functions.push(function);
        }

        self
    }

    /// Maximum score value after applying all the functions
    pub fn max_boost<T>(mut self, max_boost: T) -> Self
    where
        T: num_traits::AsPrimitive<f32>,
    {
        self.max_boost = Some(max_boost.as_());
        self
    }

    /// By default, modifying the score does not change which documents match. To exclude documents

    /// that do not meet a certain score threshold the `min_score` parameter can be set to the
    /// desired score threshold.
    pub fn min_score<T>(mut self, min_score: T) -> Self
    where
        T: Into<f32>,
    {
        self.min_score = Some(min_score.into());
        self
    }

    /// Each document is scored by the defined functions. The parameter `score_mode` specifies how
    /// the computed scores are combined
    pub fn score_mode(mut self, score_mode: FunctionScoreMode) -> Self {
        self.score_mode = Some(score_mode);
        self
    }

    /// The newly computed score is combined with the score of the query. The parameter
    /// `boost_mode` defines how.
    pub fn boost_mode(mut self, boost_mode: FunctionBoostMode) -> Self {
        self.boost_mode = Some(boost_mode);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for FunctionScoreQuery {
    fn should_skip(&self) -> bool {
        self.query.should_skip() || self.functions.should_skip()
    }
}

serialize_with_root!("function_score": FunctionScoreQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::function_score().function(RandomScore::new()),
            json!({
                "function_score": {
                    "functions": [
                        {
                            "random_score": {}
                        }
                    ]
                }
            }),
        );

        assert_serialize_query(
            Query::function_score()
                .query(Query::term("test", 1))
                .function(RandomScore::new())
                .function(Weight::new(2.0))
                .max_boost(2.2)
                .min_score(2.3)
                .score_mode(FunctionScoreMode::Avg)
                .boost_mode(FunctionBoostMode::Max)
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

    #[test]
    fn issue_24() {
        let _ = json!({
            "function_score": {
                "boost_mode": "replace",
                "functions": [
                    {
                        "filter": { "term": { "type": "stop" } },
                        "field_value_factor": {
                            "field": "weight",
                            "factor": 1.0,
                            "missing": 1.0
                        },
                        "weight": 1.0
                    },
                    {
                        "filter": { "term": { "type": "address" } },
                        "filter": { "term": { "type": "addr" } },
                        "field_value_factor": {
                            "field": "weight",
                            "factor": 1.0,
                            "missing": 1.0
                        },
                        "weight": 1.0
                    },
                    {
                        "filter": { "term": { "type": "admin" } },
                        "field_value_factor": {
                            "field": "weight",
                            "factor": 1.0,
                            "missing": 1.0
                        },
                        "weight": 1.0
                    },
                    {
                        "filter": { "term": { "type": "poi" } },
                        "field_value_factor": {
                            "field": "weight",
                            "factor": 1.0,
                            "missing": 1.0
                        },
                        "weight": 1.0
                    },
                    {
                        "filter": { "term": { "type": "street" } },
                        "field_value_factor": {
                            "field": "weight",
                            "factor": 1.0,
                            "missing": 1.0
                        },
                        "weight": 1.0
                    }
                ]
            }
        });

        let _ = Query::function_score()
            .boost_mode(FunctionBoostMode::Replace)
            .function(
                FieldValueFactor::new("weight")
                    .factor(1.0)
                    .missing(1.0)
                    .weight(1.0)
                    .filter(Query::term("type", "stop")),
            )
            .function(
                FieldValueFactor::new("weight")
                    .factor(1.0)
                    .missing(1.0)
                    .weight(1.0)
                    .filter(Query::terms("type", ["address", "addr"])),
            )
            .function(
                FieldValueFactor::new("weight")
                    .factor(1.0)
                    .missing(1.0)
                    .weight(1.0)
                    .filter(Query::term("type", "admin")),
            )
            .function(
                FieldValueFactor::new("weight")
                    .factor(1.0)
                    .missing(1.0)
                    .weight(1.0)
                    .filter(Query::term("type", "poi")),
            )
            .function(
                FieldValueFactor::new("weight")
                    .factor(1.0)
                    .missing(1.0)
                    .weight(1.0)
                    .filter(Query::term("type", "street")),
            );
    }
}
