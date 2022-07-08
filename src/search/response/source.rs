use crate::util::ShouldSkip;
use serde::de::DeserializeOwned;
use serde_json::value::RawValue;

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
