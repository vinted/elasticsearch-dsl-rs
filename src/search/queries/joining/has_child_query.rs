use crate::search::*;
use crate::util::*;

/// Returns parent documents whose joined child documents match a provided query. You can create
/// parent-child relationships between documents in the same index using a join field mapping.
///
/// To create has_child query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::has_child("child", Query::term("tag", "elasticsearch"));
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-has-child-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct HasChildQuery {
    r#type: String,

    query: Box<Query>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    ignore_unmapped: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    max_children: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    min_children: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    score_mode: Option<HasChildScoreMode>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    inner_hits: Option<Box<InnerHits>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`HasChildQuery`]
    ///
    /// - `type` - Name of the child relationship mapped for the join field.
    /// - `query` - Query you wish to run on child documents of the `type` field. If a child
    /// document matches the search, the query returns the parent document.
    pub fn has_child<T, U>(r#type: T, query: U) -> HasChildQuery
    where
        T: ToString,
        U: Into<Query>,
    {
        HasChildQuery {
            r#type: r#type.to_string(),
            query: Box::new(query.into()),
            ignore_unmapped: None,
            max_children: None,
            min_children: None,
            score_mode: None,
            inner_hits: None,
            boost: None,
            _name: None,
        }
    }
}

impl HasChildQuery {
    /// Indicates whether to ignore an unmapped `type` and not return any documents instead of an
    /// error. Defaults to `false`.
    ///
    /// If `false`, Elasticsearch returns an error if the `type` is unmapped.
    ///
    /// You can use this parameter to query multiple indices that may not contain the `type`.
    pub fn ignore_unmapped(mut self, ignore_unmapped: bool) -> Self {
        self.ignore_unmapped = Some(ignore_unmapped);
        self
    }

    /// Maximum number of child documents that match the `query` allowed for a returned parent
    /// document. If the parent document exceeds this limit, it is excluded from the search results.
    pub fn max_children(mut self, max_children: u32) -> Self {
        self.max_children = Some(max_children);
        self
    }

    /// Minimum number of child documents that match the `query` required to match the query for a
    /// returned parent document. If the parent document does not meet this limit, it is excluded
    /// from the search results.
    pub fn min_children(mut self, min_children: u32) -> Self {
        self.min_children = Some(min_children);
        self
    }

    /// Indicates how scores for matching child documents affect the root parent document’s
    /// relevance score.
    pub fn score_mode(mut self, score_mode: HasChildScoreMode) -> Self {
        self.score_mode = Some(score_mode);
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

impl ShouldSkip for HasChildQuery {
    fn should_skip(&self) -> bool {
        self.r#type.should_skip() || self.query.should_skip()
    }
}

serialize_with_root!("has_child": HasChildQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::has_child("child", Query::term("tag", "elasticsearch")),
            json!({
                "has_child": {
                    "type": "child",
                    "query": {
                        "term": {
                            "tag": {
                                "value": "elasticsearch"
                            }
                        }
                    }
                }
            }),
        );

        assert_serialize_query(
            Query::has_child("child", Query::term("tag", "elasticsearch"))
                .boost(2)
                .name("test")
                .ignore_unmapped(true)
                .max_children(3)
                .min_children(2)
                .inner_hits(InnerHits::new())
                .score_mode(HasChildScoreMode::Max),
            json!({
                "has_child": {
                    "type": "child",
                    "ignore_unmapped": true,
                    "max_children": 3,
                    "min_children": 2,
                    "score_mode": "max",
                    "query": {
                        "term": {
                            "tag": {
                                "value": "elasticsearch"
                            }
                        }
                    },
                    "inner_hits": {},
                    "boost": 2.0,
                    "_name": "test"
                }
            }),
        );
    }
}
