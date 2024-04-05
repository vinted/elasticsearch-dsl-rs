//! Search APIs are used to search and aggregate data stored in Elasticsearch
//! indices and data streams. For an overview and related tutorials, see
//! [Search your data](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-your-data.html).
//!
//! Most search APIs support
//! [multi-target syntax](https://www.elastic.co/guide/en/elasticsearch/reference/current/multi-index.html),
//! with the exception of the
//! [explain API](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-explain.html).
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/current/search.html>

// Private modules
mod response;

// Public modules
pub mod aggregations;
pub mod highlight;
pub mod knn;
pub mod params;
pub mod queries;
pub mod request;
pub mod rescoring;
pub mod runtime_mappings;
pub mod script_fields;
pub mod sort;
pub mod suggesters;

// Public re-exports
pub use self::aggregations::*;
pub use self::highlight::*;
pub use self::knn::*;
pub use self::params::*;
pub use self::queries::params::*;
pub use self::queries::*;
pub use self::request::*;
pub use self::rescoring::*;
pub use self::response::*;
pub use self::runtime_mappings::*;
pub use self::script_fields::*;
pub use self::sort::*;
pub use self::suggesters::*;
