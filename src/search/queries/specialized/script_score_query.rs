use crate::search::*;
use crate::util::*;

/// A query allowing you to modify the score of documents that are retrieved by
/// a query. This can be useful if, for example, a score function is
/// computationally expensive and it is sufficient to compute the score on a
/// filtered set of documents.
///
/// To create script score query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::script_score(
///     Query::r#match("message", "elasticsearch"),
///     Script::source("doc['my-int'].value / 10"),
/// );
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-script-score-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct ScriptScoreQuery {
    query: Box<Query>,

    script: Script,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    min_score: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`ScriptScoreQuery`]
    ///
    /// - `query` - Query used to return documents
    /// - `script` - Script used to compute the score of documents returned by
    /// the `query`
    pub fn script_score<Q>(query: Q, script: Script) -> ScriptScoreQuery
    where
        Q: Into<Query>,
    {
        ScriptScoreQuery {
            query: Box::new(query.into()),
            script,
            min_score: None,
            boost: None,
            _name: None,
        }
    }
}

impl ScriptScoreQuery {
    add_boost_and_name!();
}

impl ShouldSkip for ScriptScoreQuery {}

serialize_query!("script_score": ScriptScoreQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::script_score(
                Query::r#match("message", "elasticsearch"),
                Script::source("doc['my-int'].value / 10"),
            )
            .name("_named_query")
            .boost(1.1),
            json!({
                "script_score": {
                    "_name": "_named_query",
                    "boost": 1.1,
                    "query": { "match": { "message": { "query": "elasticsearch" } } },
                    "script": {
                        "source": "doc['my-int'].value / 10"
                    }
                }
            }),
        );
    }
}
