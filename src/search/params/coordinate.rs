use serde::Serialize;
use std::fmt::Display;

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

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        serializes_coordinate(
            Coordinate::new(1.1, 2.2),
            json!([1.1, 2.2])
        );

        serializes_coordinate_from_array(
            Coordinate::from([1.1, 2.2]),
            json!([1.1, 2.2])
        );

        serializes_coordinate_from_tuple(
            Coordinate::from((1.1, 2.2)),
            json!([1.1, 2.2])
        );
    }
}
