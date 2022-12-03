use super::SpanQuery;
use crate::util::*;
use crate::Query;
use serde::Serialize;

/// TODO
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct SpanOrQuery {
    clauses: Vec<SpanQuery>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl ShouldSkip for SpanOrQuery {}

impl Query {
    /// Creates an instance of [`SpanOrQuery`]
    pub fn span_or<T>(clauses: T) -> SpanOrQuery
    where
        T: IntoIterator,
        T::Item: Into<SpanQuery>,
    {
        SpanOrQuery {
            clauses: clauses.into_iter().map(Into::into).collect(),
            boost: None,
            _name: None,
        }
    }
}

serialize_with_root!("span_or": SpanOrQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::span_or([Query::span_term("test", 1234)]),
            json!({
                "span_or": {
                    "clauses": [
                        {
                            "span_term": {
                                "test": {
                                    "value": 1234
                                }
                            }
                        }
                    ]
                }
            }),
        );

        assert_serialize_query(
            Query::span_or([Query::span_term("test", 1234)]),
            json!({
                "span_or": {
                    "clauses": [
                        {
                            "span_term": {
                                "test": {
                                    "value": 1234
                                }
                            }
                        }
                    ]
                }
            }),
        );
    }
}
