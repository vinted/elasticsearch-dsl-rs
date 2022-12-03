use crate::util::*;
use crate::{Query, SpanQuery};
use serde::Serialize;

/// TODO
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct SpanFirstQuery {
    r#match: Box<SpanQuery>,
    end: i32,
}

impl Query {
    /// Creates an instance of [`SpanFirstQuery`]
    pub fn span_first<T>(r#match: T, end: i32) -> SpanFirstQuery
    where
        T: Into<SpanQuery>,
    {
        SpanFirstQuery {
            r#match: Box::new(r#match.into()),
            end,
        }
    }
}

impl ShouldSkip for SpanFirstQuery {}

serialize_with_root!("span_first": SpanFirstQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::span_first(Query::span_term("test", 1234), 10),
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

        assert_serialize_query(
            Query::span_first(Query::span_term("test", 1234), 10),
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
