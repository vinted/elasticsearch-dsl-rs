//! A k-nearest neighbor (kNN) search finds the k nearest vectors to a query vector, as measured by a similarity metric.
//!
//! Common use cases for kNN include:
//! - Relevance ranking based on natural language processing (NLP) algorithms
//! - Product recommendations and recommendation engines
//! - Similarity search for images or videos
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/current/knn-search.html#approximate-knn>

use crate::search::*;
use crate::util::*;
use serde::Serialize;

/// Performs a k-nearest neighbor (kNN) search and returns the matching documents.
///
/// The kNN search API performs a k-nearest neighbor (kNN) search on a `dense_vector` field. Given a query vector, it
/// finds the _k_ closest vectors and returns those documents as search hits.
///
/// Elasticsearch uses the HNSW algorithm to support efficient kNN search. Like most kNN algorithms, HNSW is an
/// approximate method that sacrifices result accuracy for improved search speed. This means the results returned are
/// not always the true _k_ closest neighbors.
///
/// The kNN search API supports restricting the search using a filter. The search will return the top `k` documents
/// that also match the filter query.
///
/// To create a knn search with a query vector or query vector builder:
/// ```
/// # use elasticsearch_dsl::*;
/// # let search =
/// Search::new()
///     .knn(Knn::query_vector("test1", vec![1.0, 2.0, 3.0]))
///     .knn(Knn::query_vector_builder("test3", TextEmbedding::new("my-text-embedding-model", "The opposite of pink")));
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-knn-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Knn {
    field: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    query_vector: Option<Vec<f32>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    query_vector_builder: Option<QueryVectorBuilder>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    k: Option<u32>,

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

impl Knn {
    /// Creates an instance of [`Knn`] search with query vector
    ///
    /// - `field` - The name of the vector field to search against. Must be a dense_vector field with indexing enabled.
    /// - `query_vector` - Query vector. Must have the same number of dimensions as the vector field you are searching
    ///   against.
    pub fn query_vector<T>(field: T, query_vector: Vec<f32>) -> Self
    where
        T: ToString,
    {
        Self {
            field: field.to_string(),
            query_vector: Some(query_vector),
            query_vector_builder: None,
            k: None,
            num_candidates: None,
            filter: None,
            similarity: None,
            boost: None,
            _name: None,
        }
    }
    /// Creates an instance of [`Knn`] search with query vector builder
    ///
    /// - `field` - The name of the vector field to search against. Must be a dense_vector field with indexing enabled.
    /// - `query_vector_builder` - A configuration object indicating how to build a query_vector before executing the request.
    pub fn query_vector_builder<T, U>(field: T, query_vector_builder: U) -> Self
    where
        T: ToString,
        U: Into<QueryVectorBuilder>,
    {
        Self {
            field: field.to_string(),
            query_vector: None,
            query_vector_builder: Some(query_vector_builder.into()),
            k: None,
            num_candidates: None,
            filter: None,
            similarity: None,
            boost: None,
            _name: None,
        }
    }

    /// Number of nearest neighbors to return as top hits. This value must be less than `num_candidates`.
    ///
    /// Defaults to `size`.
    pub fn k(mut self, k: u32) -> Self {
        self.k = Some(k);
        self
    }

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

    /// The minimum similarity required for a document to be considered a match. The similarity value calculated
    /// relates to the raw similarity used. Not the document score. The matched documents are then scored according
    /// to similarity and the provided boost is applied.
    pub fn similarity(mut self, similarity: f32) -> Self {
        self.similarity = Some(similarity);
        self
    }

    add_boost_and_name!();
}

/// A configuration object indicating how to build a query_vector before executing the request.
///
/// Currently, the only supported builder is [`TextEmbedding`].
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/8.13/knn-search.html#knn-semantic-search>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryVectorBuilder {
    /// The natural language processing task to perform.
    TextEmbedding(TextEmbedding),
}

/// The natural language processing task to perform.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TextEmbedding {
    model_id: String,
    model_text: String,
}

impl From<TextEmbedding> for QueryVectorBuilder {
    fn from(embedding: TextEmbedding) -> Self {
        Self::TextEmbedding(embedding)
    }
}

impl TextEmbedding {
    /// Creates an instance of [`TextEmbedding`]
    /// - `model_id` - The ID of the text embedding model to use to generate the dense vectors from the query string.
    ///   Use the same model that generated the embeddings from the input text in the index you search against. You can
    ///   use the value of the deployment_id instead in the model_id argument.
    /// - `model_text` - The query string from which the model generates the dense vector representation.
    pub fn new<T, U>(model_id: T, model_text: U) -> Self
    where
        T: ToString,
        U: ToString,
    {
        Self {
            model_id: model_id.to_string(),
            model_text: model_text.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Search::new()
                .knn(Knn::query_vector("test1", vec![1.0, 2.0, 3.0]))
                .knn(
                    Knn::query_vector("test2", vec![4.0, 5.0, 6.0])
                        .k(3)
                        .num_candidates(100)
                        .filter(Query::term("field", "value"))
                        .similarity(0.5)
                        .boost(2.0)
                        .name("test2"),
                )
                .knn(Knn::query_vector_builder(
                    "test3",
                    TextEmbedding::new("my-text-embedding-model", "The opposite of pink"),
                ))
                .knn(
                    Knn::query_vector_builder(
                        "test4",
                        TextEmbedding::new("my-text-embedding-model", "The opposite of blue"),
                    )
                    .k(5)
                    .num_candidates(200)
                    .filter(Query::term("field", "value"))
                    .similarity(0.7)
                    .boost(2.1)
                    .name("test4"),
                ),
            json!({
                "knn": [
                    {
                        "field": "test1",
                        "query_vector": [1.0, 2.0, 3.0]
                    },
                    {
                        "field": "test2",
                        "query_vector": [4.0, 5.0, 6.0],
                        "k": 3,
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
                        "_name": "test2"
                    },
                    {
                        "field": "test3",
                        "query_vector_builder": {
                            "text_embedding": {
                                "model_id": "my-text-embedding-model",
                                "model_text": "The opposite of pink"
                            }
                        }
                    },
                    {
                        "field": "test4",
                        "query_vector_builder": {
                            "text_embedding": {
                                "model_id": "my-text-embedding-model",
                                "model_text": "The opposite of blue"
                            }
                        },
                        "k": 5,
                        "num_candidates": 200,
                        "filter": {
                            "term": {
                                "field": {
                                    "value": "value"
                                }
                            }
                        },
                        "similarity": 0.7,
                        "boost": 2.1,
                        "_name": "test4"
                    }
                ]
            }),
        );
    }
}
