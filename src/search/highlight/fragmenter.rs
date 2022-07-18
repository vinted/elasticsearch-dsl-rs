/// Specifies how text should be broken up in highlight snippets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Fragmenter {
    /// Breaks up text into same-sized fragments.
    Simple,

    /// Breaks up text into same-sized fragments, but tries to avoid breaking up text between
    /// highlighted terms. This is helpful when you’re querying for phrases. Default.
    Span,
}
