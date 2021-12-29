/// The `minimum_should_match` type alias
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-minimum-should-match.html>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct MinimumShouldMatch(String);

impl<T> From<T> for MinimumShouldMatch
where
    T: ToString,
{
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}
