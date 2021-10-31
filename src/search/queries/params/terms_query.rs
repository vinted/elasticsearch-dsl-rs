use crate::search::*;
use crate::util::*;
use serde::ser::Serialize;
use std::{collections::BTreeSet, fmt::Debug};

/// Marker trait for [terms query](crate::TermsQuery) values
pub trait Terms: Debug + Serialize + Clone {}

impl<T: Into<Scalar> + Debug + Serialize + Clone + PartialOrd + Ord> Terms for BTreeSet<T> {}

impl Terms for TermsLookup {}

/// Terms lookup fetches the field values of an existing document.
/// Elasticsearch then uses those values as search terms. This can be
/// helpful when searching for a large set of terms.
///
/// Because terms lookup fetches values from a document, the
/// [`_source`](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-source-field.html)
/// mapping field must be enabled to use terms lookup. The `_source`
/// field is enabled by default.
///
/// > By default, Elasticsearch limits the `terms` query to a maximum of
/// 65,536 terms. This includes terms fetched using terms lookup. You can
/// change this limit using the
/// [`index.max_terms_count setting`](https://www.elastic.co/guide/en/elasticsearch/reference/current/index-modules.html#index-max-terms-count).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TermsLookup {
    pub(crate) index: String,

    pub(crate) id: String,

    pub(crate) path: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub(crate) routing: Option<String>,
}
