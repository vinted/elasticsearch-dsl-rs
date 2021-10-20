use crate::{search::*, util::*};
use std::collections::BTreeSet;

/// The More Like This Query finds documents that are "like" a given set of documents.
/// In order to do so, MLT selects a set of representative terms of these input documents,
/// forms a query using these terms, executes the query and returns the results.
/// The user controls the input documents, how the terms should be selected and how the query is formed.
///
/// The simplest use case consists of asking for documents that are similar to a provided piece of text.
/// Here, we are asking for all movies that have some text similar to "Once upon a time"
/// in their "title" and in their "description" fields, limiting the number of selected terms to 12.
///
/// A more complicated use case consists of mixing texts with documents already existing in the index.
/// In this case, the syntax to specify a document is similar to the one used in the
/// [Multi GET API](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-multi-get.html).
///
/// Finally, users can mix some texts, a chosen set of documents but also provide documents not necessarily present in the index.
/// To provide documents not present in the index, the syntax is similar to
/// [artificial documents](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-termvectors.html#docs-termvectors-artificial-doc).
///
/// **How it Works**
/// Suppose we wanted to find all documents similar to a given input document. Obviously, the input document
/// itself should be its best match for that type of query. And the reason would be mostly,
/// according to [Lucene scoring formula](https://lucene.apache.org/core/4_9_0/core/org/apache/lucene/search/similarities/TFIDFSimilarity.html),
/// due to the terms with the highest tf-idf. Therefore, the terms of the input document that have the highest
/// tf-idf are good representatives of that document, and could be used within a disjunctive query (or OR) to retrieve similar documents.
/// The MLT query simply extracts the text from the input document, analyzes it, usually using the same analyzer at the field,
/// then selects the top K terms with highest tf-idf to form a disjunctive query of these terms.
///
/// To create a `more_like_this` query with `like` as a string on title field:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// MoreLikeThisQuery::new(["test"])
///     .fields(["title"]);
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::term("test", 123);
/// Query::more_like_this(["test"])
///     .fields(["title"]);
/// ```
/// To create a `more_like_this` query with string and document id fields on title and description with optional fields:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// MoreLikeThisQuery::new([Like::from(Document::new("123")), Like::from("test")])
///     .fields(["title", "description"])
///     .min_term_freq(1)
///     .max_query_terms(12)
///     .boost(1.2)
///     .name("more_like_this");
/// ```
/// or
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::more_like_this([Like::from(Document::new("123")), Like::from("test")])
///     .fields(["title", "description"])
///     .min_term_freq(1)
///     .max_query_terms(12)
///     .boost(1.2)
///     .name("more_like_this");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-mlt-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MoreLikeThisQuery {
    #[serde(rename = "more_like_this")]
    inner: Inner,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Inner {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fields: Option<Vec<String>>,

    like: Vec<Like>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    unlike: Option<Vec<Like>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    min_term_freq: Option<i64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    max_query_terms: Option<i64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    min_doc_freq: Option<i64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    max_doc_freq: Option<i64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    min_word_length: Option<i64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    max_word_length: Option<i64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    stop_words: Option<Vec<String>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    analyzer: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    minimum_should_match: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fail_on_unsupported_field: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost_terms: Option<f64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    include: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<Boost>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

/// Types for `like` and `unlike` fields
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Like {
    /// String/text which will be used in `like` field array
    String(String),

    /// Struct to describe elasticsearch document which will be used in `like` field array
    Document(Document),
}

impl From<String> for Like {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl<'a> From<&'a str> for Like {
    fn from(value: &'a str) -> Self {
        Self::String(value.into())
    }
}

impl From<Document> for Like {
    fn from(value: Document) -> Self {
        Self::Document(value)
    }
}

/// One of `like` and `unlike` types which has like document structure
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Document {
    _id: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _index: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _routing: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _source: Option<SourceFilter>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    _stored_fields: BTreeSet<String>,
}

impl Document {
    /// Creates an instance of [Document](Document)
    ///
    /// - `id` - document id as string.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            _id: id.into(),
            _stored_fields: BTreeSet::new(),
            _index: None,
            _routing: None,
            _source: None,
        }
    }

    /// The index that contains the document. Required if no index is specified in the request URI.
    pub fn index(mut self, index: impl Into<String>) -> Self {
        self._index = Some(index.into());
        self
    }

    /// The key for the primary shard the document resides on. Required if routing is used during indexing.
    pub fn routing(mut self, routing: impl Into<String>) -> Self {
        self._routing = Some(routing.into());
        self
    }

    /// If `false`, excludes all `_source` fields. Defaults to `true`.
    pub fn source(mut self, source: impl Into<SourceFilter>) -> Self {
        self._source = Some(source.into());
        self
    }

    /// The stored fields you want to retrieve.
    pub fn stored_fields<I>(mut self, stored_fields: I) -> Self
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        self._stored_fields = stored_fields
            .into_iter()
            .map(|value| value.to_string())
            .collect();
        self
    }
}

