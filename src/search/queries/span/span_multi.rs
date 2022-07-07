use crate::util::*;
use crate::{MultiTermQuery, Query};
use serde::Serialize;

/// TODO
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SpanMultiQuery {
    #[serde(rename = "span_multi")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    r#match: Box<MultiTermQuery>,
}

impl ShouldSkip for SpanMultiQuery {}

impl Query {
    /// Creates an instance of [`SpanMultiQuery`]
    #[allow(unused)]
    pub fn span_multi<Q>(r#match: Q) -> SpanMultiQuery
        where Q: Into<MultiTermQuery>{
        SpanMultiQuery {
            inner: Inner {
                r#match: Box::new(r#match.into()),
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
            Query::span_multi(MultiTermQuery::Prefix(Query::prefix(
                "test", "1234",
            ))),
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

        assert_serialize(
            Query::span_multi(MultiTermQuery::Prefix(Query::prefix(
                "test", "1234",
            ))),
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
