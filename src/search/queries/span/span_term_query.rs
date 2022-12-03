use crate::util::*;
use crate::{Query, Term};
use serde::Serialize;

/// TODO
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct SpanTermQuery {
    #[serde(skip)]
    field: String,

    value: Option<Term>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`SpanTermQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `value` - Term you wish to find in the provided field.
    /// To return a document, the term must exactly match the field value, including whitespace and capitalization.
    pub fn span_term<T, U>(field: T, value: U) -> SpanTermQuery
    where
        T: ToString,
        U: Serialize,
    {
        SpanTermQuery {
            field: field.to_string(),
            value: Term::new(value),
            boost: None,
            _name: None,
        }
    }
}

impl SpanTermQuery {
    add_boost_and_name!();
}

impl ShouldSkip for SpanTermQuery {
    fn should_skip(&self) -> bool {
        self.value.should_skip()
    }
}

serialize_with_root_keyed!("span_term": SpanTermQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::span_term("test", 123u32),
            json!({
                "span_term": {
                    "test": {
                        "value": 123
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::span_term("test", 123).boost(2).name("test"),
            json!({
                "span_term": {
                    "test": {
                        "value": 123,
                        "boost": 2.0,
                        "_name": "test"
                    }
                }
            }),
        );
    }
}
