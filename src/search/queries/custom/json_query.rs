use crate::search::*;
use crate::util::*;

/// Raw JSON query for something not yet supported.
///
/// To create JSON query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::json(serde_json::json!({ "term": { "user": "username" } }));
/// ```
/// **NOTE**: This is fallible and can lead to incorrect queries and
/// rejected search requests, use ar your own risk.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct JsonQuery(serde_json::Value);

impl Query {
    /// Creates an instance of [`JsonQuery`]
    ///
    /// - `query` - raw JSON query
    pub fn json(query: serde_json::Value) -> JsonQuery {
        JsonQuery(query)
    }
}

impl From<serde_json::Value> for Query {
    fn from(value: serde_json::Value) -> Self {
        Self::Json(JsonQuery(value))
    }
}

impl From<serde_json::Value> for JsonQuery {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}

impl ShouldSkip for JsonQuery {
    fn should_skip(&self) -> bool {
        !self.0.is_object()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize(
            Query::json(json!({ "term": { "user": "username" } })),
            json!({ "term": { "user": "username" } }),
        );
    }
}
