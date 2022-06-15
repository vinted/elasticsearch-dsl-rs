use crate::util::*;
use crate::{Query, SpanQuery};
use serde::Serialize;

/// TODO
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SpanOrQuery {
    #[serde(rename = "span_or")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    clauses: Vec<SpanQuery>,
}

impl ShouldSkip for SpanOrQuery {}

impl Query {
    /// Creates an instance of [`SpanOrQuery`]
    #[allow(unused)]
    pub fn span_or(clauses: Vec<SpanQuery>) -> SpanOrQuery {
        SpanOrQuery {
            inner: Inner { clauses },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Query::span_or(vec![SpanQuery::SpanTerm(Query::span_term(
                "test", 1234u32,
            ))]),
            json!({
                "span_or": {
                    "clauses": [
                        {
                            "span_term": {
                                "test": {
                                    "value": 1234
                                }
                            }
                        }
                    ]
                }
            }),
        );

        assert_serialize(
            Query::span_or(vec![SpanQuery::SpanTerm(Query::span_term(
                "test", 1234u32,
            ))]),
            json!({
                "span_or": {
                    "clauses": [
                        {
                            "span_term": {
                                "test": {
                                    "value": 1234
                                }
                            }
                        }
                    ]
                }
            }),
        );
    }
}
