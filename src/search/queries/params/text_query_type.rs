/// The way the `multi_match` query is executed internally.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-multi-match-query.html#multi-match-types>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TextQueryType {
    /// Finds documents which match any field, but uses the `_score` from the
    /// best field. See
    /// [`best_fields`](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-multi-match-query.html#type-best-fields).
    BestFields,

    /// Finds documents which match any field and combines the `_score` from
    /// each field. See
    /// [`most_fields`](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-multi-match-query.html#type-most-fields).
    MostFields,

    /// Treats fields with the same `analyzer` as though they were one big
    /// field. Looks for each word in **any** field. See
    /// [`cross_fields`](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-multi-match-query.html#type-cross-fields).
    CrossFields,

    /// Runs a `match_phrase` query on each field and uses the `_score` from
    /// the best field. See
    /// [`phrase` and `phrase_prefix`](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-multi-match-query.html#type-phrase).
    Phrase,

    /// Runs a `match_phrase_prefix` query on each field and uses the `_score`
    /// from the best field. See
    /// [`phrase` and `phrase_prefix`](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-multi-match-query.html#type-phrase).
    PhrasePrefix,

    /// Creates a `match_bool_prefix` query on each field and combines the
    /// `_score` from each field. See
    /// [`bool_prefix`](https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-multi-match-query.html#type-bool-prefix).
    BoolPrefix,
}
