use crate::util::ShouldSkip;
use crate::Fuzziness;

/// Suggester fuzziness parameters
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize)]
pub struct SuggestFuzziness {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fuzziness: Option<Fuzziness>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    min_length: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    prefix_length: Option<u64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    transpositions: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    unicode_aware: Option<bool>,
}

impl SuggestFuzziness {
    /// Creates a new instance of [SuggestFuzziness]
    pub fn new() -> Self {
        Default::default()
    }

    /// The fuzziness factor, defaults to [`AUTO`](Fuzziness::Auto). See [Fuzziness] for allowed
    /// settings.
    pub fn fuzziness<T>(mut self, fuzziness: T) -> Self
    where
        T: Into<Fuzziness>,
    {
        self.fuzziness = Some(fuzziness.into());
        self
    }

    /// If set to `true`, transpositions are counted as one change instead of two, defaults to
    /// `true`
    pub fn transpositions(mut self, transpositions: bool) -> Self {
        self.transpositions = Some(transpositions);
        self
    }

    /// Minimum length of the input before fuzzy suggestions are returned, defaults `3`
    pub fn min_length(mut self, min_length: u64) -> Self {
        self.min_length = Some(min_length);
        self
    }

    /// Minimum length of the input, which is not checked for fuzzy alternatives, defaults to `1`
    pub fn prefix_length(mut self, prefix_length: u64) -> Self {
        self.prefix_length = Some(prefix_length);
        self
    }

    /// If `true`, all measurements (like fuzzy edit distance, transpositions, and lengths) are
    /// measured in Unicode code points instead of in bytes. This is slightly slower than raw
    /// bytes, so it is set to `false` by default.
    pub fn unicode_aware(mut self, unicode_aware: bool) -> Self {
        self.unicode_aware = Some(unicode_aware);
        self
    }
}
