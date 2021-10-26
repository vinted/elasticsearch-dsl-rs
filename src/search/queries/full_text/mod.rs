//! The full text queries enable you to search analyzed text fields such as the body of an email.
//! The query string is processed using the same analyzer that was applied to the field during
//! indexing.
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/current/full-text-queries.html>

mod combined_fields_query;
mod match_bool_prefix_query;
mod match_phrase_prefix_query;
mod match_phrase_query;
mod match_query;
mod multi_match_query;
mod query_string_query;
mod simple_query_string_query;

pub use self::combined_fields_query::*;
pub use self::match_bool_prefix_query::*;
pub use self::match_phrase_prefix_query::*;
pub use self::match_phrase_query::*;
pub use self::match_query::*;
pub use self::multi_match_query::*;
pub use self::query_string_query::*;
pub use self::simple_query_string_query::*;
