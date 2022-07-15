/// Special sorting field variants
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
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

impl ToString for SortSpecialField {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Score => "_score",
            Self::DocumentIndexOrder => "_doc",
            Self::ShardDocumentOrder => "_shard_doc",
        })
    }
}
