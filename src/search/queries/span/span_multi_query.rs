use crate::util::*;
use crate::{MultiTermQuery, Query};
use serde::Serialize;

/// The span_multi query allows you to wrap a `multi term query` (one of
/// [`wildcard`](crate::WildcardQuery), [`fuzzy`](crate::FuzzyQuery),
/// [`prefix`](crate::PrefixQuery), [`range`](crate::RangeQuery) or
/// [`regexp`](crate::RegexpQuery) query) as a [`span query`](crate::SpanQuery), so it can be
/// nested.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-span-multi-term-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct SpanMultiQuery {
    r#match: Box<MultiTermQuery>,
}

impl ShouldSkip for SpanMultiQuery {}

serialize_with_root!("span_multi": SpanMultiQuery);

impl Query {
    /// Creates an instance of [`SpanMultiQuery`]
    #[allow(unused)]
    pub fn span_multi<Q>(r#match: Q) -> SpanMultiQuery
    where
        Q: Into<MultiTermQuery>,
    {
        SpanMultiQuery {
            r#match: Box::new(r#match.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::span_multi(Query::prefix("test", "1234")),
            json!({
                "span_multi": {
                    "match" : {
                        "prefix": {
                            "test": {
                                "value": "1234"
                            }
                        }
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::span_multi(Query::prefix("test", "1234")),
            json!({
                "span_multi": {
                    "match" : {
                        "prefix": {
                            "test": {
                                "value": "1234"
                            }
                        }
                    }
                }
            }),
        );
    }
}
