use serde::Serialize;
use std::{error::Error, fmt::Display, num::ParseFloatError, str::FromStr};

/// An error which can be returned when parsing a coordinate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseGeoCoordinateError {
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

    /// Latitude is invalid
    ///
    /// This will occur when latitude isn't between -90 and 90, inclusive
    LatitudeInvalid,

    /// Longitude is invalid
    ///
    /// This will occur when longitude isn't between -180 and 180, inclusive.
    LongitudeInvalid,
}

impl Display for ParseGeoCoordinateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(e) => e.fmt(f),
            Self::Missing => "fewer than two coordinates are passed".fmt(f),
            Self::Redundant => "more than two coordinates are passed".fmt(f),
            Self::LatitudeInvalid => "latitude not between -90 and 90".fmt(f),
            Self::LongitudeInvalid => "longitude not between -180 and 180".fmt(f),
        }
    }
}

impl Error for ParseGeoCoordinateError {}

/// Represents a point in two dimensional space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoCoordinate {
    latitude: f32,
    longitude: f32,
}

impl GeoCoordinate {
    /// Creates an instance of [`GeoCoordinate`]
    pub fn new(latitude: f32, longitude: f32) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

impl Display for GeoCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("[{}, {}]", &self.longitude, &self.latitude).fmt(f)
    }
}

impl Serialize for GeoCoordinate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        [self.longitude, self.latitude].serialize(serializer)
    }
}

impl From<[f32; 2]> for GeoCoordinate {
    fn from(value: [f32; 2]) -> Self {
        Self {
            latitude: value[1],
            longitude: value[0],
        }
    }
}

impl From<(f32, f32)> for GeoCoordinate {
    fn from(value: (f32, f32)) -> Self {
        Self {
            latitude: value.1,
            longitude: value.0,
        }
    }
}

impl FromStr for GeoCoordinate {
    type Err = ParseGeoCoordinateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(',');

        let longitude = values
            .next()
            .ok_or(Self::Err::Missing)?
            .trim()
            .parse()
            .map_err(Self::Err::Invalid)?;

        if !(-180. ..=180.).contains(&longitude) {
            return Err(Self::Err::LongitudeInvalid);
        }

        let latitude = values
            .next()
            .ok_or(Self::Err::Missing)?
            .trim()
            .parse()
            .map_err(Self::Err::Invalid)?;

        if !(-90. ..=90.).contains(&latitude) {
            return Err(Self::Err::LatitudeInvalid);
        }

        match values.next() {
            Some(_) => Err(Self::Err::Redundant),
            None => Ok(Self {
                longitude,
                latitude,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn serialization() {
        assert_serialize(GeoCoordinate::new(1.1, 2.2), json!([2.2, 1.1]));
        assert_serialize(GeoCoordinate::from([2.2, 1.1]), json!([2.2, 1.1]));
        assert_serialize(GeoCoordinate::from((2.2, 1.1)), json!([2.2, 1.1]));
    }

    #[test]
    fn from_str() {
        assert_eq!(
            GeoCoordinate::from_str("1.1, 2.2").unwrap(),
            GeoCoordinate::new(2.2, 1.1)
        );
        assert_eq!(
            GeoCoordinate::from_str("1,2").unwrap(),
            GeoCoordinate::new(2., 1.)
        );

        assert!(GeoCoordinate::from_str("0,-91").is_err());
        assert!(GeoCoordinate::from_str("0,91").is_err());
        assert!(GeoCoordinate::from_str("-181,0").is_err());
        assert!(GeoCoordinate::from_str("181,0").is_err());
        assert!(GeoCoordinate::from_str("1.1").is_err());
        assert!(GeoCoordinate::from_str("1.1,2.2,3").is_err());
        assert!(GeoCoordinate::from_str("abc").is_err());
    }
}
