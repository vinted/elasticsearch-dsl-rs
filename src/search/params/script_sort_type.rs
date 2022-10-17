/// How to treat sorting script value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptSortType {
    /// Sort script result as a string
    String,

    /// Sort script result as a number
    Number,
}
