/// Tests if a type is serialized to correct JSON [`Value`]
#[cfg(test)]
pub(crate) fn assert_serialize<S>(subject: S, expectation: serde_json::Value)
where
    S: serde::Serialize,
{
    let string = serde_json::to_string(&subject).unwrap();
    let result: serde_json::Value = serde_json::from_str(&string).unwrap();

    assert_eq!(result, expectation)
}

/// Tests if a query is serialized to correct JSON [`Value`]
#[cfg(test)]
pub(crate) fn assert_serialize_query<S>(subject: S, expectation: serde_json::Value)
where
    S: Into<crate::Query>,
{
    let subject = crate::Search::new().query(subject);
    let expectation = json!({ "query": expectation });

    assert_serialize(subject, expectation)
}
