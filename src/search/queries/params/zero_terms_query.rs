/// Indicates whether no documents are returned if the `analyzer` removes all
/// tokens, such as when using a `stop` filter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ZeroTermsQuery {
    /// No documents are returned if the `analyzer` removes all tokens.
    None,

    /// Returns all documents, similar to a
    /// [`match_all`](crate::queries::MatchAllQuery)
    /// query.
    All,
}
