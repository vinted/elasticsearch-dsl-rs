use super::Time;

/// A point in time (PIT) is a point that represents a consistent view of the data at that time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PointInTime {
    id: String,
    keep_alive: Time,
}

impl PointInTime {
    /// Creates a new instance of [`PointInTime`].
    ///
    /// - The `id` parameter tells Elasticsearch to execute the request using contexts from this point in time.
    /// - The `keep_alive` parameter tells Elasticsearch how long it should extend the time to live of the point in time.
    pub fn new<T>(id: T, keep_alive: Time) -> Self
    where
        T: ToString,
    {
        Self {
            id: id.to_string(),
            keep_alive,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{util::assert_serialize, Search};

    #[test]
    fn adds_boolean() {
        assert_serialize(
            Search::new().pit(PointInTime::new("46ToAwMDaWR5BXV1aWQyKwZub2RlXzMAAAAAAAAAACoBYwADaWR4BXV1aWQxAgZub2RlXzEAAAAAAAAAAAEBYQADaWR5BXV1aWQyKgZub2RlXzIAAAAAAAAAAAwBYgACBXV1aWQyAAAFdXVpZDEAAQltYXRjaF9hbGw_gAAAAA", Time::Minutes(1))),
            json!({
                "pit": {
                    "id": "46ToAwMDaWR5BXV1aWQyKwZub2RlXzMAAAAAAAAAACoBYwADaWR4BXV1aWQxAgZub2RlXzEAAAAAAAAAAAEBYQADaWR5BXV1aWQyKgZub2RlXzIAAAAAAAAAAAwBYgACBXV1aWQyAAAFdXVpZDEAAQltYXRjaF9hbGw_gAAAAA",
                    "keep_alive": "1m"
                }
            }),
        );
    }
}
