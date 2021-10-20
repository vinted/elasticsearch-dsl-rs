use serde::{Serialize, Serializer};
use std::ops::Range;

/// Some queries and APIs support parameters to allow inexact _fuzzy_ matching,
/// using the `fuzziness` parameter.
///
/// When querying `text` or `keyword` fields, `fuzziness` is interpreted as a
/// [Levenshtein Edit Distance](https://en.wikipedia.org/wiki/Levenshtein_distance)
/// — the number of one character changes that need to be made to one string to make it the same as another string.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#fuzziness>
#[derive(Debug, PartialEq, Clone)]
pub enum Fuzziness {
    /// Generates an edit distance based on the length of the term.
    ///
    /// `AUTO` should generally be the preferred value for `fuzziness`.
    Auto,

    /// Low and high distance arguments may be optionally provided
    /// `AUTO:[low],[high]`. If not specified, the default values are 3 and 6,
    /// equivalent to `AUTO:3,6` that make for lengths:
    ///
    /// **`0..2`**
    ///
    /// &nbsp;&nbsp;&nbsp;&nbsp;Must match exactly
    ///
    /// **`3..5`**
    ///
    /// &nbsp;&nbsp;&nbsp;&nbsp;One edit allowed
    ///
    /// **`>5`**
    ///
    /// &nbsp;&nbsp;&nbsp;&nbsp;Two edits allowed
    Range(Range<u8>),

    /// The maximum allowed Levenshtein Edit Distance (or number of edits)
    Distance(u8),
}

impl Serialize for Fuzziness {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Auto => serializer.serialize_str("AUTO"),
            Self::Range(r) => serializer.serialize_str(&format!("AUTO:{},{}", r.start, r.end)),
            Self::Distance(d) => serializer.serialize_u8(*d),
        }
    }
}

impl From<Range<u8>> for Fuzziness {
    fn from(v: Range<u8>) -> Self {
        Self::Range(v)
    }
}

impl From<u8> for Fuzziness {
    fn from(v: u8) -> Self {
        Self::Distance(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implements_from_u8() {
        let result = Fuzziness::from(8);

        let expectation = Fuzziness::Distance(8);

        assert_eq!(result, expectation);
    }

    #[test]
    fn implements_from_range_u8() {
        let result = Fuzziness::from(0..2);

        let expectation = Fuzziness::Range(0..2);

        assert_eq!(result, expectation);
    }

    #[test]
    fn serializes_auto() {
        let result = serde_json::to_string(&Fuzziness::Auto).unwrap();

        let expectation = "\"AUTO\"";

        assert_eq!(result, expectation);
    }

    #[test]
    fn serializes_auto_low_high() {
        let result = serde_json::to_string(&Fuzziness::Range(0..2)).unwrap();

        let expectation = "\"AUTO:0,2\"";

        assert_eq!(result, expectation);
    }

    #[test]
    fn serializes_distance() {
        let result = serde_json::to_string(&Fuzziness::Distance(5)).unwrap();

        let expectation = "5";

        assert_eq!(result, expectation);
    }
}
