use crate::search::*;
use crate::util::*;

/// A query that accepts any other query as base64 encoded string.
///
/// This query is more useful in the context of the Java high-level REST client
/// or transport client to also accept queries as json formatted string. In
/// these cases queries can be specified as a json or yaml formatted string or
/// as a query builder (which is a available in the Java high-level REST client).
///
/// To create wrapper query:
/// ```
/// # use elasticsearch_dsl::queries::*;
/// # use elasticsearch_dsl::queries::params::*;
/// # let query =
/// Query::wrapper("eyJ0ZXJtIiA6IHsgInVzZXIuaWQiIDogImtpbWNoeSIgfX0=");
/// ```
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-wrapper-query.html>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct WrapperQuery {
    query: String,
}

impl Query {
    /// Creates an instance of [`WrapperQuery`]
    pub fn wrapper<S>(query: S) -> WrapperQuery
    where
        S: ToString,
    {
        WrapperQuery {
            query: query.to_string(),
        }
    }
}

impl ShouldSkip for WrapperQuery {}

serialize_with_root!("wrapper": WrapperQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::wrapper("eyJ0ZXJtIiA6IHsgInVzZXIuaWQiIDogImtpbWNoeSIgfX0="),
            json!({ "wrapper": { "query": "eyJ0ZXJtIiA6IHsgInVzZXIuaWQiIDogImtpbWNoeSIgfX0=" } }),
        );
    }
}
