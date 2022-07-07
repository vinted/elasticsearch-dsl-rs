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
mod span_field_masking;
mod span_first;
mod span_multi;
mod span_near;
mod span_not;
mod span_or;
mod span_term;
mod span_within;

pub use span_containing_query::*;
pub use span_field_masking::*;
pub use span_first::*;
pub use span_multi::*;
pub use span_near::*;
pub use span_not::*;
pub use span_or::*;
pub use span_term::*;
pub use span_within::*;
use crate::Query;
use crate::util::ShouldSkip;

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

impl Into<Query> for SpanQuery {
    fn into(self) -> Query {
        match self {
            SpanQuery::SpanContaining(q) => Query::SpanContaining(q),
            SpanQuery::SpanFieldMasking(q) => Query::SpanFieldMasking(q),
            SpanQuery::SpanFirst(q) => Query::SpanFirst(q),
            SpanQuery::SpanMulti(q) => Query::SpanMulti(q),
            SpanQuery::SpanNear(q) => Query::SpanNear(q),
            SpanQuery::SpanNot(q) => Query::SpanNot(q),
            SpanQuery::SpanOr(q) => Query::SpanOr(q),
            SpanQuery::SpanTerm(q) => Query::SpanTerm(q),
            SpanQuery::SpanWithin(q) => Query::SpanWithin(q),
        }
    }
}
