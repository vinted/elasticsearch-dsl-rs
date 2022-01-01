use serde::Serialize;
use std::fmt::Display;

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
}
