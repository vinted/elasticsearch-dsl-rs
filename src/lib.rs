//! A strongly typed query DSL that maps 1 to 1 with the Elasticsearch query DSL.
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

// Private modules
#[macro_use]
mod macros;
mod response;
mod ser;
mod util;

// Public modules
pub mod aggregations;
pub mod analyze;
pub mod highlight;
pub mod params;
pub mod queries;
pub mod rescoring;
pub mod search;
pub mod sort;

// Crate re-exports
pub(crate) use self::ser::*;
pub(crate) use self::util::*;

// Public re-exports
pub use self::aggregations::*;
pub use self::analyze::*;
pub use self::highlight::*;
pub use self::params::*;
pub use self::queries::params::*;
pub use self::queries::*;
pub use self::rescoring::*;
pub use self::response::*;
pub use self::search::*;
pub use self::sort::*;
