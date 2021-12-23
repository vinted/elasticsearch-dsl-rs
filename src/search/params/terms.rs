use super::*;
use crate::util::*;

/// A collection of terms
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct Terms(std::collections::BTreeSet<Term>);

impl<T> From<T> for Terms
where
    T: IntoIterator,
    T::Item: Into<Term>,
{
    fn from(value: T) -> Self {
        Self(value.into_iter().map(Into::into).collect())
    }
}

impl ShouldSkip for Terms {
    fn should_skip(&self) -> bool {
        self.0.is_empty()
    }
}
