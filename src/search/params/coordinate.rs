use serde::Serialize;
use std::{error::Error, fmt::Display, num::ParseFloatError, str::FromStr};

/// An error which can be returned when parsing a coordinate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseCoordinateError {
    /// Coordinate is invalid
    ///
    /// This will occur when coordinate cannot be parsed as [`f32`]
    Invalid(ParseFloatError),

    /// Coordinate is missing
    ///
    /// This will occur when fewer than two coordinates are passed
    Missing,

    /// Coordinate is redundant
    ///
    /// This will occur when more than two coordinates are passed
    Redundant,
}

impl Display for ParseCoordinateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(e) => e.fmt(f),
            Self::Missing => "fewer than two coordinates are passed".fmt(f),
            Self::Redundant => "more than two coordinates are passed".fmt(f),
        }
    }
}

impl Error for ParseCoordinateError {}

/// Represents a point in two dimensional space
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Coordinate {
    x: f32,
    y: f32,
}

impl Coordinate {
    /// Creates an instance of [`Coordinate`]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("[{}, {}]", &self.x, &self.y).fmt(f)
    }
}

impl Serialize for Coordinate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        [self.x, self.y].serialize(serializer)
    }
}

impl From<[f32; 2]> for Coordinate {
    fn from(value: [f32; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

impl From<(f32, f32)> for Coordinate {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl FromStr for Coordinate {
    type Err = ParseCoordinateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(',');

        let x = values
            .next()
            .ok_or(Self::Err::Missing)?
            .trim()
            .parse()
            .map_err(Self::Err::Invalid)?;

        let y = values
            .next()
            .ok_or(Self::Err::Missing)?
            .trim()
            .parse()
            .map_err(Self::Err::Invalid)?;

        match values.next() {
            Some(_) => Err(Self::Err::Redundant),
            None => Ok(Self { x, y }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn serialization() {
        assert_serialize(Coordinate::new(1.1, 2.2), json!([1.1, 2.2]));
        assert_serialize(Coordinate::from([1.1, 2.2]), json!([1.1, 2.2]));
        assert_serialize(Coordinate::from((1.1, 2.2)), json!([1.1, 2.2]));
    }

    #[test]
    fn from_str() {
        assert_eq!(
            Coordinate::from_str("1.1, 2.2").unwrap(),
            Coordinate::new(1.1, 2.2)
        );
        assert_eq!(
            Coordinate::from_str("1,2").unwrap(),
            Coordinate::new(1., 2.)
        );

        assert!(Coordinate::from_str("").is_err());
        assert!(Coordinate::from_str("1.1").is_err());
        assert!(Coordinate::from_str("1.1,2.2,3").is_err());
        assert!(Coordinate::from_str("1.1,abc").is_err());
        assert!(Coordinate::from_str("abc").is_err());
    }
}
