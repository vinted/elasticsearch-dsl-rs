use crate::ShouldSkip;
use chrono::{DateTime, Utc};
use std::{cmp::Ordering, convert::TryFrom};

/// An enum type for leaf level queries such as terms, range, values
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Scalar {
    /// Boolean value
    Bool(bool),

    /// String value
    String(String),

    /// Signed integer value
    SignedInteger(i64),

    /// Unsigned integer value
    UnsignedInteger(u64),

    /// Floating 32-bit point value
    Float32(f32),

    /// Floating 64-bit point value
    Float64(f64),

    /// DateTime
    DateTime(DateTime<Utc>),
}

/// Optional scalar newtype
///
/// Needed for custom From implementations
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct OptionalScalar(Option<Scalar>);

fn try_eq<L, R>(left: &L, right: &R) -> bool
where
    L: TryFrom<R> + PartialEq + Copy,
    R: PartialEq + Copy,
{
    L::try_from(*right).map_or(false, |r| left.eq(&r))
}

impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Scalar::Bool(s), Scalar::Bool(o)) => s.eq(o),
            (Scalar::String(s), Scalar::String(o)) => s.eq(o),
            (Scalar::SignedInteger(s), Scalar::SignedInteger(o)) => s.eq(o),
            (Scalar::SignedInteger(s), Scalar::UnsignedInteger(o)) => try_eq(s, o),
            (Scalar::UnsignedInteger(s), Scalar::SignedInteger(o)) => try_eq(s, o),
            (Scalar::UnsignedInteger(s), Scalar::UnsignedInteger(o)) => s.eq(o),
            (Scalar::Float32(s), Scalar::Float32(o)) => s.eq(o),
            (Scalar::Float32(s), Scalar::Float64(o)) => try_eq(o, s),
            (Scalar::Float64(s), Scalar::Float32(o)) => try_eq(s, o),
            (Scalar::Float64(s), Scalar::Float64(o)) => s.eq(o),
            (Scalar::DateTime(s), Scalar::DateTime(o)) => s.eq(o),
            _ => false,
        }
    }
}

impl Eq for Scalar {}

impl PartialOrd for Scalar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self, other) {
            (Self::Bool(s), Self::Bool(o)) => s.partial_cmp(o),
            (Self::String(s), Self::String(o)) => s.partial_cmp(o),
            (Self::SignedInteger(s), Self::SignedInteger(o)) => s.partial_cmp(o),
            (Self::UnsignedInteger(s), Self::UnsignedInteger(o)) => s.partial_cmp(o),
            (Self::Float32(s), Self::Float32(o)) => s.partial_cmp(o),
            (Self::Float64(s), Self::Float64(o)) => s.partial_cmp(o),
            (Self::DateTime(s), Self::DateTime(o)) => s.partial_cmp(o),
            _ => Some(Ordering::Less),
        }
    }
}

impl Ord for Scalar {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self, other) {
            (Self::Bool(s), Self::Bool(o)) => s.cmp(o),
            (Self::String(s), Self::String(o)) => s.cmp(o),
            (Self::SignedInteger(s), Self::SignedInteger(o)) => s.cmp(o),
            (Self::UnsignedInteger(s), Self::UnsignedInteger(o)) => s.cmp(o),
            (Self::Float32(s), Self::Float32(o)) => s.partial_cmp(o).unwrap_or(Ordering::Less),
            (Self::Float64(s), Self::Float64(o)) => s.partial_cmp(o).unwrap_or(Ordering::Less),
            (Self::DateTime(s), Self::DateTime(o)) => s.cmp(o),
            _ => Ordering::Less,
        }
    }
}

impl From<bool> for Scalar {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<String> for Scalar {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Scalar {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<i8> for Scalar {
    fn from(value: i8) -> Self {
        Self::SignedInteger(value as i64)
    }
}

impl From<i16> for Scalar {
    fn from(value: i16) -> Self {
        Self::SignedInteger(value as i64)
    }
}

impl From<i32> for Scalar {
    fn from(value: i32) -> Self {
        Self::SignedInteger(value as i64)
    }
}

impl From<i64> for Scalar {
    fn from(value: i64) -> Self {
        Self::SignedInteger(value)
    }
}

impl From<u8> for Scalar {
    fn from(value: u8) -> Self {
        Self::UnsignedInteger(value as u64)
    }
}

impl From<u16> for Scalar {
    fn from(value: u16) -> Self {
        Self::UnsignedInteger(value as u64)
    }
}

impl From<u32> for Scalar {
    fn from(value: u32) -> Self {
        Self::UnsignedInteger(value as u64)
    }
}

impl From<u64> for Scalar {
    fn from(value: u64) -> Self {
        Self::UnsignedInteger(value)
    }
}

impl From<f32> for Scalar {
    fn from(value: f32) -> Self {
        Self::Float32(value)
    }
}

impl From<f64> for Scalar {
    fn from(value: f64) -> Self {
        Self::Float64(value)
    }
}

impl From<DateTime<Utc>> for Scalar {
    fn from(value: DateTime<Utc>) -> Self {
        Self::DateTime(value)
    }
}

impl<T> From<T> for OptionalScalar
where
    T: Into<Scalar>,
{
    fn from(value: T) -> Self {
        Self(Some(value.into()))
    }
}

impl<T> From<Option<T>> for OptionalScalar
where
    T: Into<Scalar>,
{
    fn from(value: Option<T>) -> Self {
        Self(value.map(Into::into))
    }
}

impl ShouldSkip for Scalar {
    fn should_skip(&self) -> bool {
        match self {
            Self::String(ref s) => s.should_skip(),
            _ => false,
        }
    }
}

impl ShouldSkip for OptionalScalar {
    fn should_skip(&self) -> bool {
        match &self.0 {
            Some(value) => value.should_skip(),
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn partial_equality() {
        let values = vec![
            (Scalar::Bool(true), Scalar::Bool(true)),
            (Scalar::String("a".into()), Scalar::String("a".into())),
            (Scalar::SignedInteger(16), Scalar::SignedInteger(16)),
            (Scalar::SignedInteger(16), Scalar::UnsignedInteger(16)),
            (Scalar::UnsignedInteger(16), Scalar::SignedInteger(16)),
            (Scalar::UnsignedInteger(16), Scalar::UnsignedInteger(16)),
            (Scalar::Float32(1.), Scalar::Float32(1.)),
            (Scalar::Float32(1.), Scalar::Float64(1.)),
            (Scalar::Float64(1.), Scalar::Float32(1.)),
            (Scalar::Float64(1.), Scalar::Float64(1.)),
            (
                Scalar::DateTime(Utc.ymd(2021, 3, 10).and_hms(10, 42, 0)),
                Scalar::DateTime(Utc.ymd(2021, 3, 10).and_hms(10, 42, 0)),
            ),
        ];

        for (left, right) in values {
            assert_eq!(left, right);
        }
    }
}
