/// Score explanation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Explanation {
    /// Cumulative score description
    pub description: String,

    /// Cumulative score
    pub value: f64,

    /// Score details
    #[serde(default)]
    pub details: Vec<Explanation>,
}
