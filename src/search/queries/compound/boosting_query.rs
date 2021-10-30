use crate::search::*;
use crate::util::*;

/// Returns documents matching a `positive` query while reducing the
/// [relevance score](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores)
/// of documents that also match a `negative` query.
///
/// You can use the `boosting` query to demote certain documents without excluding them from the search results.
///
/// To create boosting query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::boosting(Query::term("test1", 123), Query::term("test2", 456), 0.2)
///    .boost(3)
///    .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-boosting-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BoostingQuery {
    #[serde(rename = "boosting")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    positive: Box<Query>,
    negative: Box<Query>,
    negative_boost: NegativeBoost,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`BoostingQuery`]
    ///
    /// - `positive` - Query you wish to run. Any returned documents must match this query.
    /// - `negative` - Query used to decrease the
    /// [relevance score](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores)
    /// of matching documents.<br>
    /// If a returned document matches the `positive` query and this query, the `boosting` query
    /// calculates the final
    /// [relevance score](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores)
    /// for the document as follows:
    ///     1. Take the original relevance score from the `positive` query.
    ///     2. Multiply the score by the `negative_boost` value.
    /// - `negative_boost` - Floating point number between `0` and `1.0` used to decrease the
    /// [relevance scores](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores)
    /// of documents matching the `negative` query.
    pub fn boosting<Q, B>(positive: Q, negative: Q, negative_boost: B) -> BoostingQuery
    where
        Q: Into<Query>,
        B: Into<NegativeBoost>,
    {
        BoostingQuery {
            inner: Inner {
                positive: Box::new(positive.into()),
                negative: Box::new(negative.into()),
                negative_boost: negative_boost.into(),
                boost: None,
                _name: None,
            },
        }
    }
}

impl BoostingQuery {
    add_boost_and_name!();
}

impl ShouldSkip for BoostingQuery {
    fn should_skip(&self) -> bool {
        self.inner.positive.should_skip() || self.inner.negative.should_skip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            Query::boosting(Query::term("test1", 123), Query::term("test2", 456), 0.2),
            json!({
                "boosting": {
                    "positive": {
                        "term": {
                            "test1": {
                                "value": 123
                            }
                        }
                    },
                    "negative": {
                        "term": {
                            "test2": {
                                "value": 456
                            }
                        }
                    },
                    "negative_boost": 0.2
                }
            })
        );

        with_all_fields(
            Query::boosting(Query::term("test1", 123), Query::term("test2", 456), 0.2)
                .boost(3)
                .name("test"),
            json!({
                "boosting": {
                    "positive": {
                        "term": {
                            "test1": {
                                "value": 123
                            }
                        }
                    },
                    "negative": {
                        "term": {
                            "test2": {
                                "value": 456
                            }
                        }
                    },
                    "negative_boost": 0.2,
                    "boost": 3.0,
                    "_name": "test"
                }
            })
        );
    }
}
