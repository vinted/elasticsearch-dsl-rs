use crate::util::ShouldSkip;
use serde::de::DeserializeOwned;
use serde_json::{value::RawValue, Value};

/// Document source with delayed deserialization
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Source(Box<RawValue>);

impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl ShouldSkip for Source {
    fn should_skip(&self) -> bool {
        self.eq(&Source::default())
    }
}

impl Source {
    /// Parses document source into a concrete type
    pub fn parse<T>(&self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        serde_json::from_str(self.0.get())
    }

    /// Creates source from a string
    pub fn from_string(value: String) -> Result<Self, serde_json::Error> {
        RawValue::from_string(value).map(Self)
    }
}

impl From<Value> for Source {
    /// Creates source from a [Value]
    ///
    /// Calling expect here because [Value] always represents a valid JSON and it
    /// _should be safe_ to do so.
    fn from(value: Value) -> Self {
        Self(RawValue::from_string(format!("{value}")).expect("valid json"))
    }
}

impl From<Box<RawValue>> for Source {
    fn from(value: Box<RawValue>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a RawValue> for Source {
    fn from(value: &'a RawValue) -> Self {
        Self(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_from_json_value() {
        let _ = Source::from(json!({"key": "value"}));
        let _ = Source::from(json!({"key": ["value", 1, 1.2, true, null, {"key2": "value2"}]}));
        let _ = Source::from(json!(["one", 2, 3.0, false, null, {}]));
    }
}
