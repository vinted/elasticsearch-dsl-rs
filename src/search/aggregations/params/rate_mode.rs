/// Calculate sum or number of values of the field for [elasticsearch_dsl::search::RateAggregation]
#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RateMode {
    /// calculate the sum of all values field
    Sum,
    /// use the number of values in the field
    ValueCount,
}
