//! You can use **term-level queries** to find documents based on precise values in structured data.
//! Examples of structured data include date ranges, IP addresses, prices, or product IDs.
//!
//! Unlike full-text queries, term-level queries do not analyze search terms. Instead, term-level
//! queries match the exact terms stored in a field.

mod exists_query;
mod fuzzy_query;
mod ids_query;
mod prefix_query;
mod range_query;
mod regexp_query;
mod term_query;
mod terms_query;
mod terms_set_query;
mod wildcard_query;

pub use self::exists_query::*;
pub use self::fuzzy_query::*;
pub use self::ids_query::*;
pub use self::prefix_query::*;
pub use self::range_query::*;
pub use self::regexp_query::*;
pub use self::term_query::*;
pub use self::terms_query::*;
pub use self::terms_set_query::*;
pub use self::wildcard_query::*;
