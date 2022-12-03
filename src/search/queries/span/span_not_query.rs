use crate::util::*;
use crate::{Query, SpanQuery};
use serde::Serialize;

/// TODO
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct SpanNotQuery {
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

impl Query {
    /// Creates an instance of [`SpanNotQuery`]
    pub fn span_not<T, U>(exclude: T, include: U) -> SpanNotQuery
    where
        T: IntoIterator,
        T::Item: Into<SpanQuery>,
        U: IntoIterator,
        U::Item: Into<SpanQuery>,
    {
        SpanNotQuery {
            exclude: exclude.into_iter().map(Into::into).collect(),
            include: include.into_iter().map(Into::into).collect(),
            dist: None,
            post: None,
            pre: None,
        }
    }
}

impl SpanNotQuery {
    /// TODO
    pub fn dist(mut self, dist: i32) -> Self {
        self.dist = Some(dist);
        self
    }

    /// TODO
    pub fn post(mut self, post: i32) -> Self {
        self.post = Some(post);
        self
    }

    /// TODO
    pub fn pre(mut self, pre: i32) -> Self {
        self.pre = Some(pre);
        self
    }
}

impl ShouldSkip for SpanNotQuery {}

serialize_with_root!("span_not": SpanNotQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::span_not(
                [Query::span_term("foo", 1234)],
                [Query::span_term("bar", 4321)],
            )
            .dist(1234)
            .post(4321)
            .pre(5678),
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

        assert_serialize_query(
            Query::span_not(
                [Query::span_term("foo", 1234)],
                [Query::span_term("bar", 4321)],
            )
            .dist(1234)
            .post(4321)
            .pre(5678),
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
