/// Tests if a type is serialized to correct JSON [`Value`]
#[cfg(test)]
pub(crate) fn assert_serialize<T>(subject: T, expectation: serde_json::Value)
where
    T: serde::Serialize,
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
pub(crate) fn assert_serialize_query<T>(subject: T, expectation: serde_json::Value)
where
    T: Into<crate::Query>,
{
    let subject = crate::Search::new().query(subject);
    let expectation = json!({ "query": expectation });

    assert_serialize(subject, expectation)
}

/// Tests if an aggregation is serialized to correct JSON [`Value`]
#[cfg(test)]
pub(crate) fn assert_serialize_aggregation<T>(subject: T, expectation: serde_json::Value)
where
    T: Into<crate::Aggregation>,
{
    let subject = crate::Search::new().aggregate("aggregation_name", subject);
    let expectation = json!({ "aggs": { "aggregation_name": expectation } });

    assert_serialize(subject, expectation)
}

/// Tests if sorting criteria is serialized to correct JSON [`Value`]
#[cfg(test)]
pub(crate) fn assert_serialize_sort<T>(subject: T, expectation: serde_json::Value)
where
    T: IntoIterator,
    T::Item: Into<crate::Sort>,
{
    let subject = crate::Search::new().sort(subject);
    let expectation = json!({ "sort": expectation });

    assert_serialize(subject, expectation)
}

/// Tests if rescoring criteria is serialized to correct JSON [`Value`]
#[cfg(test)]
pub(crate) fn assert_serialize_rescore<T>(subject: T, expectation: serde_json::Value)
where
    T: IntoIterator,
    T::Item: Into<crate::Rescore>,
{
    let subject = crate::Search::new().rescore(subject);
    let expectation = json!({ "rescore": [expectation] });

    assert_serialize(subject, expectation)
}
