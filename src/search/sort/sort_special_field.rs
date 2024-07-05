/// Special sorting field variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SortSpecialField {
    /// Document score
    #[serde(rename = "_score")]
    Score,

    /// The most efficient way to sort, does not guarantee any order, useful for scrolling
    #[serde(rename = "_doc")]
    DocumentIndexOrder,

    /// Sorts by shard doc value, useful for PIT queries
    #[serde(rename = "_shard_doc")]
    ShardDocumentOrder,
}

impl std::fmt::Display for SortSpecialField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Score => "_score".fmt(f),
            Self::DocumentIndexOrder => "_doc".fmt(f),
            Self::ShardDocumentOrder => "_shard_doc".fmt(f),
        }
    }
}
