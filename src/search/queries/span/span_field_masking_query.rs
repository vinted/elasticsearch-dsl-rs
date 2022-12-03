use crate::util::*;
use crate::{Query, SpanQuery};

/// Wrapper to allow span queries to participate in composite single-field span queries by lying about their search field. The span field masking query maps to Lucene’s SpanFieldMaskingQuery
///
/// This can be used to support queries like span-near or span-or across different fields, which is not ordinarily permitted.
///
/// Span field masking query is invaluable in conjunction with multi-fields when same content is indexed with multiple analyzers. For instance we could index a field with the standard analyzer which breaks text up into words, and again with the english analyzer which stems words into their root form.
///
/// Example:
///
/// ```http
/// GET /_search
/// {
///   "query": {
///     "span_near": {
///       "clauses": [
///         {
///           "span_term": {
///             "text": "quick brown"
///           }
///         },
///         {
///           "span_field_masking": {
///             "query": {
///               "span_term": {
///                 "text.stems": "fox"
///               }
///             },
///             "field": "text"
///           }
///         }
///       ],
///       "slop": 5,
///       "in_order": false
///     }
///   }
/// }
/// ```
///
/// Note: as span field masking query returns the masked field, scoring will be done using the norms of the field name supplied. This may lead to unexpected scoring behavior.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-span-field-masking-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct SpanFieldMaskingQuery {
    query: Box<SpanQuery>,
    field: String,
}

impl Query {
    /// Creates an instance of [`SpanFieldMaskingQuery`]
    #[allow(unused)]
    pub fn span_field_masking<Q, F>(query: Q, field: F) -> SpanFieldMaskingQuery
    where
        Q: Into<SpanQuery>,
        F: ToString,
    {
        SpanFieldMaskingQuery {
            query: Box::new(query.into()),
            field: field.to_string(),
        }
    }
}

impl ShouldSkip for SpanFieldMaskingQuery {}

serialize_with_root!("span_field_masking": SpanFieldMaskingQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::span_field_masking(Query::span_term("test", 1234), "test"),
            json!({
                "span_field_masking": {
                    "query": {
                        "span_term": {
                            "test": {
                                "value": 1234
                            }
                        }
                    },
                    "field": "test"
                }
            }),
        );

        assert_serialize_query(
            Query::span_field_masking(Query::span_term("test", 1234), "test"),
            json!({
                "span_field_masking": {
                    "query": {
                        "span_term": {
                            "test": {
                                "value": 1234
                            }
                        }
                    },
                    "field": "test"
                }
            }),
        );
    }
}
