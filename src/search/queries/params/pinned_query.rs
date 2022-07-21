use crate::Set;

/// Ids or documents to filter by
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PinnedQueryValues {
    /// [Document IDs](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-id-field.html)
    /// listed in the order they are to appear in results.
    Ids(Set<String>),

    /// Documents listed in the order they are to appear in results.
    Docs(Set<PinnedDocument>),
}

/// Pinned document
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PinnedDocument {
    _index: String,
    _id: String,
}

impl PinnedDocument {
    /// Creates an instance of [`PinnedDocument`]
    pub fn new<IX, ID>(index: IX, id: ID) -> Self
    where
        IX: ToString,
        ID: ToString,
    {
        Self {
            _index: index.to_string(),
            _id: id.to_string(),
        }
    }
}

impl PinnedQueryValues {
    /// Creates an instance of [`PinnedQueryValues`] with [`PinnedQueryValues::Ids`]
    pub fn ids<I>(ids: I) -> Self
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        Self::Ids(ids.into_iter().map(|x| x.to_string()).collect())
    }

    /// Creates an instance of [`PinnedQueryValues`] with [`PinnedQueryValues::Docs`]
    pub fn docs<I>(docs: I) -> Self
    where
        I: IntoIterator<Item = PinnedDocument>,
    {
        Self::Docs(docs.into_iter().collect())
    }
}
