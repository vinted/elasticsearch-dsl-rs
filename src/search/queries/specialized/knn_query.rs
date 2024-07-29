use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// Finds the _k_ nearest vectors to a query vector, as measured by a similarity metric. _knn_ query finds nearest
/// vectors through approximate search on indexed dense_vectors. The preferred way to do approximate kNN search is
/// through the
/// [top level knn section](https://www.elastic.co/guide/en/elasticsearch/reference/current/knn-search.html) of a
/// search request. _knn_ query is reserved for expert cases, where there is a need to combine this query with other queries.
///
/// > `knn` query doesn’t have a separate `k` parameter. `k` is defined by `size` parameter of a search request
/// > similar to other queries. `knn` query collects `num_candidates` results from each shard, then merges them to get
/// > the top `size` results.
///
/// To create a knn query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # let query =
/// Query::knn("test", vec![1.0, 2.0, 3.0]);
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-knn-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct KnnQuery {
    field: String,

    query_vector: Vec<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    num_candidates: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    filter: Option<Box<Query>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    similarity: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl KnnQuery {
    /// The number of nearest neighbor candidates to consider per shard. Cannot exceed 10,000. Elasticsearch collects
    /// `num_candidates` results from each shard, then merges them to find the top results. Increasing `num_candidates`
    /// tends to improve the accuracy of the final results. Defaults to `Math.min(1.5 * size, 10_000)`.
    pub fn num_candidates(mut self, num_candidates: u32) -> Self {
        self.num_candidates = Some(num_candidates);
        self
    }

    /// Query to filter the documents that can match. The kNN search will return the top documents that also match
    /// this filter. The value can be a single query or a list of queries. If `filter` is not provided, all documents
    /// are allowed to match.
    ///
    /// The filter is a pre-filter, meaning that it is applied **during** the approximate kNN search to ensure that
    /// `num_candidates` matching documents are returned.
    pub fn filter<T>(mut self, filter: T) -> Self
    where
        T: Into<Query>,
    {
        self.filter = Some(Box::new(filter.into()));
        self
    }

    ///  The minimum similarity required for a document to be considered a match. The similarity value calculated
    /// relates to the raw similarity used. Not the document score. The matched documents are then scored according
    /// to similarity and the provided boost is applied.
    pub fn similarity(mut self, similarity: f32) -> Self {
        self.similarity = Some(similarity);
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for KnnQuery {}

serialize_with_root!("knn": KnnQuery);

impl Query {
    /// Creates an instance of [`KnnQuery`]
    ///
    /// - `field` - The name of the vector field to search against. Must be a dense_vector field with indexing enabled.
    /// - `query_vector` - Query vector. Must have the same number of dimensions as the vector field you are searching
    ///   against.
    pub fn knn<T>(field: T, query_vector: Vec<f32>) -> KnnQuery
    where
        T: ToString,
    {
        KnnQuery {
            field: field.to_string(),
            query_vector,
            num_candidates: None,
            filter: None,
            similarity: None,
            boost: None,
            _name: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::knn("test", vec![1.0, 2.0, 3.0]),
            json!({
                "knn": {
                    "field": "test",
                    "query_vector": [1.0, 2.0, 3.0]
                }
            }),
        );

        assert_serialize_query(
            Query::knn("test", vec![1.0, 2.0, 3.0])
                .num_candidates(100)
                .filter(Query::term("field", "value"))
                .similarity(0.5)
                .boost(2.0)
                .name("test"),
            json!({
                "knn": {
                    "field": "test",
                    "query_vector": [1.0, 2.0, 3.0],
                    "num_candidates": 100,
                    "filter": {
                        "term": {
                            "field": {
                                "value": "value"
                            }
                        }
                    },
                    "similarity": 0.5,
                    "boost": 2.0,
                    "_name": "test"
                }
            }),
        );
    }
}
