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
#[serde(remote = "Self")]
pub struct PinnedQuery {
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
            values,
            organic: Box::new(organic.into()),
            boost: None,
            _name: None,
        }
    }
}

impl PinnedQuery {
    add_boost_and_name!();
}

impl ShouldSkip for PinnedQuery {
    fn should_skip(&self) -> bool {
        self.organic.should_skip()
    }
}

serialize_with_root!("pinned": PinnedQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::pinned(PinnedQueryValues::ids([1]), Query::term("user_id", 2)),
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
            }),
        );

        assert_serialize_query(
            Query::pinned(
                PinnedQueryValues::docs([PinnedDocument::new("index", 1)]),
                Query::term("user_id", 2),
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
            }),
        );

        assert_serialize_query(
            Query::pinned(PinnedQueryValues::ids([1]), Query::term("user_id", 2))
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
            }),
        );

        assert_serialize_query(
            Query::pinned(
                PinnedQueryValues::docs([PinnedDocument::new("index", 1)]),
                Query::term("user_id", 2),
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
            }),
        );
    }
}
