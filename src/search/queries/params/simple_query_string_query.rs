use serde::ser::{Serialize, Serializer};

/// You can use the flags parameter to enable more optional operators for Lucene’s regular
/// expression engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimpleQueryStringQueryFlags {
    /// Enables all optional operators.
    All,

    /// Enables the `+` AND operator.
    And,

    /// Enables `\` as an escape character.
    Escape,

    /// Enables the `~N` operator after a word, where `N` is an integer
    /// denoting the allowed edit distance for matching. See
    /// [Fuzziness](https://www.elastic.co/guide/en/elasticsearch/reference/current/common-options.html#fuzziness).
    Fuzzy,

    /// Enables the `~N` operator, after a phrase where `N` is the maximum
    /// number of positions allowed between matching tokens. Synonymous to
    /// [SLOP](SimpleQueryStringQueryFlags::Slop).
    Near,

    /// Disables all operators.
    None,

    /// Enables the `-` NOT operator.
    Not,

    /// Enables the `\|` OR operator.
    Or,

    /// Enables the `"` quotes operator used to search for phrases.
    Phrase,

    /// Enables the `(` and `)` operators to control operator precedence.
    Precedence,

    /// Enables the `*` prefix operator.
    Prefix,

    /// Enables the `~N` operator, after a phrase where `N` is maximum
    /// number of positions allowed between matching tokens. Synonymous to
    /// [NEAR](SimpleQueryStringQueryFlags::Near).
    Slop,

    /// Enables whitespace as split characters.
    Whitespace,
}

impl From<SimpleQueryStringQueryFlags> for &'static str {
    fn from(value: SimpleQueryStringQueryFlags) -> Self {
        match value {
            SimpleQueryStringQueryFlags::All => "ALL",
            SimpleQueryStringQueryFlags::And => "AND",
            SimpleQueryStringQueryFlags::Escape => "ESCAPE",
            SimpleQueryStringQueryFlags::Fuzzy => "FUZZY",
            SimpleQueryStringQueryFlags::Near => "NEAR",
            SimpleQueryStringQueryFlags::None => "NONE",
            SimpleQueryStringQueryFlags::Not => "NOT",
            SimpleQueryStringQueryFlags::Or => "OR",
            SimpleQueryStringQueryFlags::Phrase => "PHRASE",
            SimpleQueryStringQueryFlags::Precedence => "PRECEDENCE",
            SimpleQueryStringQueryFlags::Prefix => "PREFIX",
            SimpleQueryStringQueryFlags::Slop => "SLOP",
            SimpleQueryStringQueryFlags::Whitespace => "WHITESPACE",
        }
    }
}

impl From<SimpleQueryStringQueryFlags> for String {
    fn from(value: SimpleQueryStringQueryFlags) -> Self {
        <&'static str>::from(value).to_string()
    }
}

impl std::fmt::Display for SimpleQueryStringQueryFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <&'static str>::from(*self).fmt(f)
    }
}

impl Serialize for SimpleQueryStringQueryFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        <&'static str>::from(*self).serialize(serializer)
    }
}
