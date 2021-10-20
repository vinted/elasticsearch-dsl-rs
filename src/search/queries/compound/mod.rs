//! Compound queries wrap other compound or leaf queries, either to combine their results and
//! scores, to change their behavior, or to switch from query to filter context.

mod bool_query;
mod boosting_query;
mod constant_score_query;
mod dis_max_query;
mod function_score_query;

pub use self::bool_query::*;
pub use self::boosting_query::*;
pub use self::constant_score_query::*;
pub use self::dis_max_query::*;
pub use self::function_score_query::*;
