use super::Query;
use crate::util::ShouldSkip;

/// A collection of queries
#[derive(Default, Clone, PartialEq, Serialize)]
pub struct QueryCollection(Vec<Query>);

impl std::fmt::Debug for QueryCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoIterator for QueryCollection {
    type Item = Query;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl ShouldSkip for QueryCollection {
    fn should_skip(&self) -> bool {
        self.0.should_skip()
    }
}

impl QueryCollection {
    /// Extends query collection
    pub fn extend<T>(&mut self, query: T)
    where
        T: IntoIterator,
        T::Item: Into<Query>,
    {
        self.0.extend(
            query
                .into_iter()
                .map(Into::into)
                .filter(ShouldSkip::should_keep),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_query() {
        let mut queries = QueryCollection::default();

        let query = Query::terms("test", [1]);

        queries.extend(query);

        assert_eq!(queries.0.len(), 1);
    }

    #[test]
    fn adds_queries() {
        let mut queries = QueryCollection::default();

        let query_1 = Query::terms("test", [1]);
        let query_2 = Query::terms("test", [2]);

        queries.extend([query_1, query_2]);

        assert_eq!(queries.0.len(), 2);
    }

    #[test]
    fn skips_queries() {
        let mut queries = QueryCollection::default();

        let empty_values: [i32; 0] = [];

        let query_1 = Query::terms("test", empty_values).into();
        let query_2 = Query::from(Query::terms("test", empty_values));
        let query_3 = Query::Terms(Query::terms("test", empty_values));

        queries.extend([query_1, query_2, query_3]);

        assert!(queries.0.is_empty());
    }

    #[test]
    fn skips_query() {
        let mut queries = QueryCollection::default();

        let empty_values: [i32; 0] = [];

        let query = Query::terms("test", empty_values);

        queries.extend(query);

        assert!(queries.0.is_empty());
    }
}