impl Query {
    /// Creates an instance of [MoreLikeThisQuery](MoreLikeThisQuery)
    ///
    /// - `like` - free form text and/or a single or multiple documents.
    pub fn more_like_this<I>(like: I) -> MoreLikeThisQuery
    where
        I: IntoIterator,
        I::Item: Into<Like>,
    {
        MoreLikeThisQuery::new(like)
    }
}

impl MoreLikeThisQuery {
    /// Creates an instance of [MoreLikeThisQuery](MoreLikeThisQuery)
    ///
    /// - `like` - free form text and/or a single or multiple documents.
    pub fn new<I>(like: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Like>,
    {
        Self {
            inner: Inner {
                like: like.into_iter().map(Into::into).collect(),
                fields: None,
                unlike: None,
                min_term_freq: None,
                max_query_terms: None,
                min_doc_freq: None,
                max_doc_freq: None,
                min_word_length: None,
                max_word_length: None,
                stop_words: None,
                analyzer: None,
                minimum_should_match: None,
                fail_on_unsupported_field: None,
                boost_terms: None,
                include: None,
                boost: None,
                _name: None,
            },
        }
    }

    /// A list of fields to fetch and analyze the text from.
    /// Defaults to the index.query.default_field index setting, which has a default value of *.
    /// The * value matches all fields eligible for `term-level queries`, excluding metadata fields.
    pub fn fields<I>(mut self, fields: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        self.inner.fields = Some(fields.into_iter().map(Into::into).collect());
        self
    }

    /// The unlike parameter is used in conjunction with like in order not to select terms found in a chosen set of documents.
    /// In other words, we could ask for documents like: "Apple", but unlike: "cake crumble tree". The syntax is the same as like.
    pub fn unlike<I>(mut self, unlike: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Like>,
    {
        self.inner.unlike = Some(unlike.into_iter().map(Into::into).collect());
        self
    }

    /// The maximum number of query terms that will be selected.
    /// Increasing this value gives greater accuracy at the expense of query execution speed.
    /// Defaults to 25.
    pub fn max_query_terms(mut self, max_query_terms: impl Into<i64>) -> Self {
        self.inner.max_query_terms = Some(max_query_terms.into());
        self
    }

    /// The minimum term frequency below which the terms will be ignored from the input document.
    /// Defaults to 2.
    pub fn min_term_freq(mut self, min_term_freq: impl Into<i64>) -> Self {
        self.inner.min_term_freq = Some(min_term_freq.into());
        self
    }

    /// The minimum document frequency below which the terms will be ignored from the input document.
    /// Defaults to 5.
    pub fn min_doc_freq(mut self, min_doc_freq: impl Into<i64>) -> Self {
        self.inner.min_doc_freq = Some(min_doc_freq.into());
        self
    }

    /// The maximum document frequency above which the terms will be ignored from the input document.
    /// This could be useful in order to ignore highly frequent words such as stop words.
    /// Defaults to unbounded (Integer.MAX_VALUE, which is 2^31-1 or 2147483647).
    pub fn max_doc_freq(mut self, max_doc_freq: impl Into<i64>) -> Self {
        self.inner.max_doc_freq = Some(max_doc_freq.into());
        self
    }

    /// The minimum word length below which the terms will be ignored. Defaults to 0.
    pub fn min_word_length(mut self, min_word_length: impl Into<i64>) -> Self {
        self.inner.min_word_length = Some(min_word_length.into());
        self
    }

    /// The maximum word length above which the terms will be ignored. Defaults to unbounded (0).
    pub fn max_word_length(mut self, max_word_length: impl Into<i64>) -> Self {
        self.inner.max_word_length = Some(max_word_length.into());
        self
    }

    /// An array of stop words. Any word in this set is considered "uninteresting" and ignored.
    /// If the analyzer allows for stop words, you might want to tell MLT to explicitly ignore them,
    /// as for the purposes of document similarity it seems reasonable to assume that "a stop word is never interesting".
    pub fn stop_words<I>(mut self, stop_words: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        self.inner.stop_words = Some(stop_words.into_iter().map(Into::into).collect());
        self
    }

    /// The analyzer that is used to analyze the free form text.
    /// Defaults to the analyzer associated with the first field in `fields`.
    pub fn analyzer(mut self, analyzer: impl Into<String>) -> Self {
        self.inner.analyzer = Some(analyzer.into());
        self
    }

    /// After the disjunctive query has been formed, this parameter controls the number of terms that must match.
    /// The syntax is the same as the `minimum should match`. (Defaults to "30%").
    pub fn minimum_should_match(mut self, minimum_should_match: impl Into<String>) -> Self {
        self.inner.minimum_should_match = Some(minimum_should_match.into());
        self
    }

    /// Controls whether the query should fail (throw an exception) if any of the specified fields are not of the supported types (text or keyword).
    /// Set this to false to ignore the field and continue processing. Defaults to true.
    pub fn fail_on_unsupported_field(mut self, fail_on_unsupported_field: impl Into<bool>) -> Self {
        self.inner.fail_on_unsupported_field = Some(fail_on_unsupported_field.into());
        self
    }

    /// Each term in the formed query could be further boosted by their tf-idf score. This sets the boost factor to use when using this feature.
    /// Defaults to deactivated (0). Any other positive value activates terms boosting with the given boost factor.
    pub fn boost_terms(mut self, boost_terms: impl Into<f64>) -> Self {
        self.inner.boost_terms = Some(boost_terms.into());
        self
    }

    /// Specifies whether the input documents should also be included in the search results returned. Defaults to `false`.
    pub fn include(mut self, include: impl Into<bool>) -> Self {
        self.inner.include = Some(include.into());
        self
    }

    add_boost_and_name!();
}

impl ShouldSkip for MoreLikeThisQuery {
    fn should_skip(&self) -> bool {
        self.inner.like.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod string {
        use super::*;

        test_serialization! {
            with_required_fields(
                MoreLikeThisQuery::new(["test"])
                    .fields(["title"]),
                json!({
                    "more_like_this": {
                        "fields": ["title"],
                        "like": [
                            "test"
                        ]
                    }
                })
            );

            with_optional_fields(
                MoreLikeThisQuery::new(["test"])
                    .fields(["title", "description"])
                    .min_term_freq(1)
                    .max_query_terms(12)
                    .boost(1.2)
                    .name("more_like_this"),
                json!({
                    "more_like_this": {
                        "fields": ["title", "description"],
                        "like": [
                            "test"
                        ],
                        "min_term_freq": 1,
                        "max_query_terms": 12,
                        "boost": 1.2,
                        "_name": "more_like_this"
                    }
                })
            );
        }
    }

    mod document {
        use super::*;

        test_serialization! {
            with_required_fields(
                MoreLikeThisQuery::new([Document::new("123")])
                    .fields(["title"]),
                json!({
                    "more_like_this": {
                        "fields": ["title"],
                        "like": [
                            {
                                "_id": "123"
                            }
                        ]
                    }
                })
            );

            with_optional_fields(
                MoreLikeThisQuery::new([Document::new("123")])
                    .fields(["title", "description"])
                    .min_term_freq(1)
                    .max_query_terms(12)
                    .boost(1.2)
                    .name("more_like_this"),
                json!({
                    "more_like_this": {
                        "fields": ["title", "description"],
                        "like": [
                            {
                                "_id": "123"
                            }
                        ],
                        "min_term_freq": 1,
                        "max_query_terms": 12,
                        "boost": 1.2,
                        "_name": "more_like_this"
                    }
                })
            );
        }
    }

    mod both {
        use super::*;

        test_serialization! {
            with_required_fields(
                MoreLikeThisQuery::new([Like::from(Document::new("123")), Like::from("test")])
                    .fields(["title"]),
                json!({
                    "more_like_this": {
                        "fields": ["title"],
                        "like": [
                            {
                                "_id": "123"
                            },
                            "test"
                        ]
                    }
                })
            );

            with_optional_fields(
                MoreLikeThisQuery::new([Like::from(Document::new("123").index("test_index").routing("test_routing").source(false)), Like::from("test")])
                    .fields(["title", "description"])
                    .min_term_freq(1)
                    .max_query_terms(12)
                    .boost(1.2)
                    .name("more_like_this"),
                json!({
                    "more_like_this": {
                        "fields": ["title", "description"],
                        "like": [
                            {
                                "_id": "123",
                                "_index": "test_index",
                                "_routing": "test_routing",
                                "_source": false
                            },
                            "test"
                        ],
                        "min_term_freq": 1,
                        "max_query_terms": 12,
                        "boost": 1.2,
                        "_name": "more_like_this"
                    }
                })
            );
        }
    }
}
