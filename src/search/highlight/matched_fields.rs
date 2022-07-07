/// Matched fields logic with type conversions
#[derive(Clone, Default, PartialEq, Eq, Serialize)]
pub struct MatchedFields(Vec<String>);

impl std::fmt::Debug for MatchedFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<T> for MatchedFields
where
    T: IntoIterator,
    T::Item: ToString,
{
    fn from(value: T) -> Self {
        Self(value.into_iter().map(|x| x.to_string()).collect())
    }
}
