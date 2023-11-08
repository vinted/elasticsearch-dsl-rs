//! collapse search results: https://www.elastic.co/guide/en/elasticsearch/reference/current/collapse-search-results.html

/// Internal representation for collapse 
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Collapse {
    field: String
}


impl Collapse {
    /// Creates an instance of [`Collapse`]
    ///
    /// - `field` - Field you wish to collapse on
    pub fn field<T>(field: T) -> Self 
    where
        T: ToString
    {
        Self { field: field.to_string() }
    }
}