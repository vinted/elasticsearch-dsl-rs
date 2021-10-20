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

// Private modules
mod bool_query;
mod boosting_query;
mod constant_score_query;
mod dis_max_query;
mod distance_feature_query;
mod exists_query;
mod function_score_query;
mod fuzzy_query;
mod ids_query;
mod match_all_query;
mod match_bool_prefix_query;
mod match_none_query;
mod match_phrase_prefix_query;
mod match_phrase_query;
mod match_query;
mod more_like_this_query;
mod multi_match_query;
mod nested_query;
mod percolate_query;
mod prefix_query;
mod range_query;
mod rank_feature_query;
mod regexp_query;
mod term_query;
mod terms_query;
mod terms_set_query;
mod wildcard_query;

// Public re-exports
pub use self::bool_query::*;
pub use self::boosting_query::*;
pub use self::constant_score_query::*;
pub use self::dis_max_query::*;
pub use self::distance_feature_query::*;
pub use self::exists_query::*;
pub use self::function_score_query::*;
pub use self::fuzzy_query::*;
pub use self::ids_query::*;
pub use self::match_all_query::*;
pub use self::match_bool_prefix_query::*;
pub use self::match_none_query::*;
pub use self::match_phrase_prefix_query::*;
pub use self::match_phrase_query::*;
pub use self::match_query::*;
pub use self::more_like_this_query::*;
pub use self::multi_match_query::*;
pub use self::nested_query::*;
pub use self::percolate_query::*;
pub use self::prefix_query::*;
pub use self::range_query::*;
pub use self::rank_feature_query::*;
pub use self::regexp_query::*;
pub use self::term_query::*;
pub use self::terms_query::*;
pub use self::terms_set_query::*;
pub use self::wildcard_query::*;

crate::query!(Query {
    Bool(BoolQuery),
    Prefix(PrefixQuery),
    Regexp(RegexpQuery),
    Wildcard(WildcardQuery),
    TermsSet(TermsSetQuery),
    Term(TermQuery),
    TermsValues(TermsQuery<std::collections::BTreeSet<crate::Scalar>>),
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
});
