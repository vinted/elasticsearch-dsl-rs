use super::params::*;
use super::Query;
use crate::ShouldSkip;

/// Wraps a [filter query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-bool-query.html)
/// and returns every matching document with a
/// [relevance score](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores)
/// equal to the `boost` parameter value.
///
/// To create constant score query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// ConstantScoreQuery::new(TermQuery::new("test1", 123))
///     .boost(3)
///     .name("test");
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::constant_score(Query::term("test1", 123))
///     .boost(3)
///     .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-constant-score-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ConstantScoreQuery {
    #[serde(rename = "constant_score")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    filter: Box<Query>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [ConstantScoreQuery](ConstantScoreQuery)
    ///
    /// - `filter` - [Filter query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-bool-query.html)
    /// you wish to run. Any returned documents must match this query.<br>
    /// Filter queries do not calculate
    /// [relevance scores](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores).
    /// To speed up performance, Elasticsearch automatically caches frequently used filter queries.
    pub fn constant_score(filter: impl Into<Query>) -> ConstantScoreQuery {
        ConstantScoreQuery::new(filter)
    }
}

impl ConstantScoreQuery {
    /// Creates an instance of [ConstantScoreQuery](ConstantScoreQuery)
    ///
    /// - `filter` - [Filter query](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-bool-query.html)
    /// you wish to run. Any returned documents must match this query.<br>
    /// Filter queries do not calculate
    /// [relevance scores](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores).
    /// To speed up performance, Elasticsearch automatically caches frequently used filter queries.
    pub fn new(filter: impl Into<Query>) -> Self {
        Self {
            inner: Inner {
                filter: Box::new(filter.into()),
                boost: None,
                _name: None,
            },
        }
    }

    add_boost_and_name!();
}

impl ShouldSkip for ConstantScoreQuery {
    fn should_skip(&self) -> bool {
        self.inner.filter.should_skip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::queries::TermQuery;

    test_serialization! {
        with_required_fields(
            ConstantScoreQuery::new(TermQuery::new("test1", 123)),
            json!({
                "constant_score": {
                    "filter": {
                        "term": {
                            "test1": {
                                "value": 123
                            }
                        }
                    }
                }
            })
        );

        with_all_fields(
            ConstantScoreQuery::new(TermQuery::new("test1", 123)).boost(3).name("test"),
            json!({
                "constant_score": {
                    "filter": {
                        "term": {
                            "test1": {
                                "value": 123
                            }
                        }
                    },
                    "boost": 3.0,
                    "_name": "test"
                }
            })
        );
    }
}
