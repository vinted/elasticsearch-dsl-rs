/// Aggregation name
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AggregationName(String);

impl std::fmt::Debug for AggregationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<T> for AggregationName
where
    T: ToString,
{
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}
