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
                self.boost = Some(boost);
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
            self._name = Some(name.to_string());
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

#[doc(hidden)]
#[macro_export]
macro_rules! serialize_with_root {
    ($root:tt : $inner:ty) => {
        impl $crate::serde::Serialize for $inner {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: $crate::serde::ser::Serializer,
            {
                use $crate::serde::ser::SerializeStruct;

                struct Wrapper<'a> {
                    root: &'a $inner,
                }

                impl<'a> $crate::serde::Serialize for Wrapper<'a> {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: $crate::serde::Serializer,
                    {
                        <$inner>::serialize(&self.root, serializer)
                    }
                }

                let mut state = serializer.serialize_struct("Wrapper", 1)?;
                state.serialize_field($root, &Wrapper { root: self })?;
                state.end()
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! serialize_with_root_keyed {
    ($root:tt : $inner:ty) => {
        impl $crate::serde::Serialize for $inner {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: $crate::serde::ser::Serializer,
            {
                use $crate::serde::ser::SerializeStruct;

                struct Wrapper<'a> {
                    root: &'a $inner,
                }

                impl<'a> $crate::serde::Serialize for Wrapper<'a> {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: $crate::serde::Serializer,
                    {
                        <$inner>::serialize(&self.root, serializer)
                    }
                }

                let mut state = serializer.serialize_struct("Wrapper", 1)?;
                state.serialize_field(
                    $root,
                    &KeyValuePair::new(&self.field, &Wrapper { root: self }),
                )?;
                state.end()
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! serialize_keyed {
    ($inner:ty : $field:ident) => {
        impl $crate::serde::Serialize for $inner {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: $crate::serde::ser::Serializer,
            {
                use $crate::serde::ser::SerializeMap;

                struct Wrapper<'a> {
                    root: &'a $inner,
                }

                impl<'a> $crate::serde::Serialize for Wrapper<'a> {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: $crate::serde::Serializer,
                    {
                        <$inner>::serialize(&self.root, serializer)
                    }
                }

                let mut state = serializer.serialize_map(Some(1))?;
                state.serialize_entry(&self.$field, &Wrapper { root: self })?;
                state.end()
            }
        }
    };
}
