/// Indicates if the snippet should be HTML encoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum Encoder {
    /// No encoding
    Default,

    /// HTML-escape the snippet text and then insert the highlighting tags
    Html,
}
