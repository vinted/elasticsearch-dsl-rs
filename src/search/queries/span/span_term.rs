use crate::util::*;
use crate::{Boost, Query, Term};
use serde::ser::{Serialize, SerializeMap, Serializer};

/// TODO
#[derive(Debug, Clone, PartialEq)]
pub struct SpanTermQuery {
    field: String,
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    value: Term,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`SpanTermQuery`]
    ///
    /// - `field` - Field you wish to search.
    /// - `value` - Term you wish to find in the provided field.
    /// To return a document, the term must exactly match the field value, including whitespace and capitalization.
    pub fn span_term(field: impl Into<String>, value: impl Into<Term>) -> SpanTermQuery {
        SpanTermQuery {
            field: field.into(),
            inner: Inner {
                value: value.into(),
                boost: None,
                _name: None,
            },
        }
    }
}

impl SpanTermQuery {
    add_boost_and_name!();
}

impl ShouldSkip for SpanTermQuery {
    fn should_skip(&self) -> bool {
        self.inner.value.should_skip()
    }
}

impl Serialize for SpanTermQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hash = std::collections::HashMap::new();
        let _ = hash.insert(&self.field, &self.inner);

        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("span_term", &hash)?;
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Query::span_term("test", 123u32),
            json!({
                "span_term": {
                    "test": {
                        "value": 123u32
                    }
                }
            }),
        );

        assert_serialize(
            Query::span_term("test", 123u32)
                .boost(2u32)
                .name("test"),
            json!({
                "span_term": {
                    "test": {
                        "value": 123u32,
                        "boost": 2u32,
                        "_name": "test"
                    }
                }
            }),
        );
    }
}
