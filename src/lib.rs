#![doc = include_str!("../README.md")]
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

// Macro modules
#[macro_use]
mod macros;

// Crate modules
pub(crate) mod util;

// Public modules
pub mod analyze;
pub mod search;

// Public re-exports
pub use self::search::*;
