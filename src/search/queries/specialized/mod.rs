//! This group contains queries which do not fit into the other groups

mod distance_feature_query;
mod more_like_this_query;
mod percolate_lookup_query;
mod percolate_query;
mod pinned_query;
mod rank_feature_query;

pub use self::distance_feature_query::*;
pub use self::more_like_this_query::*;
pub use self::percolate_lookup_query::*;
pub use self::percolate_query::*;
pub use self::pinned_query::*;
pub use self::rank_feature_query::*;
