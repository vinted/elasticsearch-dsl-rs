//! A container type for boost values

use std::{cmp::Ordering, convert::TryFrom, fmt};

const ERROR_MSG: &str = "Boost value cannot be negative";

/// A container type for boost values
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub struct Boost(Inner);

impl fmt::Display for Boost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Inner::U64(value) => value.fmt(f),
            Inner::F32(value) => value.fmt(f),
            Inner::F64(value) => value.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
enum Inner {
    U64(u64),
    F32(f32),
    F64(f64),
}

// i8

impl TryFrom<i8> for Boost {
    type Error = &'static str;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(ERROR_MSG)
        } else {
            Ok(Self(Inner::U64(value as u64)))
        }
    }
}

impl PartialEq<i8> for Boost {
    fn eq(&self, other: &i8) -> bool {
        match self.0 {
            Inner::U64(value) => value.eq(&(*other as u64)),
            Inner::F32(value) => value.eq(&(*other as f32)),
            Inner::F64(value) => value.eq(&(*other as f64)),
        }
    }
}

impl PartialOrd<i8> for Boost {
    fn partial_cmp(&self, other: &i8) -> Option<Ordering> {
        match self.0 {
            Inner::U64(value) => {
                if other < &0 {
                    Some(Ordering::Greater)
                } else {
                    value.partial_cmp(&(*other as u64))
                }
            }
            Inner::F32(value) => value.partial_cmp(&(*other as f32)),
            Inner::F64(value) => value.partial_cmp(&(*other as f64)),
        }
    }
}

// i16

impl TryFrom<i16> for Boost {
    type Error = &'static str;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(ERROR_MSG)
        } else {
            Ok(Self(Inner::U64(value as u64)))
        }
    }
}

impl PartialEq<i16> for Boost {
    fn eq(&self, other: &i16) -> bool {
        match self.0 {
            Inner::U64(value) => value.eq(&(*other as u64)),
            Inner::F32(value) => value.eq(&(*other as f32)),
            Inner::F64(value) => value.eq(&(*other as f64)),
        }
    }
}

impl PartialOrd<i16> for Boost {
    fn partial_cmp(&self, other: &i16) -> Option<Ordering> {
        match self.0 {
            Inner::U64(value) => {
                if other < &0 {
                    Some(Ordering::Greater)
                } else {
                    value.partial_cmp(&(*other as u64))
                }
            }
            Inner::F32(value) => value.partial_cmp(&(*other as f32)),
            Inner::F64(value) => value.partial_cmp(&(*other as f64)),
        }
    }
}

// i32

impl TryFrom<i32> for Boost {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(ERROR_MSG)
        } else {
            Ok(Self(Inner::U64(value as u64)))
        }
    }
}

impl PartialEq<i32> for Boost {
    fn eq(&self, other: &i32) -> bool {
        match self.0 {
            Inner::U64(value) => value.eq(&(*other as u64)),
            Inner::F32(value) => value.eq(&(*other as f32)),
            Inner::F64(value) => value.eq(&(*other as f64)),
        }
    }
}

impl PartialOrd<i32> for Boost {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        match self.0 {
            Inner::U64(value) => {
                if other < &0 {
                    Some(Ordering::Greater)
                } else {
                    value.partial_cmp(&(*other as u64))
                }
            }
            Inner::F32(value) => value.partial_cmp(&(*other as f32)),
            Inner::F64(value) => value.partial_cmp(&(*other as f64)),
        }
    }
}

// i64

impl TryFrom<i64> for Boost {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(ERROR_MSG)
        } else {
            Ok(Self(Inner::U64(value as u64)))
        }
    }
}

impl PartialEq<i64> for Boost {
    fn eq(&self, other: &i64) -> bool {
        match self.0 {
            Inner::U64(value) => value.eq(&(*other as u64)),
            Inner::F32(value) => value.eq(&(*other as f32)),
            Inner::F64(value) => value.eq(&(*other as f64)),
        }
    }
}

