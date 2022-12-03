use super::SpanQuery;
use crate::util::*;
use crate::Query;

/// Returns matches which enclose another span query. The span containing query maps to Lucene
/// `SpanContainingQuery`. <br/>
/// The `big` and `little` clauses can be any span type query. Matching spans from `big` that
/// contain matches from `little` are returned.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-span-containing-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct SpanContainingQuery {
    little: Box<SpanQuery>,
    big: Box<SpanQuery>,
}

impl Query {
    /// Creates an instance of [`SpanContainingQuery`]
    pub fn span_containing<T, U>(little: T, big: U) -> SpanContainingQuery
    where
        T: Into<SpanQuery>,
        U: Into<SpanQuery>,
    {
        SpanContainingQuery {
            little: Box::new(little.into()),
            big: Box::new(big.into()),
        }
    }
}

impl ShouldSkip for SpanContainingQuery {}

serialize_with_root!("span_containing": SpanContainingQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::span_containing(
                Query::span_term("little", "1324"),
                Query::span_term("big", "4321"),
            ),
            json!({
                "span_containing": {
                    "little": {
                        "span_term": {
                            "little": {
                                "value": "1324"
                            }
                        }
                    },
                    "big": {
                        "span_term": {
                            "big": {
                                "value": "4321"
                            }
                        }
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::span_containing(
                Query::span_term("little", "1324"),
                Query::span_term("big", "4321"),
            ),
            json!({
                "span_containing": {
                    "little": {
                        "span_term": {
                            "little": {
                                "value": "1324"
                            }
                        }
                    },
                    "big": {
                        "span_term": {
                            "big": {
                                "value": "4321"
                            }
                        }
                    }
                }
            }),
        );
    }
}
