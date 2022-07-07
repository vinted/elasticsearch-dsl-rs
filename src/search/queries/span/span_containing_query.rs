use crate::util::*;
use crate::{Query, SpanQuery};

/// TODO
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SpanContainingQuery {
    #[serde(rename = "span_containing")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    little: Box<SpanQuery>,
    big: Box<SpanQuery>,
}

impl ShouldSkip for SpanContainingQuery {}

impl Query {
    /// Creates an instance of [`SpanContainingQuery`]
    #[allow(unused)]
    pub fn span_containing<L, B>(little: L, big: B) -> SpanContainingQuery
    where
        L: Into<SpanQuery>,
        B: Into<SpanQuery>,
    {
        SpanContainingQuery {
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
            Query::span_containing(
                SpanQuery::SpanTerm(Query::span_term("little", "1324")),
                SpanQuery::SpanTerm(Query::span_term("big", "4321")),
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

        assert_serialize(
            Query::span_containing(
                SpanQuery::SpanTerm(Query::span_term("little", "1324")),
                SpanQuery::SpanTerm(Query::span_term("big", "4321")),
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
