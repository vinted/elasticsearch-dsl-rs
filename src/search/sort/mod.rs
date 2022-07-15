//! Allows you to add one or more sorts on specific fields.
//! Each sort can be reversed as well.
//! The sort is defined on a per field level, with special field name for `_score` to sort by score, and `_doc` to sort by index order.
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/master/search-your-data.html>

mod field_sort;
mod sort_;
mod sort_collection;
mod sort_missing;
mod sort_mode;
mod sort_order;
mod sort_special_field;

pub use self::field_sort::*;
pub use self::sort_::*;
pub use self::sort_collection::*;
pub use self::sort_missing::*;
pub use self::sort_mode::*;
pub use self::sort_order::*;
pub use self::sort_special_field::*;
