use crate::util::*;
use crate::Query;
use serde::Serialize;

/// TODO
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SpanNearQuery {
    #[serde(rename = "span_near")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    clauses: Vec<Query>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    in_order: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    slop: Option<i32>,
}

impl ShouldSkip for SpanNearQuery {
    fn should_skip(&self) -> bool {
        self.inner.clauses.should_skip()
    }
}

impl Query {
    /// Creates an instance of [`SpanNearQuery`]
    #[allow(unused)]
    pub fn span_near(clauses: Vec<Query>) -> SpanNearQuery {
        SpanNearQuery {
            inner: Inner {
                clauses,
                in_order: None,
                slop: None,
            },
        }
    }
}

impl SpanNearQuery {
    /// TODO
    pub fn in_order(mut self, in_order: bool) -> Self {
        self.inner.in_order = Some(in_order);
        self
    }

    /// TODO
    pub fn slop(mut self, slop: i32) -> Self {
        self.inner.slop = Some(slop);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Query::span_near(
                vec![Query::Term(Query::term("test", 1234u32))],
            ),
            json!({
                "span_near": {
                    "clauses": [
                        {
                            "term": {
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
            Query::span_near(
                vec![Query::Term(Query::term("test", 1234u32))],
            ).in_order(true).slop(4321),
            json!({
                "span_near": {
                    "clauses": [
                        {
                            "term": {
                                "test": {
                                    "value": 1234
                                }
                            }
                        }
                    ],
                    "in_order": true,
                    "slop": 4321
                }
            }),
        );
    }
}
