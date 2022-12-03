use crate::util::*;
use crate::{Query, SpanQuery};
use serde::Serialize;

/// Removes matches which overlap with another span query or which are within x tokens before
/// (controlled by the parameter `pre`) or y tokens after (controlled by the parameter `post`)
/// another SpanQuery. The span not query maps to Lucene `SpanNotQuery`.
///
/// The `include` and `exclude` clauses can be any span type query. The `include` clause is the
/// span query whose matches are filtered, and the `exclude` clause is the span query whose matches
/// must not overlap those returned.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-span-not-query.html>
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
    /// If set the amount of tokens from within the include span can’t have overlap with the
    /// exclude span.
    ///
    /// Equivalent of setting both `pre` and `post`.
    pub fn dist(mut self, dist: i32) -> Self {
        self.dist = Some(dist);
        self
    }

    /// If set the amount of tokens after the include span can’t have overlap with the exclude span.
    ///
    /// Defaults to 0.
    pub fn post(mut self, post: i32) -> Self {
        self.post = Some(post);
        self
    }

    /// If set the amount of tokens before the include span can’t have overlap with the exclude
    /// span.
    ///
    /// Defaults to 0.
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
