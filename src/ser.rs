use serde::ser::{Serialize, SerializeMap, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct KeyValuePair<T>
where
    T: Serialize,
{
    pub(crate) key: String,
    pub(crate) value: T,
}

impl<T> KeyValuePair<T>
where
    T: Serialize,
{
    /// Creates an instance of [`KeyValuePair`]
    pub(crate) fn new<S>(key: S, value: T) -> Self
    where
        S: Into<String>,
    {
        Self {
            key: key.into(),
            value,
        }
    }
}

impl<T> Serialize for KeyValuePair<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.key, &self.value)?;
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        serializes_as_key_value_pair(KeyValuePair::new("key", "value"), json!({ "key": "value" }));
    }
}
