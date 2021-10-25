//! Strongly typed Elasticsearch query params

// Common parameters
mod boost;
mod fuzziness;
mod has_child_query;
mod inner_hits;
mod negative_boost;
mod operator;
mod rewrite;
mod tie_breaker;
mod zero_terms_query;

// Query specific parameters
mod function_score_query;
mod multi_match_query;
mod nested_query;
mod percolate_query;
mod range_query;
mod regexp_query;
mod simple_query_string_query;
mod terms_query;
mod terms_set_query;

// Public re-exports
pub use self::boost::*;
pub use self::function_score_query::*;
pub use self::fuzziness::*;
pub use self::has_child_query::*;
pub use self::inner_hits::*;
pub use self::multi_match_query::*;
pub use self::negative_boost::*;
pub use self::nested_query::*;
pub use self::operator::*;
pub use self::percolate_query::*;
pub use self::range_query::*;
pub use self::regexp_query::*;
pub use self::rewrite::*;
pub use self::simple_query_string_query::*;
pub use self::terms_query::*;
pub use self::terms_set_query::*;
pub use self::tie_breaker::*;
pub use self::zero_terms_query::*;

/// The `minimum_should_match` type alias
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-minimum-should-match.html>
pub type MinimumShouldMatch = String;
