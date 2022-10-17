/// Boolean logic used to interpret text in the `query` value
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Operator {
    /// For example, a `query` value of `capital of Hungary` is interpreted as
    /// `capital OR of OR Hungary`.
    Or,

    /// For example, a `query` value of `capital of Hungary` is interpreted as
    /// `capital AND of AND Hungary`.
    ///
    And,
}
