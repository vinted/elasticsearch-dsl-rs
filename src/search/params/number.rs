use std::cmp::Ordering;

/// Numeric enum
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number(N);

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
enum N {
    /// Non-negative integers
    Pos(u64),

    /// Negative integers
    Neg(i64),

    /// 32-bit floats
    F32(f32),

    /// 64-bit floats
    F64(f64),
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            N::Pos(value) => value.fmt(f),
            N::Neg(value) => value.fmt(f),
            N::F32(value) => value.fmt(f),
            N::F64(value) => value.fmt(f),
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            N::Pos(value) => value.fmt(f),
            N::Neg(value) => value.fmt(f),
            N::F32(value) => value.fmt(f),
            N::F64(value) => value.fmt(f),
        }
    }
}

impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Self(N::Pos(value as u64))
    }
}

impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Self(N::Pos(value as u64))
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Self(N::Pos(value as u64))
    }
}

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Self(N::Pos(value))
    }
}

impl From<i8> for Number {
    fn from(value: i8) -> Self {
        if value < 0 {
            Self(N::Neg(value as i64))
        } else {
            Self(N::Pos(value as u64))
        }
    }
}

impl From<i16> for Number {
    fn from(value: i16) -> Self {
        if value < 0 {
            Self(N::Neg(value as i64))
        } else {
            Self(N::Pos(value as u64))
        }
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        if value < 0 {
            Self(N::Neg(value as i64))
        } else {
            Self(N::Pos(value as u64))
        }
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        if value < 0 {
            Self(N::Neg(value))
        } else {
            Self(N::Pos(value as u64))
        }
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Self(N::F32(value))
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self(N::F64(value))
    }
}

impl From<&u8> for Number {
    fn from(value: &u8) -> Self {
        Self(N::Pos(*value as u64))
    }
}

impl From<&u16> for Number {
    fn from(value: &u16) -> Self {
        Self(N::Pos(*value as u64))
    }
}

impl From<&u32> for Number {
    fn from(value: &u32) -> Self {
        Self(N::Pos(*value as u64))
    }
}

impl From<&u64> for Number {
    fn from(value: &u64) -> Self {
        Self(N::Pos(*value))
    }
}

impl From<&i8> for Number {
    fn from(value: &i8) -> Self {
        let value = *value;
        if value < 0 {
            Self(N::Neg(value as i64))
        } else {
            Self(N::Pos(value as u64))
        }
    }
}

impl From<&i16> for Number {
    fn from(value: &i16) -> Self {
        let value = *value;
        if value < 0 {
            Self(N::Neg(value as i64))
        } else {
            Self(N::Pos(value as u64))
        }
    }
}

impl From<&i32> for Number {
    fn from(value: &i32) -> Self {
        let value = *value;
        if value < 0 {
            Self(N::Neg(value as i64))
        } else {
            Self(N::Pos(value as u64))
        }
    }
}

impl From<&i64> for Number {
    fn from(value: &i64) -> Self {
        let value = *value;
        if value < 0 {
            Self(N::Neg(value))
        } else {
            Self(N::Pos(value as u64))
        }
    }
}

impl From<&f32> for Number {
    fn from(value: &f32) -> Self {
        Self(N::F32(*value))
    }
}

impl From<&f64> for Number {
    fn from(value: &f64) -> Self {
        Self(N::F64(*value))
    }
}

impl PartialEq for N {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Positive
            (N::Pos(value), N::Pos(other)) => value.eq(other),
            (N::Pos(_), N::Neg(_)) => false,
            (N::Pos(value), N::F32(other)) => (*value as f32).eq(other),
            (N::Pos(value), N::F64(other)) => (*value as f64).eq(other),

            // Negative
            (N::Neg(_), N::Pos(_)) => false,
            (N::Neg(value), N::Neg(other)) => value.eq(other),
            (N::Neg(value), N::F32(other)) => (*value as f32).eq(other),
            (N::Neg(value), N::F64(other)) => (*value as f64).eq(other),

