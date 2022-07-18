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
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-HasChild-query.html>
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
    boost: Option<Boost>,

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
                .max_children(3u32)
                .min_children(2u32)
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
                    "boost": 2,
                    "_name": "test"
                }
            }),
        );
    }
}
