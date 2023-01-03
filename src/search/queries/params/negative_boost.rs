//! A container type for boost values
use std::{f32, fmt};

/// A container type for boost values
#[derive(Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub struct NegativeBoost(f32);

impl fmt::Debug for NegativeBoost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for NegativeBoost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl NegativeBoost {
    /// Minimum boost value
    const MINIMUM: f32 = 0f32;

    /// Maximum boost value
    const MAXIMUM: f32 = 1f32;

    /// Creates a new instance of a negative boost value
    ///
    /// Floating point number between `0` and `1.0` used to decrease the
    /// [relevance scores](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores)
    /// of documents matching the `negative` query.
    pub fn new(boost: f32) -> Self {
        Self(boost.clamp(Self::MINIMUM, Self::MAXIMUM))
    }
}

impl From<f32> for NegativeBoost {
    fn from(boost: f32) -> Self {
        Self::new(boost)
    }
}

impl From<i32> for NegativeBoost {
    fn from(boost: i32) -> Self {
        Self::new(boost as f32)
    }
}

impl PartialEq<f32> for NegativeBoost {
    fn eq(&self, other: &f32) -> bool {
        self.0.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds_integers() {
        let min: NegativeBoost = (-1).into();
        let max: NegativeBoost = 101.into();

        assert_eq!(min, NegativeBoost::new(0f32));
        assert_eq!(max, NegativeBoost::new(1f32));
    }

    #[test]
    fn out_of_bounds_floats() {
        let min: NegativeBoost = (-1.0).into();
        let max: NegativeBoost = 101.0.into();

        assert_eq!(min, NegativeBoost::new(0f32));
        assert_eq!(max, NegativeBoost::new(1f32));
    }

    #[test]
    fn within_bounds_floats() {
        let min: NegativeBoost = 0.01.into();
        let max: NegativeBoost = 0.99.into();

        assert_eq!(min, NegativeBoost::new(0.01));
        assert_eq!(max, NegativeBoost::new(0.99));
    }
}
