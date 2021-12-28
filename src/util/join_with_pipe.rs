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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn tests_serialization() {
        #[derive(Serialize)]
        struct JoinWithPipe {
            #[serde(serialize_with = "join_with_pipe")]
            value: &'static [i32],
        }

        assert_serialize(
            JoinWithPipe { value: &[1, 2, 3] },
            json!({ "value": "1|2|3" }),
        )
    }
}
