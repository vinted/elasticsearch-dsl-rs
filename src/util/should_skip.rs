/// Trait to handle skippable queries or their values
pub(crate) trait ShouldSkip {
    /// Whether a query or a query value can be skipped
    fn should_skip(&self) -> bool {
        false
    }
}

impl ShouldSkip for String {
    fn should_skip(&self) -> bool {
        self.trim().is_empty()
    }
}

impl ShouldSkip for str {
    fn should_skip(&self) -> bool {
        self.trim().is_empty()
    }
}

impl ShouldSkip for &str {
    fn should_skip(&self) -> bool {
        self.trim().is_empty()
    }
}

impl<T> ShouldSkip for Option<T> {
    fn should_skip(&self) -> bool {
        self.is_none()
    }
}

impl<T> ShouldSkip for &Option<T> {
    fn should_skip(&self) -> bool {
        self.is_none()
    }
}

impl<T> ShouldSkip for Vec<T> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<T> ShouldSkip for std::collections::HashSet<T> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<T> ShouldSkip for &std::collections::HashSet<T> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<T> ShouldSkip for std::collections::BTreeSet<T> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<T> ShouldSkip for &std::collections::BTreeSet<T> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<'a, T> ShouldSkip for &'a [T] {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> ShouldSkip for std::collections::HashMap<K, V> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> ShouldSkip for &std::collections::HashMap<K, V> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> ShouldSkip for std::collections::BTreeMap<K, V> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> ShouldSkip for &std::collections::BTreeMap<K, V> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}
