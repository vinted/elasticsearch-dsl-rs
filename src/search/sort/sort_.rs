use crate::SortSpecialField;

use super::FieldSort;
use std::borrow::Cow;

/// Sorting criterion
#[derive(Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Sort {
    /// Special sort field,
    SpecialField(SortSpecialField),

    /// Sorts by field name
    Field(String),

    /// Sorts by field name with finer control
    FieldSort(FieldSort),
}

impl std::fmt::Debug for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpecialField(sort) => sort.fmt(f),
            Self::Field(sort) => sort.fmt(f),
            Self::FieldSort(sort) => sort.fmt(f),
        }
    }
}

impl From<SortSpecialField> for Sort {
    fn from(value: SortSpecialField) -> Self {
        Self::SpecialField(value)
    }
}

impl From<&str> for Sort {
    fn from(value: &str) -> Self {
        Self::Field(value.to_string())
    }
}

impl From<Cow<'_, str>> for Sort {
    fn from(value: Cow<'_, str>) -> Self {
        Self::Field(value.to_string())
    }
}

impl From<String> for Sort {
    fn from(value: String) -> Self {
        Self::Field(value)
    }
}

impl From<FieldSort> for Sort {
    fn from(value: FieldSort) -> Self {
        Self::FieldSort(value)
    }
}

impl IntoIterator for Sort {
    type Item = Self;

    type IntoIter = std::option::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Some(self).into_iter()
    }
}
