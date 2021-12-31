/// Aggregation name
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AggregationName(String);

impl<T> From<T> for AggregationName
where
    T: ToString,
{
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}
