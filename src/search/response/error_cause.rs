use crate::util::ShouldSkip;
use serde_json::Value;
use std::collections::HashMap;

/// Error cause
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorCause {
    /// Deeper error cause
    pub caused_by: Option<Box<ErrorCause>>,

    /// Error cause reason
    pub reason: Option<String>,

    /// Root error cause
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub root_cause: Vec<ErrorCause>,

    /// Exception stack trace
    pub stack_trace: Option<String>,

    /// Suppressed error causes
    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default)]
    pub suppressed: Vec<ErrorCause>,

    /// Type of error cause
    #[serde(rename = "type")]
    pub ty: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip", default, flatten)]
    additional_details: HashMap<String, Value>,
}
