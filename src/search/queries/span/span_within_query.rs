use super::SpanQuery;
use crate::util::*;
use crate::Query;
use serde::Serialize;

/// Returns matches which are enclosed inside another span query. The span within query maps to
/// Lucene `SpanWithinQuery`.
///
/// The `big` and `little` clauses can be any span type query. Matching spans from `little` that
/// are enclosed within `big` are returned.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-span-within-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct SpanWithinQuery {
    big: Box<SpanQuery>,
    little: Box<SpanQuery>,
}

impl Query {
    /// Creates an instance of [`SpanContainingQuery`]
    pub fn span_within<T, U>(little: T, big: U) -> SpanWithinQuery
    where
        T: Into<SpanQuery>,
        U: Into<SpanQuery>,
    {
        SpanWithinQuery {
            little: Box::new(little.into()),
            big: Box::new(big.into()),
        }
    }
}

impl ShouldSkip for SpanWithinQuery {}

serialize_with_root!("span_within": SpanWithinQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::span_within(
                Query::span_term("little", 1234),
                Query::span_term("big", 4321),
            ),
            json!({
                "span_within": {
                    "little": {
                        "span_term": {
                            "little": {
                                "value": 1234
                            }
                        }
                    },
                    "big": {
                        "span_term": {
                            "big": {
                                "value": 4321
                            }
                        }
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::span_within(
                Query::span_term("little", 1234),
                Query::span_term("big", 4321),
            ),
            json!({
                "span_within": {
                    "little": {
                        "span_term": {
                            "little": {
                                "value": 1234
                            }
                        }
                    },
                    "big": {
                        "span_term": {
                            "big": {
                                "value": 4321
                            }
                        }
                    }
                }
            }),
        );
    }
}
