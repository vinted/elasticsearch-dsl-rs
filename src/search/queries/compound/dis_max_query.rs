use crate::{search::*, util::*};
/// Returns documents matching one or more wrapped queries, called query clauses or clauses.
///
/// If a returned document matches multiple query clauses, the `dis_max` query assigns the document
/// the highest relevance score from any matching clause, plus a tie breaking increment for any
/// additional matching subqueries.
///
/// You can use the `dis_max` to search for a term in fields mapped with different
/// [boost](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-boost.html)
/// factors.
///
/// To create disjunction max query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// DisMaxQuery::new()
///     .query(MatchQuery::new("t1", "text"))
///     .query(MatchQuery::new("t2", "text"))
///     .tie_breaker(0.5)
///     .boost(3)
///     .name("test");
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::dis_max()
///     .query(Query::r#match("t1", "text"))
///     .query(Query::r#match("t2", "text"))
///     .tie_breaker(0.5)
///     .boost(3)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-dis-max-query.html>
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct DisMaxQuery {
    #[serde(rename = "dis_max")]
    inner: Inner,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
struct Inner {
    queries: Vec<Query>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    tie_breaker: Option<TieBreaker>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [DisMaxQuery](DisMaxQuery)
    pub fn dis_max() -> DisMaxQuery {
        DisMaxQuery::new()
    }
}

impl DisMaxQuery {
    /// Creates an instance of [DisMaxQuery](DisMaxQuery)
    pub fn new() -> Self {
        Default::default()
    }

    /// Contains one or more query clauses. Returned documents
    /// **must match one or more** of these queries. If a document matches multiple queries,
    /// Elasticsearch uses the highest
    /// [relevance score](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html)
    pub fn query(mut self, query: impl Into<Option<Query>>) -> Self {
        let query = query.into();

        if let Some(query) = query {
            if !query.should_skip() {
                self.inner.queries.push(query);
            }
        }

        self
    }

    /// Contains one or more query clauses. Returned documents
    /// **must match one or more** of these queries. If a document matches multiple queries,
    /// Elasticsearch uses the highest
    /// [relevance score](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html)
    pub fn queries<I>(mut self, queries: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Query>,
    {
        for query in queries.into_iter().map(Into::into) {
            if !query.should_skip() {
                self.inner.queries.push(query);
            }
        }

        self
    }

    /// Floating point number between `0` and `1.0` used to increase the
    /// [relevance scores](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores)
    /// of documents matching multiple query clauses. Defaults to `0.0`.
    ///
    /// You can use the `tie_breaker` value to assign higher relevance scores to
    /// documents that contain the same term in multiple fields than documents that
    /// contain this term in only the best of those multiple fields, without
    /// confusing this with the better case of two different terms in the multiple
    /// fields.
    ///
    /// If a document matches multiple clauses, the `dis_max` query calculates
    /// the relevance score for the document as follows:
    /// 1. Take the relevance score from a matching clause with the highest score.
    /// 2. Multiply the score from any other matching clauses by the tie_breaker value.
    /// 3. Add the highest score to the multiplied scores.
    ///
    /// If the `tie_breaker` value is greater than `0.0`, all matching clauses
    /// count, but the clause with the highest score counts most.
    pub fn tie_breaker(mut self, tie_breaker: impl Into<TieBreaker>) -> Self {
        self.inner.tie_breaker = Some(tie_breaker.into());
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for DisMaxQuery {
    fn should_skip(&self) -> bool {
        self.inner.queries.should_skip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::queries::MatchQuery;

    test_serialization! {
        with_required_fields(
            DisMaxQuery::new()
                .query(MatchQuery::new("t1", "text"))
                .query(MatchQuery::new("t2", "text")),
            json!({
                "dis_max": {
                    "queries": [
                        {
                            "match": {
                                "t1": {
                                    "query": "text"
                                }
                            }
                        },
                        {
                            "match": {
                                "t2": {
                                    "query": "text"
                                }
                            }
                        }
                    ]
                }
            })
        );

        with_multiple_queries(
            DisMaxQuery::new()
                .queries([MatchQuery::new("t1", "text"), MatchQuery::new("t2", "text")]),
            json!({
                "dis_max": {
                    "queries": [
                        {
                            "match": {
                                "t1": {
                                    "query": "text"
                                }
                            }
                        },
                        {
                            "match": {
                                "t2": {
                                    "query": "text"
                                }
                            }
                        }
                    ]
                }
            })
        );

        with_all_fields(
            DisMaxQuery::new()
                .query(MatchQuery::new("t1", "text"))
                .query(MatchQuery::new("t2", "text"))
                .tie_breaker(0.5)
                .boost(3)
                .name("test"),
            json!({
                "dis_max": {
                    "queries": [
                        {
                            "match": {
                                "t1": {
                                    "query": "text"
                                }
                            }
                        },
                        {
                            "match": {
                                "t2": {
                                    "query": "text"
                                }
                            }
                        }
                    ],
                    "tie_breaker": 0.5,
                    "boost": 3.0,
                    "_name": "test"
                }
            })
        );
    }
}
