use crate::util::*;
use crate::Query;
use crate::SpanQuery;
use serde::Serialize;

/// TODO
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct SpanNearQuery {
    clauses: Vec<SpanQuery>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    in_order: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    slop: Option<i32>,
}

impl ShouldSkip for SpanNearQuery {
    fn should_skip(&self) -> bool {
        self.clauses.should_skip()
    }
}

impl Query {
    /// Creates an instance of [`SpanNearQuery`]
    pub fn span_near<T>(clauses: T) -> SpanNearQuery
    where
        T: IntoIterator,
        T::Item: Into<SpanQuery>,
    {
        SpanNearQuery {
            clauses: clauses.into_iter().map(Into::into).collect(),
            in_order: None,
            slop: None,
        }
    }
}

impl SpanNearQuery {
    /// TODO
    pub fn in_order(mut self, in_order: bool) -> Self {
        self.in_order = Some(in_order);
        self
    }

    /// TODO
    pub fn slop(mut self, slop: i32) -> Self {
        self.slop = Some(slop);
        self
    }
}

serialize_with_root!("span_near": SpanNearQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::span_near([Query::span_term("test", 1234)]),
            json!({
                "span_near": {
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
            Query::span_near([Query::span_term("test", 1234)])
                .in_order(true)
                .slop(4321),
            json!({
                "span_near": {
                    "clauses": [
                        {
                            "span_term": {
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
