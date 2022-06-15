use crate::util::*;
use crate::{Query, SpanQuery};
use serde::Serialize;

/// TODO
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SpanWithinQuery {
    #[serde(rename = "span_within")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    big: Box<SpanQuery>,
    little: Box<SpanQuery>,
}

impl ShouldSkip for SpanWithinQuery {}

impl Query {
    /// Creates an instance of [`SpanContainingQuery`]
    #[allow(unused)]
    pub fn span_within(little: impl Into<SpanQuery>, big: impl Into<SpanQuery>) -> SpanWithinQuery {
        SpanWithinQuery {
            inner: Inner {
                little: Box::new(little.into()),
                big: Box::new(big.into()),
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
            Query::span_within(
                SpanQuery::SpanTerm(Query::span_term("little", 1234u32)),
                SpanQuery::SpanTerm(Query::span_term("big", 4321u32)),
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

        assert_serialize(
            Query::span_within(
                SpanQuery::SpanTerm(Query::span_term("little", 1234u32)),
                SpanQuery::SpanTerm(Query::span_term("big", 4321u32)),
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
