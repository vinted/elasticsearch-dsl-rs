use serde::ser::{Serialize, Serializer};

/// You can use the flags parameter to enable more optional operators for Lucene’s regular
/// expression engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegexpFlag {
    /// Enables all optional operators.
    All,

    /// Enables the `~` operator. You can use `~` to negate the shortest following pattern.
    /// For example:
    ///
    /// `a~bc   # matches 'adc' and 'aec' but not 'abc'`
    Complement,

    /// Enables the `<>` operators. You can use `<>` to match a numeric range. For example:
    ///
    /// `foo<1-100>      # matches 'foo1', 'foo2' ... 'foo99', 'foo100'`
    /// `foo<01-100>     # matches 'foo01', 'foo02' ... 'foo99', 'foo100'`
    Interval,

    /// Enables the `&` operator, which acts as an AND operator. The match will succeed if patterns
    /// on both the left side AND the right side matches. For example:
    ///
    /// `aaa.+&.+bbb  # matches 'aaabbb'`
    Intersection,

    /// Enables the `@` operator. You can use @ to match any entire string.
    ///
    /// You can combine the `@` operator with `&` and `~` operators to create an
    /// "everything except" logic. For example:
    ///
    /// `@&~(abc.+)  # matches everything except terms beginning with 'abc'`
    Anystring,
}

impl From<RegexpFlag> for &'static str {
    fn from(value: RegexpFlag) -> Self {
        match value {
            RegexpFlag::All => "ALL",
            RegexpFlag::Complement => "COMPLEMENT",
            RegexpFlag::Interval => "INTERVAL",
            RegexpFlag::Intersection => "INTERSECTION",
            RegexpFlag::Anystring => "ANYSTRING",
        }
    }
}

impl From<RegexpFlag> for String {
    fn from(value: RegexpFlag) -> Self {
        <&'static str>::from(value).to_string()
    }
}

impl std::fmt::Display for RegexpFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <&'static str>::from(*self).fmt(f)
    }
}

impl Serialize for RegexpFlag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        <&'static str>::from(*self).serialize(serializer)
    }
}
