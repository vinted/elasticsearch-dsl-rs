//! Suggests similar looking terms based on a provided text by using a suggester.
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-suggesters.html>

mod completion_suggester;
mod suggest_context_query;
mod suggest_fuzziness;
mod suggester;

pub use self::completion_suggester::*;
pub use self::suggest_context_query::*;
pub use self::suggest_fuzziness::*;
pub use self::suggester::*;
