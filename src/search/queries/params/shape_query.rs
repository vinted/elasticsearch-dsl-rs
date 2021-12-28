use serde::Serialize;

/// Relation between coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpatialRelation {
    /// Return all documents whose `shape` field intersects the query geometry
    Intersects,

    /// Return all documents whose `shape` field has nothing in common with the
    /// query geometry.
    Disjoint,

    /// Return all documents whose `shape` field is within the query geometry.
    Within,

    /// Return all documents whose `shape` field contains the query geometry.
    Contains,
}

impl Default for SpatialRelation {
    fn default() -> Self {
        Self::Intersects
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn serialization() {
        assert_serialize(
            [
                SpatialRelation::Intersects,
                SpatialRelation::Disjoint,
                SpatialRelation::Within,
                SpatialRelation::Contains,
            ],
            json!(["INTERSECTS", "DISJOINT", "WITHIN", "CONTAINS"]),
        );
    }
}
