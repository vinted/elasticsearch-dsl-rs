use crate::params::*;
use crate::util::*;
use chrono::{DateTime, Utc};
use std::time::SystemTime;

/// Leaf term value
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Term(Option<Inner>);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(untagged)]
enum Inner {
    /// Boolean value
    Bool(bool),

    /// String value
    String(String),

    /// Number value
    Number(Number),

    /// Date
    Date(Date),
}

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => value.fmt(f),
            None => "None".fmt(f),
        }
    }
}

impl std::fmt::Debug for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(value) => value.fmt(f),
            Self::String(value) => value.fmt(f),
            Self::Number(value) => value.fmt(f),
            Self::Date(value) => value.fmt(f),
        }
    }
}

impl From<bool> for Term {
    fn from(value: bool) -> Self {
        Self(Some(Inner::Bool(value)))
    }
}

impl From<String> for Term {
    fn from(value: String) -> Self {
        Self(Some(Inner::String(value)))
    }
}

impl From<&str> for Term {
    fn from(value: &str) -> Self {
        Self(Some(Inner::String(value.into())))
    }
}

impl From<i8> for Term {
    fn from(value: i8) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<i16> for Term {
    fn from(value: i16) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<i32> for Term {
    fn from(value: i32) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<i64> for Term {
    fn from(value: i64) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<u8> for Term {
    fn from(value: u8) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<u16> for Term {
    fn from(value: u16) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<u32> for Term {
    fn from(value: u32) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<u64> for Term {
    fn from(value: u64) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<f32> for Term {
    fn from(value: f32) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<f64> for Term {
    fn from(value: f64) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<SystemTime> for Term {
    fn from(value: SystemTime) -> Self {
        Self(Some(Inner::Date(Date::from(value))))
    }
}

impl From<DateTime<Utc>> for Term {
    fn from(value: DateTime<Utc>) -> Self {
        Self(Some(Inner::Date(Date::from(value))))
    }
}

impl From<&i8> for Term {
    fn from(value: &i8) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<&i16> for Term {
    fn from(value: &i16) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<&i32> for Term {
    fn from(value: &i32) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<&i64> for Term {
    fn from(value: &i64) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<&u8> for Term {
    fn from(value: &u8) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<&u16> for Term {
    fn from(value: &u16) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<&u32> for Term {
    fn from(value: &u32) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<&u64> for Term {
    fn from(value: &u64) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<&f32> for Term {
    fn from(value: &f32) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<&f64> for Term {
    fn from(value: &f64) -> Self {
        Self(Some(Inner::Number(Number::from(value))))
    }
}

impl From<&SystemTime> for Term {
    fn from(value: &SystemTime) -> Self {
        Self(Some(Inner::Date(Date::from(value))))
    }
}

impl From<&DateTime<Utc>> for Term {
    fn from(value: &DateTime<Utc>) -> Self {
        Self(Some(Inner::Date(Date::from(value))))
    }
}

impl From<Option<bool>> for Term {
    fn from(value: Option<bool>) -> Self {
        Self(value.map(Inner::Bool))
    }
}

impl From<Option<String>> for Term {
    fn from(value: Option<String>) -> Self {
        Self(value.map(Inner::String))
    }
}

impl From<Option<&str>> for Term {
    fn from(value: Option<&str>) -> Self {
        Self(value.map(Into::into).map(Inner::String))
    }
}

impl From<Option<i8>> for Term {
    fn from(value: Option<i8>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<i16>> for Term {
    fn from(value: Option<i16>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<i32>> for Term {
    fn from(value: Option<i32>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<i64>> for Term {
    fn from(value: Option<i64>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<u8>> for Term {
    fn from(value: Option<u8>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<u16>> for Term {
    fn from(value: Option<u16>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<u32>> for Term {
    fn from(value: Option<u32>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<u64>> for Term {
    fn from(value: Option<u64>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<f32>> for Term {
    fn from(value: Option<f32>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<f64>> for Term {
    fn from(value: Option<f64>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<SystemTime>> for Term {
    fn from(value: Option<SystemTime>) -> Self {
        Self(value.map(Date::from).map(Inner::Date))
    }
}

impl From<Option<DateTime<Utc>>> for Term {
    fn from(value: Option<DateTime<Utc>>) -> Self {
        Self(value.map(Date::from).map(Inner::Date))
    }
}

impl From<Option<&i8>> for Term {
    fn from(value: Option<&i8>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<&i16>> for Term {
    fn from(value: Option<&i16>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<&i32>> for Term {
    fn from(value: Option<&i32>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<&i64>> for Term {
    fn from(value: Option<&i64>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<&u8>> for Term {
    fn from(value: Option<&u8>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<&u16>> for Term {
    fn from(value: Option<&u16>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<&u32>> for Term {
    fn from(value: Option<&u32>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<&u64>> for Term {
    fn from(value: Option<&u64>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<&f32>> for Term {
    fn from(value: Option<&f32>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<&f64>> for Term {
    fn from(value: Option<&f64>) -> Self {
        Self(value.map(Number::from).map(Inner::Number))
    }
}

impl From<Option<&SystemTime>> for Term {
    fn from(value: Option<&SystemTime>) -> Self {
        Self(value.map(Date::from).map(Inner::Date))
    }
}

impl From<Option<&DateTime<Utc>>> for Term {
    fn from(value: Option<&DateTime<Utc>>) -> Self {
        Self(value.map(Date::from).map(Inner::Date))
    }
}

impl ShouldSkip for Term {
    fn should_skip(&self) -> bool {
        match &self.0 {
            None => true,
            Some(Inner::String(value)) => value.should_skip(),
            _ => false,
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
            (Term::from(true), Term::from(true)),
            (Term::from("a"), Term::from("a")),
            (Term::from(16), Term::from(16)),
            (Term::from(-1), Term::from(-1)),
            (Term::from(1f32), Term::from(1f32)),
            (Term::from(1f32), Term::from(1f64)),
            (Term::from(1f64), Term::from(1f32)),
            (Term::from(1f64), Term::from(1f64)),
            (
                Term::from(Utc.ymd(2021, 3, 10).and_hms(10, 42, 0)),
                Term::from(Utc.ymd(2021, 3, 10).and_hms(10, 42, 0)),
            ),
        ];

        for (left, right) in values {
            assert_eq!(left, right);
        }
    }
}
