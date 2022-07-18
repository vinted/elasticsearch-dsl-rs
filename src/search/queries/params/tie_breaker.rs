/// Floating point number between `0` and `1.0` used to increase the
/// [relevance scores](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores)
/// of documents matching multiple query clauses. Defaults to `0.0`.
///
/// You can use the `tie_breaker` value to assign higher relevance scores to
/// documents that contain the same term in multiple fields than documents that
/// contain this term in only the best of those multiple fields, without
/// confusing this with the better case of two different terms in the multiple
/// fields.
///
/// If a document matches multiple clauses, the `dis_max` query calculates
/// the relevance score for the document as follows:
/// 1. Take the relevance score from a matching clause with the highest score.
/// 2. Multiply the score from any other matching clauses by the tie_breaker value.
/// 3. Add the highest score to the multiplied scores.
///
/// If the `tie_breaker` value is greater than `0.0`, all matching clauses
/// count, but the clause with the highest score counts most.
#[derive(Clone, Copy, PartialEq, Serialize)]
pub struct TieBreaker(f32);

impl std::fmt::Debug for TieBreaker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl TieBreaker {
    /// Minimum value
    const MINIMUM: f32 = 0f32;

    /// Maximum value
    const MAXIMUM: f32 = 1f32;

    /// Creates a valid instance of [TieBreaker]
    pub fn new(tie_breaker: f32) -> Option<Self> {
        debug_assert!(
            (Self::MINIMUM..=Self::MAXIMUM).contains(&tie_breaker),
            "Tie breaker must bet between 0 and 1"
        );

        Some(Self(tie_breaker))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn none_when_negative() {
        let _ = TieBreaker::new(-0.1);
    }

    #[test]
    #[should_panic]
    fn none_when_more_than_1() {
        let _ = TieBreaker::new(1.1);
    }

    #[test]
    fn some_when_within_range() {
        assert_eq!(TieBreaker::new(0.5), Some(TieBreaker(0.5)));
    }
}
