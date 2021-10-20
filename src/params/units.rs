use serde::ser::{Serialize, Serializer};

/// Whenever durations need to be specified, e.g. for a `timeout` parameter,
/// the duration must specify the unit, like `2d` for 2 days.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#time-units>
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(missing_docs)]
pub enum Time {
    Days(u64),
    Hours(u64),
    Minutes(u64),
    Seconds(u64),
    Milliseconds(u64),
    Microseconds(u64),
    Nanoseconds(u64),
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Days(u) => format!("{}d", u),
            Self::Hours(u) => format!("{}h", u),
            Self::Minutes(u) => format!("{}m", u),
            Self::Seconds(u) => format!("{}s", u),
            Self::Milliseconds(u) => format!("{}ms", u),
            Self::Microseconds(u) => format!("{}micros", u),
            Self::Nanoseconds(u) => format!("{}nanos", u),
        }
        .serialize(serializer)
    }
}

/// Whenever the byte size of data needs to be specified, e.g. when setting a
/// buffer size parameter, the value must specify the unit,
/// like `10kb` for 10 kilobytes.
/// Note that these units use powers of 1024, so `1kb` means 1024 bytes.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#byte-units>
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(missing_docs)]
pub enum Byte {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
    Petabytes(u64),
}

impl Serialize for Byte {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bytes(u) => format!("{}b", u),
            Self::Kilobytes(u) => format!("{}kb", u),
            Self::Megabytes(u) => format!("{}mb", u),
            Self::Gigabytes(u) => format!("{}gb", u),
            Self::Terabytes(u) => format!("{}tb", u),
            Self::Petabytes(u) => format!("{}pb", u),
        }
        .serialize(serializer)
    }
}

/// Unit-less quantities means that they don’t have a "unit"
/// like "bytes" or "Hertz" or "meter" or "long tonne".
///
/// If one of these quantities is large we’ll print it out like 10m for
/// 10,000,000 or 7k for 7,000. We’ll still print 87 when we mean 87 though.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#size-units>
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(missing_docs)]
pub enum Size {
    Kilo(u64),
    Mega(u64),
    Giga(u64),
    Tera(u64),
    Peta(u64),
}

impl Serialize for Size {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Kilo(u) => format!("{}k", u),
            Self::Mega(u) => format!("{}m", u),
            Self::Giga(u) => format!("{}g", u),
            Self::Tera(u) => format!("{}t", u),
            Self::Peta(u) => format!("{}p", u),
        }
        .serialize(serializer)
    }
}

/// Wherever distances need to be specified, such as the `distance` parameter
/// in the
/// [Geo-distance](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-geo-distance-query.html)
/// ), the default unit is meters if none is specified.
/// Distances can be specified in other units,
/// such as `"1km"` or `"2mi"` (2 miles).
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-geo-distance-query.html>
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(missing_docs)]
pub enum Distance {
    Miles(u64),
    Yards(u64),
    Feet(u64),
    Inches(u64),
    Kilometers(u64),
    Meters(u64),
    Centimeter(u64),
    Millimeters(u64),
    NauticalMiles(u64),
}

impl Serialize for Distance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Miles(u) => format!("{}mi", u),
            Self::Yards(u) => format!("{}yd", u),
            Self::Feet(u) => format!("{}ft", u),
            Self::Inches(u) => format!("{}in", u),
            Self::Kilometers(u) => format!("{}km", u),
            Self::Meters(u) => format!("{}m", u),
            Self::Centimeter(u) => format!("{}cm", u),
            Self::Millimeters(u) => format!("{}mm", u),
            Self::NauticalMiles(u) => format!("{}nmi", u),
        }
        .serialize(serializer)
    }
}