impl PartialOrd<i64> for Boost {
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        match self.0 {
            Inner::U64(value) => {
                if other < &0 {
                    Some(Ordering::Greater)
                } else {
                    value.partial_cmp(&(*other as u64))
                }
            }
            Inner::F32(value) => value.partial_cmp(&(*other as f32)),
            Inner::F64(value) => value.partial_cmp(&(*other as f64)),
        }
    }
}

// u8

impl From<u8> for Boost {
    fn from(value: u8) -> Self {
        Self(Inner::U64(value as u64))
    }
}

impl PartialEq<u8> for Boost {
    fn eq(&self, other: &u8) -> bool {
        match self.0 {
            Inner::U64(value) => value.eq(&(*other as u64)),
            Inner::F32(value) => value.eq(&(*other as f32)),
            Inner::F64(value) => value.eq(&(*other as f64)),
        }
    }
}

impl PartialOrd<u8> for Boost {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        match self.0 {
            Inner::U64(value) => value.partial_cmp(&(*other as u64)),
            Inner::F32(value) => value.partial_cmp(&(*other as f32)),
            Inner::F64(value) => value.partial_cmp(&(*other as f64)),
        }
    }
}

// u16

impl From<u16> for Boost {
    fn from(value: u16) -> Self {
        Self(Inner::U64(value as u64))
    }
}

impl PartialEq<u16> for Boost {
    fn eq(&self, other: &u16) -> bool {
        match self.0 {
            Inner::U64(value) => value.eq(&(*other as u64)),
            Inner::F32(value) => value.eq(&(*other as f32)),
            Inner::F64(value) => value.eq(&(*other as f64)),
        }
    }
}

impl PartialOrd<u16> for Boost {
    fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
        match self.0 {
            Inner::U64(value) => value.partial_cmp(&(*other as u64)),
            Inner::F32(value) => value.partial_cmp(&(*other as f32)),
            Inner::F64(value) => value.partial_cmp(&(*other as f64)),
        }
    }
}

// u32

impl From<u32> for Boost {
    fn from(value: u32) -> Self {
        Self(Inner::U64(value as u64))
    }
}

impl PartialEq<u32> for Boost {
    fn eq(&self, other: &u32) -> bool {
        match self.0 {
            Inner::U64(value) => value.eq(&(*other as u64)),
            Inner::F32(value) => value.eq(&(*other as f32)),
            Inner::F64(value) => value.eq(&(*other as f64)),
        }
    }
}

impl PartialOrd<u32> for Boost {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        match self.0 {
            Inner::U64(value) => value.partial_cmp(&(*other as u64)),
            Inner::F32(value) => value.partial_cmp(&(*other as f32)),
            Inner::F64(value) => value.partial_cmp(&(*other as f64)),
        }
    }
}

// u64

impl From<u64> for Boost {
    fn from(value: u64) -> Self {
        Self(Inner::U64(value))
    }
}

impl PartialEq<u64> for Boost {
    fn eq(&self, other: &u64) -> bool {
        match self.0 {
            Inner::U64(value) => value.eq(other),
            Inner::F32(value) => value.eq(&(*other as f32)),
            Inner::F64(value) => value.eq(&(*other as f64)),
        }
    }
}

impl PartialOrd<u64> for Boost {
    fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
        match self.0 {
            Inner::U64(value) => value.partial_cmp(other),
            Inner::F32(value) => value.partial_cmp(&(*other as f32)),
            Inner::F64(value) => value.partial_cmp(&(*other as f64)),
        }
    }
}

// f32

impl TryFrom<f32> for Boost {
    type Error = &'static str;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value < 0. {
            Err(ERROR_MSG)
        } else {
            Ok(Self(Inner::F32(value)))
        }
    }
}

