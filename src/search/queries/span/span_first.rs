use crate::util::*;
use crate::{Query, SpanQuery};
use serde::Serialize;

/// TODO
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SpanFirstQuery {
    #[serde(rename = "span_first")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    r#match: Box<SpanQuery>,
    end: i32,
}

impl ShouldSkip for SpanFirstQuery {}

impl Query {
    /// Creates an instance of [`SpanFirstQuery`]
    #[allow(unused)]
    pub fn span_first<Q>(r#match: Q, end: i32) -> SpanFirstQuery
    where
        Q: Into<SpanQuery>,
    {
        SpanFirstQuery {
            inner: Inner {
                r#match: Box::new(r#match.into()),
                end,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Query::span_first(
                SpanQuery::SpanTerm(Query::span_term("test", 1234u32)),
                10i32,
            ),
            json!({
                "span_first": {
                    "match": {
                        "span_term": {
                            "test": {
                                "value": 1234
                            }
                        }
                    },
                    "end": 10
                }
            }),
        );

        assert_serialize(
            Query::span_first(
                SpanQuery::SpanTerm(Query::span_term("test", 1234u32)),
                10i32,
            ),
            json!({
                "span_first": {
                    "match": {
                        "span_term": {
                            "test": {
                                "value": 1234
                            }
                        }
                    },
                    "end": 10
                }
            }),
        );
    }
}
