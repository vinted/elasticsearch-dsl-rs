use crate::search::*;
use crate::util::*;

/// Wraps another query to search
/// [nested](https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html)
/// fields.
///
/// The `nested` query searches nested field objects as if they were indexed as
/// separate documents. If an object matches the search, the `nested` query
/// returns the root parent document.
///
/// To create nested query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::nested("vehicles", Query::term("vehicles.license", "ABC123"))
///     .boost(3)
///     .name("test");
/// ```
/// To create multi-level nested query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::nested("driver", Query::nested("driver.vehicle", Query::term("driver.vehicle.make", "toyota")))
///     .boost(3)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-nested-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct NestedQuery {
    path: String,

    query: Box<Query>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    score_mode: Option<NestedQueryScoreMode>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    ignore_unmapped: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    inner_hits: Option<Box<InnerHits>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`NestedQuery`]
    ///
    /// - `path` - Path to the nested object you wish to search.
    /// - `query` - Query you wish to run on nested objects in the `path`. If an object
    /// matches the search, the `nested` query returns the root parent document.<br>
    /// You can search nested fields using dot notation that includes the
    /// complete path, such as `obj1.name`.<br>
    /// Multi-level nesting is automatically supported, and detected,
    /// resulting in an inner nested query to automatically match the relevant
    /// nesting level, rather than root, if it exists within another nested
    /// query.<br>
    /// Multi-level nested queries are also supported.
    pub fn nested<T, U>(path: T, query: U) -> NestedQuery
    where
        T: Into<String>,
        U: Into<Query>,
    {
        NestedQuery {
            path: path.into(),
            query: Box::new(query.into()),
            score_mode: None,
            ignore_unmapped: None,
            inner_hits: None,
            boost: None,
            _name: None,
        }
    }
}

impl NestedQuery {
    /// Indicates how scores for matching child objects affect the root parent
    /// document’s
    /// [relevance score](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores).
    pub fn score_mode(mut self, score_mode: NestedQueryScoreMode) -> Self {
        self.score_mode = Some(score_mode);
        self
    }

    /// Indicates whether to ignore an unmapped `path` and not return any
    /// documents instead of an error. Defaults to `false`.
    ///
    /// If `false`, Elasticsearch returns an error if the `path` is an unmapped
    /// field.
    ///
    /// You can use this parameter to query multiple indices that may not
    /// contain the field `path`.
    pub fn ignore_unmapped(mut self, ignore_unmapped: bool) -> Self {
        self.ignore_unmapped = Some(ignore_unmapped);
        self
    }

    /// The [parent-join](https://www.elastic.co/guide/en/elasticsearch/reference/current/parent-join.html)
    /// and [nested](https://www.elastic.co/guide/en/elasticsearch/reference/current/nested.html)
    /// features allow the return of documents that have matches in a different scope. In the
    /// parent/child case, parent documents are returned based on matches in child documents or
    /// child documents are returned based on matches in parent documents. In the nested case,
    /// documents are returned based on matches in nested inner objects.
    ///
    /// In both cases, the actual matches in the different scopes that caused a document to be
    /// returned are hidden. In many cases, it’s very useful to know which inner nested objects
    /// (in the case of nested) or children/parent documents (in the case of parent/child) caused
    /// certain information to be returned. The inner hits feature can be used for this. This
    /// feature returns per search hit in the search response additional nested hits that caused a
    /// search hit to match in a different scope.
    ///
    /// Inner hits can be used by defining an `inner_hits` definition on a `nested`, `has_child`
    /// or `has_parent` query and filter.
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/inner-hits.html>
    pub fn inner_hits(mut self, inner_hits: InnerHits) -> Self {
        self.inner_hits = Some(Box::new(inner_hits));
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for NestedQuery {
    fn should_skip(&self) -> bool {
        self.query.should_skip()
    }
}

serialize_with_root!("nested": NestedQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::nested("vehicles", Query::term("vehicles.license", "ABC123")),
            json!({
                "nested": {
                    "path": "vehicles",
                    "query": {
                        "term": {
                            "vehicles.license": {
                                "value": "ABC123"
                            }
                        }
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::nested("vehicles", Query::term("vehicles.license", "ABC123"))
                .boost(3)
                .name("test"),
            json!({
                "nested": {
                    "path": "vehicles",
                    "query": {
                        "term": {
                            "vehicles.license": {
                                "value": "ABC123"
                            }
                        }
                    },
                    "boost": 3,
                    "_name": "test",
                }
            }),
        );

        assert_serialize_query(
            Query::nested(
                "driver",
                Query::nested(
                    "driver.vehicles",
                    Query::term("driver.vehicles.make.keyword", "toyota"),
                ),
            ),
            json!({
                "nested": {
                    "path": "driver",
                    "query": {
                        "nested": {
                            "path": "driver.vehicles",
                            "query": {
                                "term": {
                                    "driver.vehicles.make.keyword": {
                                        "value": "toyota"
                                    }
                                }
                            }
                        }
                    }
                }
            }),
        );
    }
}
