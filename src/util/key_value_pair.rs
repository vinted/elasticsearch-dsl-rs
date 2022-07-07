use serde::ser::{Serialize, SerializeMap, Serializer};

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct KeyValuePair<K, V> {
    pub(crate) key: K,
    pub(crate) value: V,
}

impl<K, V> KeyValuePair<K, V> {
    /// Creates an instance of [`KeyValuePair`]
    pub(crate) fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

impl<K, V> std::fmt::Debug for KeyValuePair<K, V>
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entry(&self.key, &self.value).finish()
    }
}

impl<K, V> Serialize for KeyValuePair<K, V>
where
    K: Serialize,
    V: Serialize,
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
    use crate::util::*;

    #[test]
    fn serializes_as_key_value_pair() {
        assert_serialize(KeyValuePair::new("key", "value"), json!({ "key": "value" }));
    }
}
