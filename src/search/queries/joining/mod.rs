//! Performing full SQL-style joins in a distributed system like Elasticsearch is prohibitively
//! expensive. Instead, Elasticsearch offers two forms of join which are designed to scale
//! horizontally.
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/current/joining-queries.html>

mod has_child_query;
mod has_parent_query;
mod nested_query;
mod parent_id_query;

pub use self::has_child_query::*;
pub use self::has_parent_query::*;
pub use self::nested_query::*;
pub use self::parent_id_query::*;
