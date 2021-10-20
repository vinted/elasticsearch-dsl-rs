use super::params::*;
use super::Query;
use crate::ShouldSkip;

/// The most simple query, which matches all documents, giving them all a
/// `_score` of `1.0`.
///
/// To create match_all query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// MatchAllQuery::new()
///     .boost(2)
///     .name("matches_everything");
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::match_all()
///     .boost(2)
///     .name("matches_everything");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-all-query.html>
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct MatchAllQuery {
    #[serde(rename = "match_all")]
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
    /// Creates an instance of [MatchAllQuery](MatchAllQuery)
    pub fn match_all() -> MatchAllQuery {
        MatchAllQuery::new()
    }
}

impl MatchAllQuery {
    /// Creates an instance of [MatchAllQuery](MatchAllQuery)
    pub fn new() -> Self {
        Self::default()
    }

    add_boost_and_name!();
}

impl ShouldSkip for MatchAllQuery {
    fn should_skip(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_fields(MatchAllQuery::new(), json!({ "match_all": {} }));

        with_all_fields(
            MatchAllQuery::new().boost(2).name("test"),
            json!({ "match_all": { "boost": 2.0, "_name": "test" } })
        );
    }
}
