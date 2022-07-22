use crate::util::ShouldSkip;

/// The completion suggester considers all documents in the index, but it is often desirable to
/// serve suggestions filtered and/or boosted by some criteria. For example, you want to suggest
/// song titles filtered by certain artists or you want to boost song titles based on their genre.
///
/// To achieve suggestion filtering and/or boosting, you can add context mappings while configuring
/// a completion field. You can define multiple context mappings for a completion field. Every
/// context mapping has a unique name and a type. There are two types: `category` and `geo`.
/// Context mappings are configured under the contexts parameter in the field mapping.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SuggestContextQuery {
    context: String,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    prefix: Option<bool>,
}

impl SuggestContextQuery {
    /// Creates an instance of [SuggestContextQuery]
    ///
    /// - `context` - The value of the category to filter/boost on
    pub fn new<T>(context: T) -> Self
    where
        T: ToString,
    {
        Self {
            context: context.to_string(),
            boost: None,
            prefix: None,
        }
    }

    /// The factor by which the score of the suggestion should be boosted, the score is computed by
    /// multiplying the boost with the suggestion weight, defaults to `1`
    pub fn boost<T>(mut self, boost: T) -> Self
    where
        T: num_traits::AsPrimitive<f32>,
    {
        self.boost = Some(boost.as_());
        self
    }

    /// Whether the category value should be treated as a prefix or not. For example, if set to
    /// `true`, you can filter category of _type1_, _type2_ and so on, by specifying a category
    /// prefix of type. Defaults to `false`
    pub fn prefix(mut self, prefix: bool) -> Self {
        self.prefix = Some(prefix);
        self
    }
}

impl IntoIterator for SuggestContextQuery {
    type Item = Self;

    type IntoIter = std::option::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Some(self).into_iter()
    }
}
