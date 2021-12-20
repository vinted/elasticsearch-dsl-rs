use crate::util::*;
use std::borrow::Cow;

/// Search text
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Text(Option<String>);

impl ShouldSkip for Text {
    fn should_skip(&self) -> bool {
        self.0.as_ref().map_or(true, ShouldSkip::should_skip)
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self(Some(value))
    }
}

impl From<Option<String>> for Text {
    fn from(value: Option<String>) -> Self {
        Self(value)
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self(Some(value.into()))
    }
}

impl From<Option<&str>> for Text {
    fn from(value: Option<&str>) -> Self {
        Self(value.map(Into::into))
    }
}

impl<'a> From<Cow<'a, str>> for Text {
    fn from(value: Cow<'a, str>) -> Self {
        Self(Some(value.into()))
    }
}

impl<'a> From<Option<Cow<'a, str>>> for Text {
    fn from(value: Option<Cow<'a, str>>) -> Self {
        Self(value.map(Into::into))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_correctly() {
        assert!(Text::from(None::<String>).should_skip());
        assert!(Text::from("").should_skip());
        assert!(Text::from("  ").should_skip());
    }

    #[test]
    fn compares_correctly() {
        assert_eq!(Text::from("abc"), Text::from(Some("abc")));
    }
}
