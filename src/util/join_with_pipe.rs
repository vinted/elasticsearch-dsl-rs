use serde::ser::{Serialize, Serializer};

pub(crate) fn join_with_pipe<S, T>(value: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    value
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("|")
        .serialize(serializer)
}
