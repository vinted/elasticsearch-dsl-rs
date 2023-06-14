use serde::Serialize;

use crate::{types::Set, util::ShouldSkip};

/// It’s also possible to store an individual field’s values by using the store mapping option.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/8.8/search-fields.html#stored-fields>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoredFields {
    /// To disable the stored fields (and metadata fields) entirely use: `_none_`
    None,

    /// Allows to selectively load specific stored fields for each document represented by a search hit.
    Fields(Set<String>),
}

impl Default for StoredFields {
    fn default() -> Self {
        Self::Fields(Set::default())
    }
}

impl ShouldSkip for StoredFields {
    fn should_skip(&self) -> bool {
        match self {
            Self::None => false,
            Self::Fields(fields) => fields.should_skip(),
        }
    }
}

impl Serialize for StoredFields {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::None => serializer.serialize_str("_none_"),
            Self::Fields(fields) => fields.serialize(serializer),
        }
    }
}

impl<T> From<T> for StoredFields
where
    T: IntoIterator,
    T::Item: ToString,
{
    fn from(value: T) -> Self {
        let fields = value.into_iter().map(|v| v.to_string()).collect::<Set<_>>();

        if fields.len() == 1 && fields.contains("_none_") {
            Self::None
        } else {
            Self::Fields(fields)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::assert_serialize;

    #[test]
    fn serialization() {
        assert_serialize(StoredFields::None, json!("_none_"));
        assert_serialize(StoredFields::from(["_none_"]), json!("_none_"));
        assert_serialize(StoredFields::from(["abc", "def"]), json!(["abc", "def"]));
    }

    #[test]
    fn should_skip() {
        assert!(!StoredFields::None.should_skip());
        assert!(!StoredFields::from(["abc", "def"]).should_skip());
        assert!(StoredFields::from(Vec::<String>::new()).should_skip());
    }
}
