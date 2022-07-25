use super::Rescore;
use crate::util::ShouldSkip;

/// Rescoring criteria
#[derive(Default, Clone, PartialEq, Serialize)]
pub struct RescoreCollection(Vec<Rescore>);

impl std::fmt::Debug for RescoreCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoIterator for RescoreCollection {
    type Item = Rescore;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl ShouldSkip for RescoreCollection {
    fn should_skip(&self) -> bool {
        self.0.should_skip()
    }
}

impl RescoreCollection {
    /// Creates a new instance of [RescoreCollection]
    pub fn new() -> Self {
        Default::default()
    }

    /// Extends rescoring collection
    pub fn extend<T>(&mut self, rescore: T)
    where
        T: IntoIterator,
        T::Item: Into<Rescore>,
    {
        self.0.extend(
            rescore
                .into_iter()
                .map(Into::into)
                .filter(ShouldSkip::should_keep),
        )
    }
}
