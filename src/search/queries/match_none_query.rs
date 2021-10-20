use super::params::*;
use super::Query;
use crate::util::*;

/// This is the inverse of the [`match_all`](crate::queries::MatchAllQuery)
/// query, which matches no documents.
///
/// To create match_none query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// MatchNoneQuery::new()
///     .boost(2)
///     .name("matches_nothing");
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::match_none()
///     .boost(2)
///     .name("matches_nothing");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-all-query.html>
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct MatchNoneQuery {
    #[serde(rename = "match_none")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
struct Inner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [MatchNoneQuery](MatchNoneQuery)
    pub fn match_none() -> MatchNoneQuery {
        MatchNoneQuery::new()
    }
}

impl MatchNoneQuery {
    /// Creates an instance of [MatchNoneQuery](MatchNoneQuery)
    pub fn new() -> Self {
        Self::default()
    }

    add_boost_and_name!();
}

impl ShouldSkip for MatchNoneQuery {}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(MatchNoneQuery::new(), json!({"match_none": {} }));

        with_all_fields(
            MatchNoneQuery::new().boost(2).name("test"),
            json!({ "match_none": { "boost": 2.0, "_name": "test" } })
        );
    }
}
