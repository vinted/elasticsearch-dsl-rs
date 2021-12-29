use crate::search::*;
use crate::util::*;

/// A query that matches documents matching boolean combinations of other queries.
/// The bool query maps to Lucene BooleanQuery.
/// It is built using one or more boolean clauses, each clause with a typed occurrence.
///
/// The bool query takes a more-matches-is-better approach, so the score from each matching must or should clause will be added together to provide the final _score for each document.
///
/// To create bool query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::bool()
///    .must(Query::term("test1", 1))
///    .must(Query::term("test2", 2))
///    .should(Query::term("test1", 3))
///    .should(Query::term("test2", 4))
///    .filter(Query::term("test1", 5))
///    .filter(Query::term("test2", 6))
///    .must_not(Query::term("test1", 7))
///    .must_not(Query::term("test2", 8))
///    .minimum_should_match("2")
///    .boost(1.3)
///    .name("test");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-bool-query.html>
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub struct BoolQuery {
    #[serde(rename = "bool")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
struct Inner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    must: Queries,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    filter: Queries,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    should: Queries,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    must_not: Queries,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    minimum_should_match: Option<MinimumShouldMatch>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`BoolQuery`]
    pub fn bool() -> BoolQuery {
        BoolQuery::default()
    }
}

impl BoolQuery {
    /// The clause (query) must appear in matching documents and will contribute to the score.
    pub fn must<Q>(mut self, query: Q) -> Self
    where
        Q: Into<Option<Query>>,
    {
        self.inner.must.push(query);
        self
    }

    /// The clause (query) must appear in matching documents and will contribute to the score.
    pub fn musts<I>(mut self, queries: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Query>,
    {
        self.inner.must.extend(queries);
        self
    }

    /// The clause (query) should appear in the matching document.
    pub fn should<Q>(mut self, query: Q) -> Self
    where
        Q: Into<Option<Query>>,
    {
        self.inner.should.push(query);
        self
    }

    /// The clause (query) should appear in the matching document.
    pub fn shoulds<I>(mut self, queries: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Query>,
    {
        self.inner.should.extend(queries);
        self
    }

    /// The clause (query) must appear in matching documents.
    /// However unlike must the score of the query will be ignored.
    /// Filter clauses are executed in [filter context](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html),
    /// meaning that scoring is ignored and clauses are considered for caching.
    pub fn filter<Q>(mut self, query: Q) -> Self
    where
        Q: Into<Option<Query>>,
    {
        self.inner.filter.push(query);
        self
    }

    /// The clause (query) must appear in matching documents.
    /// However unlike must the score of the query will be ignored.
    /// Filter clauses are executed in [filter context](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html),
    /// meaning that scoring is ignored and clauses are considered for caching.
    pub fn filters<I>(mut self, queries: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Query>,
    {
        self.inner.filter.extend(queries);
        self
    }

    /// The clause (query) must not appear in the matching documents.
    /// Clauses are executed in [filter context](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html)
    /// meaning that scoring is ignored and clauses are considered for caching.
    /// Because scoring is ignored, a score of `0` for all documents is returned.
    pub fn must_not<Q>(mut self, query: Q) -> Self
    where
        Q: Into<Option<Query>>,
    {
        self.inner.must_not.push(query);
        self
    }

    /// The clause (query) must not appear in the matching documents.
    /// Clauses are executed in [filter context](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html)
    /// meaning that scoring is ignored and clauses are considered for caching.
    /// Because scoring is ignored, a score of `0` for all documents is returned.
    pub fn must_nots<I>(mut self, queries: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Query>,
    {
        self.inner.must_not.extend(queries);
        self
    }

    /// You can use the `minimum_should_match` parameter to specify the number
    /// or percentage of should clauses returned documents must match.
    ///
    /// If the `bool` query includes at least one `should` clause and no
    /// `must` or `filter` clauses, the default value is `1`.
    /// Otherwise, the default value is `0`.
    ///
    /// For other valid values, see the
    /// [minimum_should_match parameter](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-minimum-should-match.html).
    pub fn minimum_should_match<S>(mut self, minimum_should_match: S) -> Self
    where
        S: Into<MinimumShouldMatch>,
    {
        self.inner.minimum_should_match = Some(minimum_should_match.into());
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for BoolQuery {
    fn should_skip(&self) -> bool {
        self.inner.must.should_skip()
            && self.inner.filter.should_skip()
            && self.inner.should.should_skip()
            && self.inner.must_not.should_skip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(Query::bool(), json!({ "bool": {} }));

        assert_serialize(
            Query::bool()
                .musts([Query::term("test1", 1), Query::term("test2", 2)])
                .shoulds([Query::term("test1", 3), Query::term("test2", 4)])
                .filters([Query::term("test1", 5), Query::term("test2", 6)])
                .must_nots([Query::term("test1", 7), Query::term("test2", 8)])
                .minimum_should_match("2")
                .boost(1.3)
                .name("test"),
            json!({
                "bool": {
                    "must":[
                        { "term": { "test1": {"value": 1} } },
                        { "term": { "test2": {"value": 2} } },
                    ],
                    "should":[
                        { "term": { "test1": {"value": 3} } },
                        { "term": { "test2": {"value": 4} } },
                    ],
                    "filter":[
                        { "term": { "test1": {"value": 5} } },
                        { "term": { "test2": {"value": 6} } },
                    ],
                    "must_not":[
                        { "term": { "test1": {"value": 7} } },
                        { "term": { "test2": {"value": 8} } },
                    ],
                    "minimum_should_match": "2",
                    "boost": 1.3,
                    "_name":"test"
                }
            }),
        );

        assert_serialize(
            Query::bool()
                .must(Query::term("test1", 1))
                .must(Query::term("test2", 2))
                .should(Query::term("test1", 3))
                .should(Query::term("test2", 4))
                .filter(Query::term("test1", 5))
                .filter(Query::term("test2", 6))
                .must_not(Query::term("test1", 7))
                .must_not(Query::term("test2", 8))
                .minimum_should_match("2")
                .boost(1.3)
                .name("test"),
            json!({
                "bool": {
                    "must":[
                        { "term": { "test1": {"value": 1} } },
                        { "term": { "test2": {"value": 2} } },
                    ],
                    "should":[
                        { "term": { "test1": {"value": 3} } },
                        { "term": { "test2": {"value": 4} } },
                    ],
                    "filter":[
                        { "term": { "test1": {"value": 5} } },
                        { "term": { "test2": {"value": 6} } },
                    ],
                    "must_not":[
                        { "term": { "test1": {"value": 7} } },
                        { "term": { "test2": {"value": 8} } },
                    ],
                    "minimum_should_match": "2",
                    "boost": 1.3,
                    "_name":"test"
                }
            }),
        );
    }
}
