//! You can specify a `runtime_mappings` section in a search request to create
//! runtime fields that exist only as part of the query. You specify a script
//! as part of the `runtime_mappings` section, just as you would if
//! [adding a runtime field to the mappings](https://www.elastic.co/guide/en/elasticsearch/reference/master/runtime-mapping-fields.html).
//!
//! Defining a runtime field in a search request uses the same format as
//! defining a runtime field in the index mapping. Just copy the field
//! definition from the `runtime_mappings` in the search request to the
//! `runtime` section of the index mapping.
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/master/runtime-search-request.html>

use serde::ser::{Serialize, SerializeStruct, Serializer};

/// A runtime data type that is used in a search request.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RuntimeDataType {
    /// Boolean
    Boolean,

    /// Composite
    Composite,

    /// Date with optional `format`
    Date(Option<String>),

    /// Double
    Double,

    /// Geo point
    GeoPoint,

    /// IP address
    Ip,

    /// Keyword
    Keyword,

    /// Long
    Long,
}

impl Serialize for RuntimeDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Boolean => {
                let mut state = serializer.serialize_struct("RuntimeDataType_Boolean", 1)?;
                state.serialize_field("type", "boolean")?;
                state.end()
            }
            Self::Composite => {
                let mut state = serializer.serialize_struct("RuntimeDataType_Boolean", 1)?;
                state.serialize_field("type", "composite")?;
                state.end()
            }
            Self::Date(format) => match format {
                Some(format) => {
                    let mut state = serializer.serialize_struct("RuntimeDataType_Boolean", 2)?;
                    state.serialize_field("type", "date")?;
                    state.serialize_field("format", format)?;
                    state.end()
                }
                None => {
                    let mut state = serializer.serialize_struct("RuntimeDataType_Boolean", 1)?;
                    state.serialize_field("type", "date")?;
                    state.end()
                }
            },
            Self::Double => {
                let mut state = serializer.serialize_struct("RuntimeDataType_Boolean", 1)?;
                state.serialize_field("type", "double")?;
                state.end()
            }
            Self::GeoPoint => {
                let mut state = serializer.serialize_struct("RuntimeDataType_Boolean", 1)?;
                state.serialize_field("type", "geo_point")?;
                state.end()
            }
            Self::Ip => {
                let mut state = serializer.serialize_struct("RuntimeDataType_Boolean", 1)?;
                state.serialize_field("type", "ip")?;
                state.end()
            }
            Self::Keyword => {
                let mut state = serializer.serialize_struct("RuntimeDataType_Boolean", 1)?;
                state.serialize_field("type", "keyword")?;
                state.end()
            }
            Self::Long => {
                let mut state = serializer.serialize_struct("RuntimeDataType_Boolean", 1)?;
                state.serialize_field("type", "long")?;
                state.end()
            }
        }
    }
}

/// A runtime field that is used in a search request.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct RuntimeMapping {
    #[serde(flatten)]
    r#type: RuntimeDataType,
    script: RuntimeScript,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
struct RuntimeScript {
    source: String,
}

impl RuntimeMapping {
    /// Creates a new instance of [RuntimeField]
    pub fn new<T>(r#type: RuntimeDataType, source: T) -> Self
    where
        T: ToString,
    {
        RuntimeMapping {
            r#type,
            script: RuntimeScript {
                source: source.to_string(),
            },
        }
    }

    /// Creates a new instance of [RuntimeDataType::Boolean] [RuntimeField]
    pub fn boolean<T>(source: T) -> Self
    where
        T: ToString,
    {
        Self::new(RuntimeDataType::Boolean, source)
    }

    /// Creates a new instance of [RuntimeDataType::Composite] [RuntimeField]
    pub fn composite<T>(source: T) -> Self
    where
        T: ToString,
    {
        Self::new(RuntimeDataType::Composite, source)
    }

    /// Creates a new instance of [RuntimeDataType::Date] [RuntimeField] without format
    pub fn date<T>(source: T) -> Self
    where
        T: ToString,
    {
        Self::new(RuntimeDataType::Date(None), source)
    }

    /// Creates a new instance of [RuntimeDataType::Date] [RuntimeField] with format
    pub fn date_format<F, T>(format: F, source: T) -> Self
    where
        F: ToString,
        T: ToString,
    {
        Self::new(RuntimeDataType::Date(Some(format.to_string())), source)
    }

    /// Creates a new instance of [RuntimeDataType::Double] [RuntimeField]
    pub fn double<T>(source: T) -> Self
    where
        T: ToString,
    {
        Self::new(RuntimeDataType::Double, source)
    }

    /// Creates a new instance of [RuntimeDataType::GeoPoint] [RuntimeField]
    pub fn geo_point<T>(source: T) -> Self
    where
        T: ToString,
    {
        Self::new(RuntimeDataType::GeoPoint, source)
    }

    /// Creates a new instance of [RuntimeDataType::Ip] [RuntimeField]
    pub fn ip<T>(source: T) -> Self
    where
        T: ToString,
    {
        Self::new(RuntimeDataType::Ip, source)
    }

    /// Creates a new instance of [RuntimeDataType::Keyword] [RuntimeField]
    pub fn keyword<T>(source: T) -> Self
    where
        T: ToString,
    {
        Self::new(RuntimeDataType::Keyword, source)
    }

    /// Creates a new instance of [RuntimeDataType::Long] [RuntimeField]
    pub fn long<T>(source: T) -> Self
    where
        T: ToString,
    {
        Self::new(RuntimeDataType::Long, source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn serialization() {
        assert_serialize(
            RuntimeMapping::boolean("doc['field'].value"),
            json!({
                "type": "boolean",
                "script": { "source": "doc['field'].value" }
            }),
        );

        assert_serialize(
            RuntimeMapping::composite("doc['field'].value"),
            json!({
                "type": "composite",
                "script": { "source": "doc['field'].value" }
            }),
        );

        assert_serialize(
            RuntimeMapping::date("doc['field'].value"),
            json!({
                "type": "date",
                "script": { "source": "doc['field'].value" }
            }),
        );

        assert_serialize(
            RuntimeMapping::date_format("YYYY-mm-dd", "doc['field'].value"),
            json!({
                "type": "date",
                "format": "YYYY-mm-dd",
                "script": { "source": "doc['field'].value" }
            }),
        );

        assert_serialize(
            RuntimeMapping::double("doc['field'].value"),
            json!({
                "type": "double",
                "script": { "source": "doc['field'].value" }
            }),
        );

        assert_serialize(
            RuntimeMapping::geo_point("doc['field'].value"),
            json!({
                "type": "geo_point",
                "script": { "source": "doc['field'].value" }
            }),
        );

        assert_serialize(
            RuntimeMapping::ip("doc['field'].value"),
            json!({
                "type": "ip",
                "script": { "source": "doc['field'].value" }
            }),
        );

        assert_serialize(
            RuntimeMapping::keyword("doc['field'].value"),
            json!({
                "type": "keyword",
                "script": { "source": "doc['field'].value" }
            }),
        );

        assert_serialize(
            RuntimeMapping::long("doc['field'].value"),
            json!({
                "type": "long",
                "script": { "source": "doc['field'].value" }
            }),
        );
    }
}
