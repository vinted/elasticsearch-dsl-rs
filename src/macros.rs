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
        pub fn boost<B>(mut self, boost: B) -> Self
        where
            B: std::convert::TryInto<Boost>,
        {
            if let Ok(boost) = boost.try_into() {
                self.inner.boost = Some(boost);
            }
            self
        }

        /// You can use named queries to track which queries matched
        /// returned documents. If named queries are used, the response
        /// includes a `matched_queries` property for each hit.
        ///
        /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-bool-query.html#named-queries>
        pub fn name<S>(mut self, name: S) -> Self
        where
            S: ToString,
        {
            self.inner._name = Some(name.to_string());
            self
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! add_aggregate {
    () => {
        /// Pushes aggregation
        pub fn aggregate<N, A>(mut self, aggregation_name: N, aggregation: A) -> Self
        where
            N: Into<AggregationName>,
            A: Into<Aggregation>,
        {
            let a = aggregation.into();
            let _ = self.aggs.entry(aggregation_name.into()).or_insert(a);
            self
        }
    };
}
