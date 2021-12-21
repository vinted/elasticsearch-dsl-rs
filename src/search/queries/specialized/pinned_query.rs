use crate::search::*;
use crate::util::*;

/// The most simple query, which matches all documents, giving them all a
/// `_score` of `1.0`.
///
/// To create match_all query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::pinned(PinnedQueryValues::ids([1]), Query::term("user_id", 2))
///     .boost(2)
///     .name("matches_everything");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-all-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PinnedQuery {
    #[serde(rename = "pinned")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    #[serde(flatten)]
    values: PinnedQueryValues,

    /// Any choice of query used to rank documents which will be ranked below
    /// the "pinned" documents.
    organic: Box<Query>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`PinnedQuery`]
    pub fn pinned<Q>(values: PinnedQueryValues, organic: Q) -> PinnedQuery
    where
        Q: Into<Query>,
    {
        PinnedQuery {
            inner: Inner {
                values,
                organic: Box::new(organic.into()),
                boost: None,
                _name: None,
            },
        }
    }
}

impl PinnedQuery {
    add_boost_and_name!();
}

impl ShouldSkip for PinnedQuery {
    fn should_skip(&self) -> bool {
        self.inner.organic.should_skip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        with_required_ids_fields(
            Query::pinned(
                PinnedQueryValues::ids([1]),
                Query::term("user_id", 2)
            ),
            json!({
                "pinned": {
                    "ids": ["1"],
                    "organic": {
                        "term": {
                            "user_id": {
                                "value": 2
                            }
                        }
                    }
                }
            })
        );

        with_required_docs_fields(
            Query::pinned(
                PinnedQueryValues::docs([PinnedDocument::new("index", 1)]),
                Query::term("user_id", 2)
            ),
            json!({
                "pinned": {
                    "docs": [{ "_index": "index", "_id": "1" }],
                    "organic": {
                        "term": {
                            "user_id": {
                                "value": 2
                            }
                        }
                    }
                }
            })
        );

        with_all_ids_fields(
            Query::pinned(
                PinnedQueryValues::ids([1]),
                Query::term("user_id", 2)
            )
            .boost(2)
            .name("test"),
            json!({
                "pinned": {
                    "ids": ["1"],
                    "organic": {
                        "term": {
                            "user_id": {
                                "value": 2
                            }
                        }
                    },
                    "boost": 2,
                    "_name": "test"
                }
            })
        );

        with_all_docs_fields(
            Query::pinned(
                PinnedQueryValues::docs([PinnedDocument::new("index", 1)]),
                Query::term("user_id", 2)
            )
            .boost(2)
            .name("test"),
            json!({
                "pinned": {
                    "docs": [{ "_index": "index", "_id": "1" }],
                    "organic": {
                        "term": {
                            "user_id": {
                                "value": 2
                            }
                        }
                    },
                    "boost": 2,
                    "_name": "test"
                }
            })
        );
    }
}
