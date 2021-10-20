//! A container type for boost values

use std::{cmp::Ordering, f32, fmt};

/// A container type for boost values
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub struct Boost(f32);

impl fmt::Display for Boost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Boost {
    /// Minimum boost value
    const MINIMUM: f32 = 0f32;

    /// Creates a new instance of boost value
    ///
    /// Constraints boost to non-negative values only
    pub fn new(boost: f32) -> Self {
        let boost = f32::max(Self::MINIMUM, boost);

        Self(boost)
    }
}

impl From<f32> for Boost {
    fn from(boost: f32) -> Self {
        Self::new(boost)
    }
}

impl From<i32> for Boost {
    fn from(boost: i32) -> Self {
        Self::new(boost as f32)
    }
}

impl PartialEq<f32> for Boost {
    fn eq(&self, other: &f32) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<f32> for Boost {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        match (self.0 <= *other, self.0 >= *other) {
            (false, false) => None,
            (false, true) => Some(Ordering::Greater),
            (true, false) => Some(Ordering::Less),
            (true, true) => Some(Ordering::Equal),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds() {
        assert_eq!(Boost::new(-1.0), Boost::new(0.0));
    }

    #[test]
    fn within_bounds() {
        assert_eq!(Boost::new(1.23), Boost::new(1.23));
    }

    #[test]
    fn partial_eq_with_floats() {
        assert_eq!(Boost::new(1.2).partial_cmp(&1.2f32), Some(Ordering::Equal));
    }

    #[test]
    fn partial_ord_with_floats() {
        assert!(Boost::new(1.2) <= 1.3f32);
        assert!(Boost::new(1.2) >= 1.1f32);
    }
}
