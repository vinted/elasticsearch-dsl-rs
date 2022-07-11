use crate::util::ShouldSkip;

/// Nested document metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedIdentity {
    /// Field
    pub field: String,

    /// Offset
    pub offset: u64,

    /// Nested document metadata
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", rename = "_nested")]
    pub nested: Option<Box<NestedIdentity>>,
}
