/// Numeric enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Number {
    /// Represents the type of u8, u16, u32, u64
    U64(u64),
    /// Represents the type of i8, i16, i32, i64
    I64(i64),
    /// Represents the type of f32
    F32(f32),
    /// Represents the type of f64
    F64(f64),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::U64(value) => value.fmt(f),
            Number::I64(value) => value.fmt(f),
            Number::F32(value) => value.fmt(f),
            Number::F64(value) => value.fmt(f),
        }
    }
}

impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Self::U64(value as u64)
    }
}

impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Self::U64(value as u64)
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Self::U64(value as u64)
    }
}

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<i8> for Number {
    fn from(value: i8) -> Self {
        Self::I64(value as i64)
    }
}

impl From<i16> for Number {
    fn from(value: i16) -> Self {
        Self::I64(value as i64)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::I64(value as i64)
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}
