/// Sorts highlighted fragments by score when set to [`score`](Order::Score). By default,
/// fragments will be output in the order they appear in the field
/// (order: [`none`](Order::None)). Setting this option to [`score`](Order::Score) will output
/// the most relevant fragments first. Each highlighter applies its own logic to compute
/// relevancy scores. See the document
/// [How highlighters work internally](https://www.elastic.co/guide/en/elasticsearch/reference/current/highlighting.html#how-es-highlighters-work-internally)
/// for more details how different highlighters find the best fragments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum Order {
    /// Sorts highlighted fragments by score.
    Score,

    /// Highlighted fragments will be output in the order they appear in the field.
    None,
}
