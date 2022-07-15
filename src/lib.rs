//! # Strongly typed Elasticsearch DSL written in Rust
//!
//! This is an unofficial library and doesn't yet support all the DSL, it's still work in progress.
//!
//! ## Features
//!
//! - Strongly typed queries
//! - Strongly typed aggregations
//! - Automatically skips empty queries making DSL pleasant to use
//! - Crate doesn't depend on [elasticsearch-rs](https://github.com/elastic/elasticsearch-rs) and can be used as a standalone library with any HTTP client to call Elasticsearch
//!
//! ## Installation
//!
//! Add `elasticsearch-dsl` crate and version to Cargo.toml
//!
//! ```toml
//! [dependencies]
//! elasticsearch-dsl = "0.2"
//! ```
//!
//! ## Documentation
//!
//! Documentation for the library is available on [docs.rs](https://docs.rs/elasticsearch-dsl)
//!
//! ## Quick start
//!
//! ```rust
//! use elasticsearch_dsl::*;
//!
//! fn main() {
//!     let query = Search::new()
//!         .source(false)
//!         .stats("statistics")
//!         .from(0)
//!         .size(30)
//!         .query(
//!             Query::bool()
//!                 .must(Query::multi_match(
//!                     ["title", "description"],
//!                     "you know, for search",
//!                 ))
//!                 .filter(Query::terms("tags", ["elasticsearch"]))
//!                 .should(Query::term("verified", true).boost(10)),
//!         )
//!         .aggregate(
//!             "country_ids",
//!             Aggregation::terms("country_id")
//!                 .aggregate("catalog_ids", Aggregation::terms("catalog_id"))
//!                 .aggregate("company_ids", Aggregation::terms("company_id"))
//!                 .aggregate(
//!                     "top1",
//!                     Aggregation::top_hits()
//!                         .size(1)
//!                         .sort(FieldSort::ascending("user_id")),
//!                 ),
//!         ).rescore(Rescore::new(Query::term("field", 1)).query_weight(1.2));
//! }
//! ```
//!
//! See examples for more.
//!
//! #### License
//!
//! <sup>
//! Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
//! 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
//! </sup>
#![doc(
    html_logo_url = "https://play-lh.googleusercontent.com/VvtT2Dvf_oOC3DL4c2i5hfNvwIqzdU2apScRMlmeRW10Yf-vJXnXqAjdNWE9KW5YvK0"
)]
#![deny(
    bad_style,
    const_err,
    dead_code,
    deprecated,
    improper_ctypes,
    missing_debug_implementations,
    missing_docs,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unknown_lints,
    unreachable_code,
    unreachable_pub,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_extern_crates,
    unused_import_braces,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_results,
    warnings,
    while_true
)]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate serde_json;

// Macro modules
#[macro_use]
mod macros;

// Crate modules
#[macro_use]
pub(crate) mod util;

// Public modules
pub mod analyze;
pub mod search;

// Public re-exports
pub use self::analyze::*;
pub use self::search::*;
