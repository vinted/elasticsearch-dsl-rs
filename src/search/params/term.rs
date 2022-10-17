use serde::ser::{self, Serialize};

/// Elasticsearch term value
#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum Term {
    /// Boolean value
    Boolean(bool),

    /// Positive only integer number
    PositiveNumber(u64),

    /// Negative only integer number
    NegativeNumber(i64),

    /// 32-bit floating point number, separate from 64-bit to not lose precision
    Float32(f32),

    /// 64-bit floating point number, separate from 32-bit to not lose precision
    Float64(f64),

    /// A string value
    String(String),
}

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(term) => term.fmt(f),
            Self::PositiveNumber(term) => term.fmt(f),
            Self::NegativeNumber(term) => term.fmt(f),
            Self::Float32(term) => term.fmt(f),
            Self::Float64(term) => term.fmt(f),
            Self::String(term) => term.fmt(f),
        }
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::PositiveNumber(l0), Self::PositiveNumber(r0)) => l0 == r0,
            (Self::NegativeNumber(l0), Self::NegativeNumber(r0)) => l0 == r0,
            (Self::Float32(l0), Self::Float32(r0)) => l0 == r0,
            (Self::Float64(l0), Self::Float64(r0)) => l0 == r0,
            (Self::Float32(l0), Self::Float64(r0)) => l0 == &(*r0 as f32),
            (Self::Float64(l0), Self::Float32(r0)) => &(*l0 as f32) == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Term {
    /// Creates a new term from a serializable value
    pub fn new<T>(term: T) -> Option<Self>
    where
        T: Serialize,
    {
        let term = term.serialize(Serializer);

        debug_assert!(term.is_ok() || term == Err(TermSerializeError::NoTerm));

        term.ok()
    }
}

struct Serializer;

impl ser::Serializer for Serializer {
    type Ok = Term;
    type Error = TermSerializeError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Term::Boolean(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        if v < 0 {
            Ok(Term::NegativeNumber(v))
        } else {
            Ok(Term::PositiveNumber(v as u64))
        }
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Term::PositiveNumber(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Term::Float32(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Term::Float64(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let v = String::from(v);

        if v.is_empty() {
            Err(TermSerializeError::NoTerm)
        } else {
            Ok(Term::String(v))
        }
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let v = String::from(v);

        if v.is_empty() {
            Err(TermSerializeError::NoTerm)
        } else {
            Ok(Term::String(v))
        }
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(TermSerializeError::NoTerm)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(TermSerializeError::NotTerm)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }
}

impl ser::SerializeSeq for Serializer {
    type Ok = Term;
    type Error = TermSerializeError;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(TermSerializeError::NotTerm)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }
}

impl ser::SerializeTuple for Serializer {
    type Ok = Term;
    type Error = TermSerializeError;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(TermSerializeError::NotTerm)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }
}

impl ser::SerializeTupleStruct for Serializer {
    type Ok = Term;
    type Error = TermSerializeError;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(TermSerializeError::NotTerm)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }
}
impl ser::SerializeTupleVariant for Serializer {
    type Ok = Term;
    type Error = TermSerializeError;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(TermSerializeError::NotTerm)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }
}

impl ser::SerializeMap for Serializer {
    type Ok = Term;
    type Error = TermSerializeError;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(TermSerializeError::NotTerm)
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(TermSerializeError::NotTerm)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }
}

impl ser::SerializeStruct for Serializer {
    type Ok = Term;
    type Error = TermSerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(TermSerializeError::NotTerm)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }
}

impl ser::SerializeStructVariant for Serializer {
    type Ok = Term;
    type Error = TermSerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        Err(TermSerializeError::NotTerm)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(TermSerializeError::NotTerm)
    }
}

/// Term conversion error
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum TermSerializeError {
    /// No term provided,
    NoTerm,

    /// Provided value was not a term
    NotTerm,
}

impl std::fmt::Display for TermSerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoTerm => "no term was provided".fmt(f),
            Self::NotTerm => "provided value was not a term".fmt(f),
        }
    }
}

impl std::error::Error for TermSerializeError {}

impl serde::ser::Error for TermSerializeError {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::NotTerm
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn serializes_primitives_correctly() {
        assert_eq!(Term::new(true), Some(Term::Boolean(true)));
        assert_eq!(Term::new(12345), Some(Term::PositiveNumber(12345)));
        assert_eq!(Term::new(-1234), Some(Term::NegativeNumber(-1234)));
        assert_eq!(Term::new(1_f32), Some(Term::Float32(1.0)));
        assert_eq!(Term::new(1_f64), Some(Term::Float64(1.0)));
        assert_eq!(Term::new('s'), Some(Term::String("s".into())));
        assert_eq!(Term::new("str"), Some(Term::String("str".into())));
        assert_eq!(
            Term::new(Utc.ymd(2022, 3, 21).and_hms(0, 5, 8)),
            Some(Term::String("2022-03-21T00:05:08Z".into()))
        );
    }

    #[test]
    fn serializes_newtypes_correctly() {
        #[derive(Serialize)]
        struct Newtype<T>(T);

        assert_eq!(Term::new(Newtype(123)), Some(Term::PositiveNumber(123)));
    }

    #[test]
    fn serializes_wrappers_correctly() {
        struct Wrapper<T> {
            value: T,
        }

        impl<T> Serialize for Wrapper<T>
        where
            T: Serialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.value.serialize(serializer)
            }
        }

        assert_eq!(
            Term::new(Wrapper { value: 123 }),
            Some(Term::PositiveNumber(123))
        );
    }

    #[test]
    fn custom_partial_eq() {
        assert_eq!(Term::Float32(1.0), Term::Float64(1.0));
        assert_eq!(Term::Float64(1.0), Term::Float32(1.0));
    }
}
