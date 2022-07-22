use super::Query;
use crate::util::*;

/// The most simple query, which matches all documents, giving them all a
/// `_score` of `1.0`.
///
/// To create match_all query:
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
#[serde(remote = "Self")]
pub struct MatchAllQuery {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

serialize_with_root!("match_all": MatchAllQuery);

impl Query {
    /// Creates an instance of [`MatchAllQuery`]
    pub fn match_all() -> MatchAllQuery {
        MatchAllQuery::default()
    }
}

impl MatchAllQuery {
    add_boost_and_name!();
}

impl ShouldSkip for MatchAllQuery {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(Query::match_all(), json!({ "match_all": {} }));

        assert_serialize_query(
            Query::match_all().boost(2).name("test"),
            json!({ "match_all": { "boost": 2.0, "_name": "test" } }),
        );
    }
}
