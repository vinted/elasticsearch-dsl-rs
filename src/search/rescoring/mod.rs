//! Rescore clause to run second query over original one results and that way give more accuracy for final results
//! <https://www.elastic.co/guide/en/elasticsearch/reference/6.8/search-request-rescore.html>

use crate::search::*;
use crate::util::*;

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
    query: Inner,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    window_size: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    rescore_query: Option<Query>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    rescore_query_weight: Option<f64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    query_weight: Option<f64>,
}

impl Rescore {
    /// Creates a new instance of [`Rescore`]
    ///
    /// - `query` - Second query which will be execute on top-k results returned by original query.
    pub fn new(query: impl Into<Option<Query>>) -> Self {
        Self {
            query: Inner {
                rescore_query: query.into(),
                rescore_query_weight: None,
                query_weight: None,
            },
            window_size: None,
        }
    }

    /// The number of docs which will be examined on each shard can be controlled by the `window_size` parameter, which defaults to 10.
    pub fn window_size(mut self, window_size: impl Into<i64>) -> Self {
        self.window_size = Some(window_size.into());
        self
    }

    /// The relative importance of the rescore query can be controlled with the `rescore_query_weight` respectively. Both default to 1.
    pub fn rescore_query_weight(mut self, rescore_query_weight: impl Into<f64>) -> Self {
        self.query.rescore_query_weight = Some(rescore_query_weight.into());
        self
    }

    /// The relative importance of the original query can be controlled with the `query_weight` respectively. Both default to 1.
    pub fn query_weight(mut self, query_weight: impl Into<f64>) -> Self {
        self.query.query_weight = Some(query_weight.into());
        self
    }
}

impl ShouldSkip for Rescore {
    fn should_skip(&self) -> bool {
        self.query
            .rescore_query
            .as_ref()
            .map(|q| q.should_skip())
            .unwrap_or(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_skip() {
        assert!(Rescore::new(Query::range("field")).should_skip());
        assert!(!Rescore::new(Query::range("field").gte(1)).should_skip());
    }

    test_serialization! {
        with_required_fields(
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
            })
        );

        with_optional_fields(
            Rescore::new(Query::term("title", "test"))
                .rescore_query_weight(0.2)
                .query_weight(0.5)
                .window_size(100),
            json!({
                "query": {
                    "rescore_query": {
                        "term": {
                            "title": {
                                "value": "test"
                            }
                        }
                    },
                    "query_weight": 0.5,
                    "rescore_query_weight": 0.2
                },
                "window_size": 100
            })
        );
    }
}
