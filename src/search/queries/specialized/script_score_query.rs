use crate::search::*;
use crate::util::*;

/// Uses a [script](https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html)
/// to provide a custom score for returned documents.
///
/// The `script_score` query is useful if, for example, a scoring function is
/// expensive and you only need to calculate the score of a filtered set of
/// documents.
///
/// To create script score query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::script(Script::source("return doc['amount'].value < 10;"));
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-script-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ScriptScoreQuery {
    #[serde(rename = "script_score")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
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
            inner: Inner {
                query: Box::new(query.into()),
                script,
                min_score: None,
                boost: None,
                _name: None,
            },
        }
    }
}

impl ScriptScoreQuery {
    add_boost_and_name!();
}

impl ShouldSkip for ScriptScoreQuery {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(
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
