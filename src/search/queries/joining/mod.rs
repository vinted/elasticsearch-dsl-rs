//! Performing full SQL-style joins in a distributed system like Elasticsearch is prohibitively
//! expensive. Instead, Elasticsearch offers two forms of join which are designed to scale
//! horizontally.

mod nested_query;

pub use self::nested_query::*;
