//! Allows constructing Elasticsearch search query.
//!
//! Elasticsearch provides a full Query DSL (Domain Specific Language) based on JSON to define queries. Think of the Query DSL as an AST (Abstract Syntax Tree) of queries, consisting of two types of clauses:
//!
//! **Leaf query clauses**
//!
//! Leaf query clauses look for a particular value in a particular field, such as the match, term or range queries. These queries can be used by themselves.
//!
//! **Compound query clauses**
//!
//! Compound query clauses wrap other leaf or compound queries and are used to combine multiple queries in a logical fashion (such as the bool or dis_max query), or to alter their behavior (such as the constant_score query).
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl.html>

// Public modules
pub mod params;

pub mod compound;
pub mod custom;
pub mod full_text;
pub mod geo;
pub mod joining;
pub mod shape;
pub mod span;
pub mod specialized;
pub mod term_level;

pub use self::compound::*;
pub use self::custom::*;
pub use self::full_text::*;
pub use self::geo::*;
pub use self::joining::*;
pub use self::shape::*;
pub use self::span::*;
pub use self::specialized::*;
pub use self::term_level::*;

// Very special queries
mod match_all_query;
mod match_none_query;

pub use self::match_all_query::*;
pub use self::match_none_query::*;

use crate::util::*;

macro_rules! query {
    ($($variant:ident($query:ty)),+ $(,)?) => {
        /// A container enum for supported Elasticsearch query types
        #[derive(Debug, Clone, PartialEq, Serialize)]
        #[serde(untagged)]
        #[allow(missing_docs)]
        pub enum Query {
            $(
                $variant($query),
            )*
        }

        impl IntoIterator for Query {
            type Item = Self;
            type IntoIter = std::option::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                if self.should_skip() {
                    None.into_iter()
                } else {
                    Some(self).into_iter()
                }
            }
        }

        impl ShouldSkip for Query {
            fn should_skip(&self) -> bool {
                match self {
                    $(
                        Query::$variant(q) => q.should_skip(),
                    )+
                }
            }
        }

        $(
            impl From<$query> for Query {
                fn from(q: $query) -> Self {
                    Query::$variant(q)
                }
            }

            impl From<$query> for Option<Query> {
                fn from(q: $query) -> Self {
                    if q.should_skip() {
                        None
                    } else {
                        Some(Query::$variant(q))
                    }
                }
            }

            impl From<$query> for Queries {
                fn from(q: $query) -> Self {
                    if q.should_skip() {
                        Default::default()
                    } else {
                        Self(vec![q.into(); 1])
                    }
                }
            }
        )+
    };
}

query!(
    Bool(BoolQuery),
    Prefix(PrefixQuery),
    Regexp(RegexpQuery),
    Wildcard(WildcardQuery),
    TermsSet(TermsSetQuery),
    Term(TermQuery),
    Terms(TermsQuery),
    TermsLookup(TermsLookupQuery),
    Exists(ExistsQuery),
    Range(RangeQuery),
    Ids(IdsQuery),
    ConstantScore(ConstantScoreQuery),
    DistanceFeatureDate(DistanceFeatureQuery<chrono::DateTime<chrono::Utc>>),
    DistanceFeatureGeo(DistanceFeatureQuery<crate::GeoPoint>),
    Match(MatchQuery),
    MatchBoolPrefix(MatchBoolPrefixQuery),
    MatchPhrasePrefix(MatchPhrasePrefixQuery),
    MatchAll(MatchAllQuery),
    MatchNone(MatchNoneQuery),
    MatchPhrase(MatchPhraseQuery),
    MultiMatch(MultiMatchQuery),
    Nested(NestedQuery),
    Boosting(BoostingQuery),
    DisMax(DisMaxQuery),
    Pinned(PinnedQuery),
    Percolate(PercolateQuery),
    PercolateLookup(PercolateLookupQuery),
    FunctionScore(FunctionScoreQuery),
    RankFeature(RankFeatureQuery),
    RankFeatureSaturation(RankFeatureSaturationQuery),
    RankFeatureLogarithm(RankFeatureLogarithmQuery),
    RankFeatureSigmoid(RankFeatureSigmoidQuery),
    RankFeatureLinear(RankFeatureLinearQuery),
    MoreLikeThis(MoreLikeThisQuery),
    Fuzzy(FuzzyQuery),
    GeoDistance(GeoDistanceQuery),
    GeoBoundingBox(GeoBoundingBoxQuery),
    GeoShapeLookup(GeoShapeLookupQuery),
    GeoShape(GeoShapeQuery),
    ShapeLookup(ShapeLookupQuery),
    Shape(ShapeQuery),
    Json(JsonQuery),
    Wrapper(WrapperQuery),
    Script(ScriptQuery),
    ScriptScore(ScriptScoreQuery),
);

/// A collection of queries
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Queries(Vec<Query>);

impl<T> From<T> for Queries
where
    T: IntoIterator,
    T::Item: Into<Option<Query>>,
{
    fn from(value: T) -> Self {
        Self(
            value
                .into_iter()
                .filter_map(Into::into)
                .filter(|x| !x.should_skip())
                .collect(),
        )
    }
}

impl ShouldSkip for Queries {
    fn should_skip(&self) -> bool {
        self.0.should_skip()
    }
}

impl Queries {
    /// Pushes multiple queries to the collection
    pub fn extend<Q>(&mut self, queries: Q)
    where
        Q: Into<Queries>,
    {
        self.0.extend(queries.into().0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_query() {
        let mut queries = Queries::default();

        let query = Query::terms("test", [1]);

        queries.extend(query);

        assert_eq!(queries.0.len(), 1);
    }

    #[test]
    fn adds_queries() {
        let mut queries = Queries::default();

        let query_1 = Query::terms("test", [1]);
        let query_2 = Query::terms("test", [2]);

        queries.extend([query_1, query_2]);

        assert_eq!(queries.0.len(), 2);
    }

    #[test]
    fn skips_queries() {
        let mut queries = Queries::default();

        let empty_values: [i32; 0] = [];

        let query_1 = Query::terms("test", empty_values).into();
        let query_2 = Query::from(Query::terms("test", empty_values));
        let query_3 = Query::Terms(Query::terms("test", empty_values));

        queries.extend([query_1, query_2, query_3]);

        assert!(queries.0.is_empty());
    }

    #[test]
    fn skips_query() {
        let mut queries = Queries::default();

        let empty_values: [i32; 0] = [];

        let query = Query::terms("test", empty_values);

        queries.extend(query);

        assert!(queries.0.is_empty());
    }
}
