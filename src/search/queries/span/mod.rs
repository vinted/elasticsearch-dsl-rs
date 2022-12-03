//! Span queries are low-level positional queries which provide expert control over the order and
//! proximity of the specified terms. These are typically used to implement very specific queries
//! on legal documents or patents.
//!
//! It is only allowed to set boost on an outer span query. Compound span queries, like span_near,
//! only use the list of matching spans of inner span queries in order to find their own spans,
//! which they then use to produce a score. Scores are never computed on inner span queries, which
//! is the reason why boosts are not allowed: they only influence the way scores are computed, not
//! spans.
//!
//! Span queries cannot be mixed with non-span queries (with the exception of the span_multi query).

mod span_containing_query;
mod span_field_masking_query;
mod span_first_query;
mod span_multi_query;
mod span_near_query;
mod span_not_query;
mod span_or_query;
mod span_term_query;
mod span_within_query;

pub use self::span_containing_query::*;
pub use self::span_field_masking_query::*;
pub use self::span_first_query::*;
pub use self::span_multi_query::*;
pub use self::span_near_query::*;
pub use self::span_not_query::*;
pub use self::span_or_query::*;
pub use self::span_term_query::*;
pub use self::span_within_query::*;

use crate::util::*;
use crate::{FuzzyQuery, PrefixQuery, RangeQuery, RegexpQuery, WildcardQuery};

macro_rules! span_query {
    ($($variant:ident($query:ty)),+ $(,)?) => {
        /// A container enum for supported Elasticsearch query types
        #[derive(Clone, PartialEq, Serialize)]
        #[serde(untagged)]
        #[allow(missing_docs)]
        pub enum SpanQuery {
            $(
                $variant($query),
            )*
        }

        impl IntoIterator for SpanQuery {
            type Item = Self;
            type IntoIter = std::option::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                if self.should_skip() {
                    None.into_iter()
                } else {
                    Some(self).into_iter()
                }
            }
        }

        impl ShouldSkip for SpanQuery {
            fn should_skip(&self) -> bool {
                match self {
                    $(
                        Self::$variant(q) => q.should_skip(),
                    )+
                }
            }
        }

        impl std::fmt::Debug for SpanQuery {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variant(q) => q.fmt(f),
                    )+
                }
            }
        }

        $(
            impl From<$query> for SpanQuery {
                fn from(q: $query) -> Self {
                    SpanQuery::$variant(q)
                }
            }

            impl PartialEq<$query> for SpanQuery {
                fn eq(&self, other: &$query) -> bool {
                    match self {
                        Self::$variant(query) => query.eq(other),
                        _ => false,
                    }
                }
            }

            impl PartialEq<SpanQuery> for $query {
                fn eq(&self, other: &SpanQuery) -> bool {
                    match other {
                        SpanQuery::$variant(query) => self.eq(query),
                        _ => false,
                    }
                }
            }

            impl From<$query> for Option<SpanQuery> {
                fn from(q: $query) -> Self {
                    if q.should_skip() {
                        None
                    } else {
                        Some(SpanQuery::$variant(q))
                    }
                }
            }
        )+
    };
}

macro_rules! multi_term_query {
    ($($variant:ident($query:ty)),+ $(,)?) => {
        /// A container enum for supported Elasticsearch query types
        #[derive(Clone, PartialEq, Serialize)]
        #[serde(untagged)]
        #[allow(missing_docs)]
        pub enum MultiTermQuery {
            $(
                $variant($query),
            )*
        }

        impl IntoIterator for MultiTermQuery {
            type Item = Self;
            type IntoIter = std::option::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                if self.should_skip() {
                    None.into_iter()
                } else {
                    Some(self).into_iter()
                }
            }
        }

        impl ShouldSkip for MultiTermQuery {
            fn should_skip(&self) -> bool {
                match self {
                    $(
                        Self::$variant(q) => q.should_skip(),
                    )+
                }
            }
        }

        impl std::fmt::Debug for MultiTermQuery {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variant(q) => q.fmt(f),
                    )+
                }
            }
        }

        $(
            impl From<$query> for MultiTermQuery {
                fn from(q: $query) -> Self {
                    MultiTermQuery::$variant(q)
                }
            }

            impl PartialEq<$query> for MultiTermQuery {
                fn eq(&self, other: &$query) -> bool {
                    match self {
                        Self::$variant(query) => query.eq(other),
                        _ => false,
                    }
                }
            }

            impl PartialEq<MultiTermQuery> for $query {
                fn eq(&self, other: &MultiTermQuery) -> bool {
                    match other {
                        MultiTermQuery::$variant(query) => self.eq(query),
                        _ => false,
                    }
                }
            }

            impl From<$query> for Option<MultiTermQuery> {
                fn from(q: $query) -> Self {
                    if q.should_skip() {
                        None
                    } else {
                        Some(MultiTermQuery::$variant(q))
                    }
                }
            }
        )+
    };
}

span_query!(
    SpanContaining(SpanContainingQuery),
    SpanFieldMasking(SpanFieldMaskingQuery),
    SpanFirst(SpanFirstQuery),
    SpanMulti(SpanMultiQuery),
    SpanNear(SpanNearQuery),
    SpanNot(SpanNotQuery),
    SpanOr(SpanOrQuery),
    SpanTerm(SpanTermQuery),
    SpanWithin(SpanWithinQuery),
);

multi_term_query!(
    Prefix(PrefixQuery),
    Regexp(RegexpQuery),
    Wildcard(WildcardQuery),
    Range(RangeQuery),
    Fuzzy(FuzzyQuery),
);
