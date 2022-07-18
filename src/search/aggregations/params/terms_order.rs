use crate::{
    util::{KeyValuePair, ShouldSkip},
    SortOrder,
};

/// Terms Aggregation sorting criterion
#[derive(Clone, PartialEq, Serialize)]
pub struct TermsOrder(KeyValuePair<String, SortOrder>);

impl std::fmt::Debug for TermsOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TermsOrder")
            .field(&self.0.key, &self.0.value)
            .finish()
    }
}

impl TermsOrder {
    /// Creates an instance of [TermsOrder]
    ///
    /// - `key` - Key to sort by
    /// - `order` - Sorting order
    pub fn new<T>(key: T, order: SortOrder) -> Self
    where
        T: ToString,
    {
        Self(KeyValuePair::new(key.to_string(), order))
    }

    /// Sorts terms by a given key in ascending order
    pub fn ascending<T>(key: T) -> Self
    where
        T: ToString,
    {
        Self::new(key, SortOrder::Asc)
    }

    /// Sorts terms by a given key in descending order
    pub fn descending<T>(key: T) -> Self
    where
        T: ToString,
    {
        Self::new(key, SortOrder::Desc)
    }

    /// Sorts terms by count ascending
    pub fn count_ascending() -> Self {
        Self::ascending("_count")
    }

    /// Sorts terms by count descending
    pub fn count_descending() -> Self {
        Self::descending("_count")
    }

    /// Sorts terms by count ascending
    pub fn key_ascending() -> Self {
        Self::ascending("_key")
    }

    /// Sorts terms by count descending
    pub fn key_descending() -> Self {
        Self::descending("_key")
    }
}

/// Terms Aggregation sorting criteria
#[derive(Default, Clone, PartialEq, Serialize)]
pub struct TermsOrderCollection(Vec<TermsOrder>);

impl ShouldSkip for TermsOrderCollection {
    fn should_skip(&self) -> bool {
        self.0.should_skip()
    }
}

impl std::fmt::Debug for TermsOrderCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<TermsOrder> for TermsOrderCollection {
    fn from(value: TermsOrder) -> Self {
        Self(vec![value])
    }
}

impl<T> From<T> for TermsOrderCollection
where
    T: IntoIterator,
    T::Item: Into<TermsOrder>,
{
    fn from(value: T) -> Self {
        Self(value.into_iter().map(Into::into).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::assert_serialize;

    #[test]
    fn serializes() {
        assert_serialize(TermsOrder::key_ascending(), json!({ "_key": "asc" }));
        assert_serialize(TermsOrder::count_descending(), json!({ "_count": "desc" }));
    }
}