impl PartialEq<f32> for Boost {
    fn eq(&self, other: &f32) -> bool {
        match self.0 {
            Inner::U64(value) => (value as f32).eq(other),
            Inner::F32(value) => value.eq(other),
            Inner::F64(value) => (value as f32).eq(other),
        }
    }
}

impl PartialOrd<f32> for Boost {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        match self.0 {
            Inner::U64(value) => (value as f32).partial_cmp(other),
            Inner::F32(value) => value.partial_cmp(other),
            Inner::F64(value) => (value as f32).partial_cmp(other),
        }
    }
}

// f64

impl TryFrom<f64> for Boost {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < 0. {
            Err(ERROR_MSG)
        } else {
            Ok(Self(Inner::F64(value)))
        }
    }
}

impl PartialEq<f64> for Boost {
    fn eq(&self, other: &f64) -> bool {
        match self.0 {
            Inner::U64(value) => (value as f64).eq(other),
            Inner::F32(value) => value.eq(&(*other as f32)),
            Inner::F64(value) => value.eq(other),
        }
    }
}

impl PartialOrd<f64> for Boost {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        match self.0 {
            Inner::U64(value) => (value as f64).partial_cmp(other),
            Inner::F32(value) => value.partial_cmp(&(*other as f32)),
            Inner::F64(value) => value.partial_cmp(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds() {
        assert!(Boost::try_from(-1_i8).is_err());
        assert!(Boost::try_from(-1_i16).is_err());
        assert!(Boost::try_from(-1_i32).is_err());
        assert!(Boost::try_from(-1_i64).is_err());
        assert!(Boost::try_from(-1_f32).is_err());
        assert!(Boost::try_from(-1_f64).is_err());
    }

    #[test]
    fn within_bounds() {
        assert!(Boost::try_from(1_i8).unwrap() == 1);
        assert!(Boost::try_from(1_i16).unwrap() == 1);
        assert!(Boost::try_from(1_i32).unwrap() == 1);
        assert!(Boost::try_from(1_i64).unwrap() == 1);
        assert!(Boost::try_from(1_u8).unwrap() == 1);
        assert!(Boost::try_from(1_u16).unwrap() == 1);
        assert!(Boost::try_from(1_u32).unwrap() == 1);
        assert!(Boost::try_from(1_u64).unwrap() == 1);
        assert!(Boost::try_from(1_f32).unwrap() == 1);
        assert!(Boost::try_from(1_f64).unwrap() == 1);
    }

    #[test]
    fn partial_ord() {
        assert!(Boost::try_from(2_i8).unwrap() > 1);
        assert!(Boost::try_from(2_i16).unwrap() > 1);
        assert!(Boost::try_from(2_i32).unwrap() > 1);
        assert!(Boost::try_from(2_i64).unwrap() > 1);
        assert!(Boost::try_from(2_u8).unwrap() > 1);
        assert!(Boost::try_from(2_u16).unwrap() > 1);
        assert!(Boost::try_from(2_u32).unwrap() > 1);
        assert!(Boost::try_from(2_u64).unwrap() > 1);
        assert!(Boost::try_from(2_f32).unwrap() > 1);
        assert!(Boost::try_from(2_f64).unwrap() > 1);

        assert!(Boost::try_from(2_i8).unwrap() > 1.);
        assert!(Boost::try_from(2_i16).unwrap() > 1.);
        assert!(Boost::try_from(2_i32).unwrap() > 1.);
        assert!(Boost::try_from(2_i64).unwrap() > 1.);
        assert!(Boost::try_from(2_u8).unwrap() > 1.);
        assert!(Boost::try_from(2_u16).unwrap() > 1.);
        assert!(Boost::try_from(2_u32).unwrap() > 1.);
        assert!(Boost::try_from(2_u64).unwrap() > 1.);
        assert!(Boost::try_from(2_f32).unwrap() > 1.);
        assert!(Boost::try_from(2_f64).unwrap() > 1.);
    }
}
