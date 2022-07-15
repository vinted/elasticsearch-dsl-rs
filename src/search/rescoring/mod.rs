//! Rescore clause to run second query over original one results and that way give more accuracy for final results
//! <https://www.elastic.co/guide/en/elasticsearch/reference/6.8/search-request-rescore.html>

mod rescore_;
mod rescore_collection;

pub use self::rescore_::*;
pub use self::rescore_collection::*;