            // 32-bit floats
            (N::F32(value), N::Pos(other)) => value.eq(&(*other as f32)),
            (N::F32(value), N::Neg(other)) => value.eq(&(*other as f32)),
            (N::F32(value), N::F32(other)) => value.eq(other),
            (N::F32(value), N::F64(other)) => value.eq(&(*other as f32)),

            // 64-bit floats
            (N::F64(value), N::Pos(other)) => value.eq(&(*other as f64)),
            (N::F64(value), N::Neg(other)) => value.eq(&(*other as f64)),
            (N::F64(value), N::F32(other)) => (*value as f32).eq(other),
            (N::F64(value), N::F64(other)) => value.eq(other),
        }
    }
}

impl Eq for N {}

impl PartialOrd for N {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            // Positive
            (N::Pos(value), N::Pos(other)) => value.partial_cmp(other),
            (N::Pos(_), N::Neg(_)) => Some(Ordering::Greater),
            (N::Pos(value), N::F32(other)) => (*value as f32).partial_cmp(other),
            (N::Pos(value), N::F64(other)) => (*value as f64).partial_cmp(other),

            // Negative
            (N::Neg(_), N::Pos(_)) => Some(Ordering::Less),
            (N::Neg(value), N::Neg(other)) => value.partial_cmp(other),
            (N::Neg(value), N::F32(other)) => (*value as f32).partial_cmp(other),
            (N::Neg(value), N::F64(other)) => (*value as f64).partial_cmp(other),

            // 32-bit floats
            (N::F32(value), N::Pos(other)) => value.partial_cmp(&(*other as f32)),
            (N::F32(value), N::Neg(other)) => value.partial_cmp(&(*other as f32)),
            (N::F32(value), N::F32(other)) => value.partial_cmp(other),
            (N::F32(value), N::F64(other)) => value.partial_cmp(&(*other as f32)),

            // 64-bit floats
            (N::F64(value), N::Pos(other)) => value.partial_cmp(&(*other as f64)),
            (N::F64(value), N::Neg(other)) => value.partial_cmp(&(*other as f64)),
            (N::F64(value), N::F32(other)) => (*value as f32).partial_cmp(other),
            (N::F64(value), N::F64(other)) => value.partial_cmp(other),
        }
    }
}

impl Ord for N {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Less)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn serialization() {
        assert_serialize(
            [
                Number::from(2),
                Number::from(-2),
                Number::from(2.2f32),
                Number::from(2.2f64),
            ],
            json!([2, -2, 2.2, 2.2]),
        )
    }

    #[test]
    fn partial_eq() {
        assert_eq!(Number::from(2f32), Number::from(2));
        assert_eq!(Number::from(2f64), Number::from(2));
        assert_eq!(Number::from(2), Number::from(2));
        assert_eq!(Number::from(-2), Number::from(-2));
    }

    #[test]
    fn partial_ord() {
        assert!(Number::from(2) > Number::from(1));
        assert!(Number::from(2) > Number::from(-1));
        assert!(Number::from(2) > Number::from(1f32));
        assert!(Number::from(2) > Number::from(1f64));

        assert!(Number::from(-2) < Number::from(1));
        assert!(Number::from(-2) < Number::from(-1));
        assert!(Number::from(-2) < Number::from(1f32));
        assert!(Number::from(-2) < Number::from(1f64));

        assert!(Number::from(2f32) > Number::from(1));
        assert!(Number::from(2f32) > Number::from(-1));
        assert!(Number::from(2f32) > Number::from(1f32));
        assert!(Number::from(2f32) > Number::from(1f64));

        assert!(Number::from(2f64) > Number::from(1));
        assert!(Number::from(2f64) > Number::from(-1));
        assert!(Number::from(2f64) > Number::from(1f32));
        assert!(Number::from(2f64) > Number::from(1f64));
    }
}
