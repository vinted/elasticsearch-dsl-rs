use crate::util::*;
use crate::{Query, SpanQuery};
use serde::Serialize;

/// TODO
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SpanNotQuery {
    #[serde(rename = "span_not")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    dist: Option<i32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    exclude: Vec<SpanQuery>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    include: Vec<SpanQuery>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    post: Option<i32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pre: Option<i32>,
}

impl ShouldSkip for SpanNotQuery {}

impl Query {
    /// Creates an instance of [`SpanNotQuery`]
    #[allow(unused)]
    pub fn span_not(
        exclude: Vec<SpanQuery>,
        include: Vec<SpanQuery>,
    ) -> SpanNotQuery {
        SpanNotQuery {
            inner: Inner {
                dist: None,
                exclude,
                include,
                post: None,
                pre: None,
            },
        }
    }
}

impl SpanNotQuery {
    /// TODO
    fn dist(mut self, dist: i32) -> Self {
        self.inner.dist = Some(dist);
        self
    }

    /// TODO
    fn post(mut self, post: i32) -> Self {
        self.inner.post = Some(post);
        self
    }

    /// TODO
    fn pre(mut self, pre: i32) -> Self {
        self.inner.pre = Some(pre);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Query::span_not(
                Some(1234),
                vec![SpanQuery::SpanTerm(Query::span_term("foo", 1234u32))],
                vec![SpanQuery::SpanTerm(Query::span_term("bar", 4321u32))],
                Some(4321),
                Some(5678),
            ),
            json!({
                "span_not": {
                    "dist": 1234,
                    "exclude": [
                        {
                            "span_term": {
                                "foo": {
                                    "value": 1234
                                }
                            }
                        }
                    ],
                    "include": [
                        {
                            "span_term": {
                                "bar": {
                                    "value": 4321
                                }
                            }
                        }
                    ],
                    "post": 4321,
                    "pre": 5678
                }
            }),
        );

        assert_serialize(
            Query::span_not(
                Some(1234),
                vec![SpanQuery::SpanTerm(Query::span_term("foo", 1234u32))],
                vec![SpanQuery::SpanTerm(Query::span_term("bar", 4321u32))],
                Some(4321),
                Some(5678),
            ),
            json!({
                "span_not": {
                    "dist": 1234,
                    "exclude": [
                        {
                            "span_term": {
                                "foo": {
                                    "value": 1234
                                }
                            }
                        }
                    ],
                    "include": [
                        {
                            "span_term": {
                                "bar": {
                                    "value": 4321
                                }
                            }
                        }
                    ],
                    "post": 4321,
                    "pre": 5678
                }
            }),
        );
    }
}
