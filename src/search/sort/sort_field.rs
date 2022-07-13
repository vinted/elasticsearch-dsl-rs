use serde::ser::{Serialize, Serializer};

/// Allows you to add one or more sorts on specific fields. Each sort can be reversed as well. The sort is defined on a per field level.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#sort-search-results>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortField {
    /// Sort by `_id` field
    Id,

    /// Sort by `_score`
    Score,

    /// Sort by key within aggregations
    Key,

    /// Sort by count within aggregations,
    Count,

    /// Sort by index order
    Doc,

    /// Sorts by a given field name
    Field(String),
}

impl Serialize for SortField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Id => "_id".serialize(serializer),
            Self::Score => "_score".serialize(serializer),
            Self::Key => "_key".serialize(serializer),
            Self::Count => "_count".serialize(serializer),
            Self::Doc => "_doc".serialize(serializer),
            Self::Field(field) => field.serialize(serializer),
        }
    }
}

impl<T> From<T> for SortField
where
    T: ToString,
{
    fn from(value: T) -> Self {
        Self::Field(value.to_string())
    }
}
