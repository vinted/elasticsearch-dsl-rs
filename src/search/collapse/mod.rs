//! collapse search results: https://www.elastic.co/guide/en/elasticsearch/reference/current/collapse-search-results.html

use crate::{SortCollection, util::ShouldSkip, Sort};

/// Second level of collapsing; this does not support inner_hits
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct InnerCollapse {
    field: String
}

impl InnerCollapse {
    /// Creates an instance of [`InnerCollapse`]
    ///
    /// - `field` - Field you wish to collapse on
    pub fn field<T>(field: T) -> Self 
    where
        T: ToString
    {
        Self { 
            field: field.to_string(),
        }
    }
}


/// Internal representation for collapse 
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Collapse {
    field: String,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    inner_hits: InnerHitsCollection,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    max_concurrent_group_searches: Option<u64>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    collapse: Option<InnerCollapse>,

}

/// Inner hits object
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct InnerHits {
    name: String,
    size: u64,
    sort: SortCollection
}

impl InnerHits {
    /// Create a new InnerHits object
    pub fn new<T, U>(
        name: T,
        size: u64,
        sort: U
    ) -> Self 
    where
        T: ToString,
        U: IntoIterator,
        U::Item: Into<Sort>
    {

        let mut sort_collection = SortCollection::default();
        sort_collection.extend(sort);
        Self {
            name: name.to_string(),
            size: size,
            sort: sort_collection
        }
    }
}


/// a collection of inner hits objects
#[derive(Serialize, Clone, PartialEq)]
pub struct InnerHitsCollection(Vec<InnerHits>);


impl ShouldSkip for InnerHitsCollection {
    fn should_skip(&self) -> bool {
        self.0.should_skip()
    }
}

impl std::fmt::Debug for InnerHitsCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoIterator for InnerHitsCollection {
    type Item = InnerHits;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl InnerHitsCollection {
    /// Extends inner hits collection
    pub fn extend<T>(&mut self, inner_hits: T)
    where
        T: IntoIterator,
        T::Item: Into<InnerHits>,
    {
        self.0.extend(
            inner_hits
                .into_iter()
                .map(Into::into)
        )
    }
}


impl Collapse {
    /// Creates an instance of [`Collapse`]
    ///
    /// - `field` - Field you wish to collapse on
    pub fn field<T>(field: T) -> Self 
    where
        T: ToString
    {
        Self { 
            field: field.to_string(),
            max_concurrent_group_searches: None,
            collapse: None,
            inner_hits: InnerHitsCollection(Vec::new())
        }
    }


    /// Add `inner_hits` to Collapse
    pub fn inner_hits<T>(mut self, inner_hits: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<InnerHits>
    {
        self.inner_hits.extend(inner_hits.into_iter());
        self
    }

    /// Add `max_concurrent_group_searches` to Collapse
    pub fn max_concurrent_group_searches(mut self, max_concurrent_group_searches: u64) -> Self
    {
        self.max_concurrent_group_searches = Some(max_concurrent_group_searches);
        self
    }

    /// Add second level of collapsing
    pub fn collapse<T>(mut self, collapse: T) -> Self 
    where 
        T: Into<InnerCollapse>
    {
        self.collapse = Some(collapse.into());
        self
    }
}



#[cfg(test)]
mod tests {
    use crate::{util::assert_serialize, FieldSort};

    use super::*;

    #[test]
    fn serialization() {

        assert_serialize(
            Collapse::field("field"),
            json!({ "field": "field" }),
        );

        assert_serialize(
            Collapse::field("field")
                .inner_hits([
                    InnerHits::new("inner_hits", 10,  [
                        FieldSort::descending("sort_field"),
                    ])
                ])
                .max_concurrent_group_searches(10)
                .collapse(InnerCollapse::field("field_inner_collapse")),
            json!({
                "field": "field",
                "inner_hits": [
                    {
                        "name": "inner_hits",
                        "size": 10,
                        "sort": [{"sort_field": {"order": "desc"}}] 
                    }
                ],
                "collapse": {"field": "field_inner_collapse"},
                "max_concurrent_group_searches": 10
            })  
        );

        assert_serialize(
            Collapse::field("field")
                .inner_hits([
                    InnerHits::new("inner_hits_0", 10,  [
                        FieldSort::descending("sort_field"),
                    ])
                ])
                .inner_hits([
                    InnerHits::new("inner_hits_1", 10,  [
                        FieldSort::descending("sort_field"),
                    ])
                ])
                .max_concurrent_group_searches(10)
                .collapse(InnerCollapse::field("field_inner_collapse")),
            json!({
                "field": "field",
                "inner_hits": [
                    {
                        "name": "inner_hits_0",
                        "size": 10,
                        "sort": [{"sort_field": {"order": "desc"}}] 
                    },
                    {
                        "name": "inner_hits_1",
                        "size": 10,
                        "sort": [{"sort_field": {"order": "desc"}}] 
                    }
                ],
                "collapse": {"field": "field_inner_collapse"},
                "max_concurrent_group_searches": 10
            })  
        );

        assert_serialize(
            Collapse::field("field")
                .collapse(InnerCollapse::field("field_inner_collapse")),
            json!({
                "field": "field",
                "collapse": {"field": "field_inner_collapse"}
            })  
        );
    }  
}