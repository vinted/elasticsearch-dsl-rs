use super::{FieldSort, Sort};
use crate::util::ShouldSkip;

/// A sorting criteria
#[derive(Default, Clone, PartialEq, Serialize)]
pub struct SortCollection(Vec<Sort>);

impl std::fmt::Debug for SortCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoIterator for SortCollection {
    type Item = Sort;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl ShouldSkip for SortCollection {
    fn should_skip(&self) -> bool {
        self.0.should_skip()
    }
}

impl SortCollection {
    /// Creates a new instance of [SortCollection]
    pub fn new() -> Self {
        Default::default()
    }

    /// Extends sorting collection
    pub fn extend<T>(&mut self, sort: T)
    where
        T: IntoIterator,
        T::Item: Into<Sort>,
    {
        self.0.extend(sort.into_iter().map(Into::into))
    }

    /// Add a field to sort by ascending order
    pub fn ascending<T>(mut self, field: T) -> Self
    where
        T: ToString,
    {
        self.0.push(Sort::FieldSort(FieldSort::ascending(field)));
        self
    }

    /// Add a field to sort by descending order
    pub fn descending<T>(mut self, field: T) -> Self
    where
        T: ToString,
    {
        self.0.push(Sort::FieldSort(FieldSort::descending(field)));
        self
    }

    /// Add a field sort
    pub fn field(mut self, field_sort: FieldSort) -> Self {
        self.0.push(Sort::FieldSort(field_sort));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::assert_serialize_sort;
    use crate::SortSpecialField;

    #[test]
    fn serializes_correctly() {
        assert_serialize_sort(["abc", "def"], json!(["abc", "def"]));

        assert_serialize_sort(
            [
                FieldSort::ascending("field1"),
                FieldSort::descending("field2"),
            ],
            json!([
                { "field1": { "order": "asc" } },
                { "field2": { "order": "desc" } },
            ]),
        );

        assert_serialize_sort(
            [
                Sort::FieldSort(
                    FieldSort::ascending("post_date").format("strict_date_optional_time_nanos"),
                ),
                Sort::Field("user".to_string()),
                Sort::FieldSort(FieldSort::descending("name")),
                Sort::FieldSort(FieldSort::descending("age")),
                Sort::SpecialField(SortSpecialField::Score),
            ],
            json!([
                { "post_date" : {"order" : "asc", "format": "strict_date_optional_time_nanos" } },
                "user",
                { "name" : { "order": "desc" } },
                { "age" : { "order": "desc" } },
                "_score"
            ]),
        );

        assert_serialize_sort(
            SortCollection::new()
                .ascending("name")
                .descending("age")
                .field(FieldSort::ascending("post_date").format("strict_date_optional_time_nanos")),
            json!([
                { "name" : { "order": "asc" } },
                { "age" : { "order": "desc" } },
                { "post_date" : {"order" : "asc", "format": "strict_date_optional_time_nanos" } },
            ]),
        );
    }
}
