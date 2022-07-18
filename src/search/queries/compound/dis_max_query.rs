use crate::search::*;
use crate::util::*;

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
/// Query::dis_max()
///     .query(Query::r#match("t1", "text"))
///     .query(Query::r#match("t2", "text"))
///     .tie_breaker(0.5)
///     .boost(3)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-dis-max-query.html>
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct DisMaxQuery {
    queries: QueryCollection,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    tie_breaker: Option<TieBreaker>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`DisMaxQuery`]
    pub fn dis_max() -> DisMaxQuery {
        DisMaxQuery::default()
    }
}

impl DisMaxQuery {
    /// Contains one or more query clauses. Returned documents
    /// **must match one or more** of these queries. If a document matches multiple queries,
    /// Elasticsearch uses the highest
    /// [relevance score](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html)
    pub fn query<T>(mut self, query: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Query>,
    {
        self.queries.extend(query);
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
    pub fn tie_breaker<T>(mut self, tie_breaker: T) -> Self
    where
        T: std::convert::TryInto<TieBreaker>,
    {
        if let Ok(tie_breaker) = tie_breaker.try_into() {
            self.tie_breaker = Some(tie_breaker);
        }
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for DisMaxQuery {
    fn should_skip(&self) -> bool {
        self.queries.should_skip()
    }
}

serialize_with_root!("dis_max": DisMaxQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::dis_max()
                .query(Query::r#match("t1", "text"))
                .query(Query::r#match("t2", "text")),
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
            }),
        );

        assert_serialize_query(
            Query::dis_max().query([Query::r#match("t1", "text"), Query::r#match("t2", "text")]),
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
            }),
        );

        assert_serialize_query(
            Query::dis_max()
                .query(Query::r#match("t1", "text"))
                .query(Query::r#match("t2", "text"))
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
                    "boost": 3,
                    "_name": "test"
                }
            }),
        );
    }
}
