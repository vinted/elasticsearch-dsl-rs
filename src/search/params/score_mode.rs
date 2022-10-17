/// The way the scores are combined can be controlled
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScoreMode {
    /// Add the original score and the rescore query score.
    Total,

    /// Multiply the original score by the rescore query score.
    Multiply,

    /// Take the min of the original score and the rescore query score.
    Min,

    /// Take the max of original score and the rescore query score.
    Max,

    /// Average the original score and the rescore query score.
    Avg,
}
