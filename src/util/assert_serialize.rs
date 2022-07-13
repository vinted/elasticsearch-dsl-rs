/// Tests if a type is serialized to correct JSON [`Value`]
#[cfg(test)]
pub(crate) fn assert_serialize<S>(subject: S, expectation: serde_json::Value)
where
    S: serde::Serialize,
{
    let string = serde_json::to_string(&subject).unwrap();
    let result: serde_json::Value = serde_json::from_str(&string).unwrap();

    pretty_assertions::assert_str_eq!(
        serde_json::to_string_pretty(&result).unwrap(),
        serde_json::to_string_pretty(&expectation).unwrap(),
    );
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

/// Tests if an aggregation is serialized to correct JSON [`Value`]
#[cfg(test)]
pub(crate) fn assert_serialize_aggregation<S>(subject: S, expectation: serde_json::Value)
where
    S: Into<crate::Aggregation>,
{
    let subject = crate::Search::new().aggregate("aggregation_name", subject);
    let expectation = json!({ "aggs": { "aggregation_name": expectation } });

    assert_serialize(subject, expectation)
}
