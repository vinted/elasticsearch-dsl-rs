//! Pipeline aggregations work on the outputs produced from other aggregations rather than from document sets, adding
//! information to the output tree. There are many different types of pipeline aggregation, each computing different information from
//! other aggregations, but these types can be broken down into two families:
//!
//! **Parent**
//! > A family of pipeline aggregations that is provided with the output of its parent aggregation and is able
//! to compute new buckets or new aggregations to add to existing buckets.
//!
//! **Sibling**
//! > Pipeline aggregations that are provided with the output of a sibling aggregation and are able to compute a
//! new aggregation which will be at the same level as the sibling aggregation.
//!
//! Pipeline aggregations can reference the aggregations they need to perform their computation by using the `buckets_path`
//! parameter to indicate the paths to the required metrics. The syntax for defining these paths can be found in the
//! <<buckets-path-syntax, `buckets_path` Syntax>> section below.
//!
//! Pipeline aggregations cannot have sub-aggregations but depending on the type it can reference another pipeline in the `buckets_path`
//! allowing pipeline aggregations to be chained.  For example, you can chain together two derivatives to calculate the second derivative
//! (i.e. a derivative of a derivative).
//!
//! > **NOTE**: Because pipeline aggregations only add to the output, when chaining pipeline aggregations the output of each pipeline aggregation
//! will be included in the final output.
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-pipeline.html>
