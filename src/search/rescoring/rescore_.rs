use crate::util::ShouldSkip;
use crate::{Query, ScoreMode};

/// Rescoring can help to improve precision by reordering just the top (eg 100 - 500)
/// documents returned by the [query](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html#request-body-search-query)
/// and [post_filter](https://www.elastic.co/guide/en/elasticsearch/reference/current/filter-search-results.html#post-filter)
/// phases, using a secondary (usually more costly) algorithm, instead of applying the costly algorithm to all documents in the index.
///
/// A `rescore` request is executed on each shard before it returns its results to be sorted by the node handling the overall search request.
///
/// Currently the rescore API has only one implementation: the query rescorer, which uses a query to tweak the scoring.
/// In the future, alternative rescorers may be made available, for example, a pair-wise rescorer.
///
/// To create a `rescore` query with simple `term` query:
/// ```
/// # use elasticsearch_dsl::rescoring::*;
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let rescore =
/// Rescore::new(Query::term("title", "test"));
/// ```
/// To create a `rescore` query with simple `term` query and optional fields:
/// ```
/// # use elasticsearch_dsl::rescoring::*;
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Rescore::new(Query::term("title", "test"))
///     .rescore_query_weight(0.2)
///     .window_size(100);
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/filter-search-results.html#rescore>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Rescore {
    query: RescoreQuery,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    window_size: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct RescoreQuery {
    rescore_query: Option<Query>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    rescore_query_weight: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    query_weight: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    score_mode: Option<ScoreMode>,
}

impl Rescore {
    /// Creates a new instance of [Rescore]
    ///
    /// - `query` - Second query which will be execute on top-k results returned by original query.
    pub fn new<T>(query: T) -> Self
    where
        T: Into<Option<Query>>,
    {
        Self {
            query: RescoreQuery {
                rescore_query: query.into(),
                rescore_query_weight: None,
                query_weight: None,
                score_mode: None,
            },
            window_size: None,
        }
    }

    /// The number of docs which will be examined on each shard can be controlled by the `window_size` parameter, which defaults to 10.
    pub fn window_size(mut self, window_size: u64) -> Self {
        self.window_size = Some(window_size);
        self
    }

    /// The relative importance of the rescore query can be controlled with the `rescore_query_weight` respectively. Both default to 1.
    pub fn rescore_query_weight(mut self, rescore_query_weight: f32) -> Self {
        self.query.rescore_query_weight = Some(rescore_query_weight);
        self
    }

    /// The relative importance of the original query can be controlled with the `query_weight` respectively. Both default to 1.
    pub fn query_weight(mut self, query_weight: f32) -> Self {
        self.query.query_weight = Some(query_weight);
        self
    }

    /// The way the scores are combined can be controlled with the
    pub fn score_mode(mut self, score_mode: ScoreMode) -> Self {
        self.query.score_mode = Some(score_mode);
        self
    }
}

impl ShouldSkip for Rescore {
    fn should_skip(&self) -> bool {
        self.query
            .rescore_query
            .as_ref()
            .map_or(true, ShouldSkip::should_skip)
    }
}

impl IntoIterator for Rescore {
    type Item = Self;

    type IntoIter = std::option::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Some(self).into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::assert_serialize_rescore;

    #[test]
    fn should_skip() {
        assert!(Rescore::new(Query::range("field")).should_skip());
        assert!(!Rescore::new(Query::range("field").gte(1)).should_skip());
    }

    #[test]
    fn serialization() {
        assert_serialize_rescore(
            Rescore::new(Query::term("title", "test")),
            json!({
                "query": {
                    "rescore_query": {
                        "term": {
                            "title": {
                                "value": "test"
                            }
                        }
                    }
                }
            }),
        );

        assert_serialize_rescore(
            Rescore::new(Query::term("title", "test"))
                .rescore_query_weight(0.2)
                .query_weight(0.5)
                .window_size(100)
                .score_mode(ScoreMode::Max),
            json!({
                "window_size": 100,
                "query": {
                    "query_weight": 0.5,
                    "rescore_query_weight": 0.2,
                    "score_mode": "max",
                    "rescore_query": {
                        "term": {
                            "title": {
                                "value": "test"
                            }
                        }
                    }
                }
            }),
        );
    }
}
