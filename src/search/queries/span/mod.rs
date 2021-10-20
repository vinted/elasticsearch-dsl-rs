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
