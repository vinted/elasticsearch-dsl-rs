use crate::{search::*, util::*};

/// Returns documents that contain an indexed value for a field.
///
/// An indexed value may not exist for a document’s field due to a variety of reasons:
///
/// - The field in the source JSON is `null` or `[]`
/// - The field has `"index" : false` set in the mapping
/// - The length of the field value exceeded an `ignore_above` setting in the mapping
/// - The field value was malformed and `ignore_malformed` was defined in the mapping
///
/// To create exists query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::exists("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-exists-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ExistsQuery {
    #[serde(rename = "exists")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`ExistsQuery`]
    ///
    /// - `field` - Name of the field you wish to search.
    /// While a field is deemed non-existent if the JSON value is `null` or `[]`,
    /// these values will indicate the field does exist:
    ///   - Empty strings, such as `""` or `"-"`
    ///   - Arrays containing `null` and another value, such as `[null, "foo"]`
    ///   - A custom [`null-value`](https://www.elastic.co/guide/en/elasticsearch/reference/current/null-value.html), defined in field mapping
    pub fn exists(field: impl Into<String>) -> ExistsQuery {
        ExistsQuery {
            inner: Inner {
                field: field.into(),
                boost: None,
                _name: None,
            },
        }
    }
}

impl ExistsQuery {
    add_boost_and_name!();
}

impl ShouldSkip for ExistsQuery {}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            Query::exists("test"),
            json!({
                "exists": {
                    "field": "test"
                }
            })
        );

        with_all_fields(
            Query::exists("test").boost(2).name("test"),
            json!({
                "exists": {
                    "field": "test",
                    "boost": 2.0,
                    "_name": "test"
                }
            })
        );
    }
}
