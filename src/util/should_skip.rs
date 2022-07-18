use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// Trait to handle skippable queries or their values
pub(crate) trait ShouldSkip {
    /// Whether a query or a query value can be skipped
    fn should_skip(&self) -> bool {
        false
    }

    /// Inverse of the condition
    fn should_keep(&self) -> bool {
        !self.should_skip()
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

impl<T> ShouldSkip for HashSet<T> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<T> ShouldSkip for &HashSet<T> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<T> ShouldSkip for BTreeSet<T> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<T> ShouldSkip for &BTreeSet<T> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<T> ShouldSkip for &[T] {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> ShouldSkip for HashMap<K, V> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> ShouldSkip for &HashMap<K, V> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> ShouldSkip for BTreeMap<K, V> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> ShouldSkip for &BTreeMap<K, V> {
    fn should_skip(&self) -> bool {
        self.is_empty()
    }
}
