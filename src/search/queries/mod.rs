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
    ($name:ident { $($variant:ident($query:ty)),+ $(,)? }) => {
        /// A container enum for supported Elasticsearch query types
        #[derive(Debug, Clone, PartialEq, Serialize)]
        #[serde(untagged)]
        #[allow(missing_docs)]
        pub enum $name {
            $(
                $variant($query),
            )*
        }

        $(
            impl From<$query> for $name {
                fn from(q: $query) -> Self {
                    $name::$variant(q)
                }
            }
        )+

        $(
            impl From<$query> for Option<$name> {
                fn from(q: $query) -> Self {
                    if q.should_skip() {
                        None
                    } else {
                        Some($name::$variant(q))
                    }
                }
            }
        )+

        impl ShouldSkip for $name {
            fn should_skip(&self) -> bool {
                match self {
                    $(
                        $name::$variant(q) => q.should_skip(),
                    )+
                }
            }
        }
    };
}

query!(Query {
    Bool(BoolQuery),
    Prefix(PrefixQuery),
    Regexp(RegexpQuery),
    Wildcard(WildcardQuery),
    TermsSet(TermsSetQuery),
    Term(TermQuery),
    TermsValues(TermsQuery<std::collections::BTreeSet<crate::Term>>),
    TermsLookup(TermsQuery<self::params::TermsLookup>),
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
    PercolateDocument(PercolateQuery<serde_json::Value>),
    PercolateDocuments(PercolateQuery<Vec<serde_json::Value>>),
    PercolateLookup(PercolateQuery<self::params::PercolateLookup>),
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
    Json(JsonQuery),
});
