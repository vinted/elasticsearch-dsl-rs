use serde::Serialize;

/// Represents a point in two dimensional space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoLocation {
    latitude: f32,
    longitude: f32,
}

impl GeoLocation {
    /// Creates an instance of [GeoLocation]
    pub fn new(latitude: f32, longitude: f32) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

impl Serialize for GeoLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        [self.longitude, self.latitude].serialize(serializer)
    }
}

impl From<[f32; 2]> for GeoLocation {
    fn from(value: [f32; 2]) -> Self {
        Self {
            latitude: value[1],
            longitude: value[0],
        }
    }
}

impl From<(f32, f32)> for GeoLocation {
    fn from(value: (f32, f32)) -> Self {
        Self {
            latitude: value.1,
            longitude: value.0,
        }
    }
}

impl IntoIterator for GeoLocation {
    type Item = Self;

    type IntoIter = std::option::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Some(self).into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn serialization() {
        assert_serialize(GeoLocation::new(1.1, 2.2), json!([2.2, 1.1]));
        assert_serialize(GeoLocation::from([2.2, 1.1]), json!([2.2, 1.1]));
        assert_serialize(GeoLocation::from((2.2, 1.1)), json!([2.2, 1.1]));
    }
}
