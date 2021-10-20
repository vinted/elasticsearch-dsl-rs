/// Matched fields logic with type conversions
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct MatchedFields(Vec<String>);

impl<T, const N: usize> From<[T; N]> for MatchedFields
where
    T: ToString,
{
    fn from(values: [T; N]) -> Self {
        Self(values.iter().map(ToString::to_string).collect())
    }
}

impl<T> From<Vec<T>> for MatchedFields
where
    T: ToString,
{
    fn from(values: Vec<T>) -> Self {
        Self(values.iter().map(ToString::to_string).collect())
    }
}

impl<'a, T> From<&'a [T]> for MatchedFields
where
    T: ToString,
{
    fn from(values: &'a [T]) -> Self {
        Self(values.iter().map(ToString::to_string).collect())
    }
}
