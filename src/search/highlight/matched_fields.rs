/// Matched fields logic with type conversions
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct MatchedFields(Vec<String>);

impl<T> From<T> for MatchedFields
where
    T: IntoIterator,
    T::Item: ToString,
{
    fn from(value: T) -> Self {
        Self(value.into_iter().map(|x| x.to_string()).collect())
    }
}
