use serde::ser::{Serialize, SerializeStruct, Serializer};

/// Locale type alias
pub type Locale = String;

/// Specifies how to break the highlighted fragments. Defaults to
/// [`sentence`](UnifiedBoundaryScanner::Sentence).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnifiedBoundaryScanner {
    /// Break highlighted fragments at the next sentence boundary, as determined by Java’s
    /// [BreakIterator](https://docs.oracle.com/javase/8/docs/api/java/text/BreakIterator.html).
    /// You can specify the locale to use with `boundary_scanner_locale`.
    ///
    /// > **Warning**<br/>
    /// > The `sentence` scanner splits sentences bigger than `fragment_size` at the first word
    /// boundary next to `fragment_size`. You can set `fragment_size` to 0 to never split any
    /// sentence.
    Sentence(Option<Locale>),

    /// Break highlighted fragments at the next word boundary, as determined by Java’s
    /// [BreakIterator](https://docs.oracle.com/javase/8/docs/api/java/text/BreakIterator.html).
    /// You can specify the locale to use with `boundary_scanner_locale`.
    Word(Option<Locale>),
}

/// Specifies how to break the highlighted fragments. Defaults to
/// [`sentence`](FvhBoundaryScanner::Chars).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FvhBoundaryScanner {
    /// Use the characters specified by `boundary_chars` as highlighting boundaries. The
    /// `boundary_max_scan` setting controls how far to scan for boundary characters.
    Chars,

    /// Break highlighted fragments at the next sentence boundary, as determined by Java’s
    /// [BreakIterator](https://docs.oracle.com/javase/8/docs/api/java/text/BreakIterator.html).
    /// You can specify the locale to use with `boundary_scanner_locale`.
    Sentence(Option<Locale>),

    /// Break highlighted fragments at the next word boundary, as determined by Java’s
    /// [BreakIterator](https://docs.oracle.com/javase/8/docs/api/java/text/BreakIterator.html).
    /// You can specify the locale to use with `boundary_scanner_locale`.
    Word(Option<Locale>),
}

impl Serialize for UnifiedBoundaryScanner {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Sentence(locale) => match locale {
                Some(locale) => {
                    let mut map =
                        serializer.serialize_struct("UnifiedBoundaryScannerSentence2", 2)?;
                    map.serialize_field("boundary_scanner", "sentence")?;
                    map.serialize_field("boundary_scanner_locale", locale)?;
                    map.end()
                }
                None => {
                    let mut map =
                        serializer.serialize_struct("UnifiedBoundaryScannerSentence1", 1)?;
                    map.serialize_field("boundary_scanner", "sentence")?;
                    map.end()
                }
            },
            Self::Word(locale) => match locale {
                Some(locale) => {
                    let mut map = serializer.serialize_struct("UnifiedBoundaryScannerWord2", 2)?;
                    map.serialize_field("boundary_scanner", "word")?;
                    map.serialize_field("boundary_scanner_locale", locale)?;
                    map.end()
                }
                None => {
                    let mut map = serializer.serialize_struct("UnifiedBoundaryScannerWord1", 1)?;
                    map.serialize_field("boundary_scanner", "word")?;
                    map.end()
                }
            },
        }
    }
}

impl Serialize for FvhBoundaryScanner {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Chars => {
                let mut map = serializer.serialize_struct("FvhBoundaryScannerChars", 1)?;
                map.serialize_field("boundary_scanner", "chars")?;
                map.end()
            }
            Self::Sentence(locale) => match locale {
                Some(locale) => {
                    let mut map = serializer.serialize_struct("FvhBoundaryScannerSentence2", 2)?;
                    map.serialize_field("boundary_scanner", "sentence")?;
                    map.serialize_field("boundary_scanner_locale", locale)?;
                    map.end()
                }
                None => {
                    let mut map = serializer.serialize_struct("FvhBoundaryScannerSentence1", 1)?;
                    map.serialize_field("boundary_scanner", "sentence")?;
                    map.end()
                }
            },
            Self::Word(locale) => match locale {
                Some(locale) => {
                    let mut map = serializer.serialize_struct("FvhBoundaryScannerWord2", 2)?;
                    map.serialize_field("boundary_scanner", "word")?;
                    map.serialize_field("boundary_scanner_locale", locale)?;
                    map.end()
                }
                None => {
                    let mut map = serializer.serialize_struct("FvhBoundaryScannerWord1", 1)?;
                    map.serialize_field("boundary_scanner", "word")?;
                    map.end()
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn serialization() {
        assert_serialize(
            UnifiedBoundaryScanner::Sentence(None),
            json!({ "boundary_scanner": "sentence" }),
        );

        assert_serialize(
            UnifiedBoundaryScanner::Sentence(Some("en-US".into())),
            json!({ "boundary_scanner": "sentence", "boundary_scanner_locale": "en-US" }),
        );

        assert_serialize(
            UnifiedBoundaryScanner::Word(None),
            json!({ "boundary_scanner": "word" }),
        );

        assert_serialize(
            UnifiedBoundaryScanner::Word(Some("en-US".into())),
            json!({ "boundary_scanner": "word", "boundary_scanner_locale": "en-US" }),
        );

        assert_serialize(
            FvhBoundaryScanner::Chars,
            json!({ "boundary_scanner": "chars" }),
        );

        assert_serialize(
            FvhBoundaryScanner::Sentence(None),
            json!({ "boundary_scanner": "sentence" }),
        );

        assert_serialize(
            FvhBoundaryScanner::Sentence(Some("en-US".into())),
            json!({ "boundary_scanner": "sentence", "boundary_scanner_locale": "en-US" }),
        );

        assert_serialize(
            FvhBoundaryScanner::Word(None),
            json!({ "boundary_scanner": "word" }),
        );

        assert_serialize(
            FvhBoundaryScanner::Word(Some("en-US".into())),
            json!({ "boundary_scanner": "word", "boundary_scanner_locale": "en-US" }),
        );
    }
}
