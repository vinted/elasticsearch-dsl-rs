/// Control how the total number of hits should be tracked.
///
/// When set to `Track` with a value `true`, the response will always track the number of hits that
/// match the query accurately.
///
/// When set to `Count` with an integer value `n`, the response accurately tracks the total
/// hit count that match the query up to `n` documents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TrackTotalHits {
    /// Whether to accurately track the number of hits that match the query accurately
    Track(bool),

    /// Accurately track the number of hits up to the specified value
    Count(i64),
}

impl From<bool> for TrackTotalHits {
    fn from(value: bool) -> Self {
        TrackTotalHits::Track(value)
    }
}

impl From<i64> for TrackTotalHits {
    fn from(value: i64) -> Self {
        TrackTotalHits::Count(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn serialization() {
        assert_serialize(
            [
                TrackTotalHits::Track(false),
                TrackTotalHits::Track(true),
                TrackTotalHits::Count(10),
            ],
            json!([false, true, 10,]),
        )
    }
}
