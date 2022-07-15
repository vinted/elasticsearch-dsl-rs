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
/// Query::match_none()
///     .boost(2)
///     .name("matches_nothing");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-all-query.html>
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(remote = "Self")]
pub struct MatchNoneQuery {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`MatchNoneQuery`]
    pub fn match_none() -> MatchNoneQuery {
        MatchNoneQuery::default()
    }
}

impl MatchNoneQuery {
    add_boost_and_name!();
}

serialize_with_root!("match_none": MatchNoneQuery);

impl ShouldSkip for MatchNoneQuery {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(Query::match_none(), json!({"match_none": {} }));

        assert_serialize_query(
            Query::match_none().boost(2).name("test"),
            json!({ "match_none": { "boost": 2, "_name": "test" } }),
        );
    }
}
