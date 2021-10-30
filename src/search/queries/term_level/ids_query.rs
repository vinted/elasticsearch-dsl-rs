use crate::search::*;
use crate::util::*;
use std::collections::BTreeSet;

/// Returns documents based on their IDs. This query uses document IDs stored in the
/// [`_id`](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-id-field.html)
/// field.
///
/// To create IDs query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::ids(vec!["2"]);
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-ids-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct IdsQuery {
    #[serde(rename = "ids")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    values: BTreeSet<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`IdsQuery`]
    ///
    /// - `values` - An array of
    /// [document IDs](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-id-field.html).
    pub fn ids<I>(values: I) -> IdsQuery
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        IdsQuery {
            inner: Inner {
                values: values.into_iter().map(|value| value.to_string()).collect(),
                boost: None,
                _name: None,
            },
        }
    }
}

impl IdsQuery {
    add_boost_and_name!();
}

impl ShouldSkip for IdsQuery {
    fn should_skip(&self) -> bool {
        self.inner.values.should_skip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(
            Query::ids(vec![1, 3, 2, 5, 4, 6]),
            json!({
                "ids": {
                    "values": ["1", "2", "3", "4", "5", "6"],
                }
            })
        );

        with_all_fields(
            Query::ids(vec![1, 3, 2, 5, 4, 6]).boost(1.3).name("test"),
            json!({
                "ids": {
                    "values": ["1", "2", "3", "4", "5", "6"],
                    "boost": 1.3,
                    "_name": "test"
                }
            })
        );
    }
}
