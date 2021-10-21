/// Used to compare serialized structures
#[doc(hidden)]
#[macro_export]
#[cfg(test)]
macro_rules! test_serialization {
    (
        $(
            $name:ident($subject:expr, $expectation:expr)
        );+
        $(;)?
    ) => {
        use serde_json::{json, Value, to_string, from_str};

        $(
            #[test]
            fn $name() {
                let string = to_string(&$subject).unwrap();
                let result: Value = from_str(&string).unwrap();

                assert_eq!(result, $expectation);
            }
        )+
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! add_boost_and_name {
    () => {
        /// Floating point number used to decrease or increase the
        /// [relevance scores](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-filter-context.html#relevance-scores)
        /// of a query. Defaults to `1.0`.
        ///
        /// You can use the boost parameter to adjust relevance scores for
        /// searches containing two or more queries.
        ///
        /// Boost values are relative to the default value of `1.0`.
        /// A boost value between 0 and `1.0` decreases the relevance score.
        /// A value greater than `1.0` increases the relevance score.
        pub fn boost(mut self, boost: impl Into<Boost>) -> Self {
            self.inner.boost = Some(boost.into());
            self
        }

        /// You can use named queries to track which queries matched
        /// returned documents. If named queries are used, the response
        /// includes a `matched_queries` property for each hit.
        ///
        /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-bool-query.html#named-queries>
        pub fn name(mut self, name: impl Into<String>) -> Self {
            self.inner._name = Some(name.into());
            self
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! add_aggregate {
    () => {
        /// Pushes aggregation
        pub fn aggregate(mut self, aggregation: impl Into<Aggregation>) -> Self {
            let a = aggregation.into();
            let _ = self.aggs.entry(a.name()).or_insert(a);
            self
        }
    };
}
