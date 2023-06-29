//! You can use the `script_fields` parameter to retrieve a
//! [script evaluation](https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting.html)
//! (based on different fields) for each hit.
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/current/search-fields.html#script-fields>

use crate::Script;

/// A script to calculate field value from the `_source` fields
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ScriptField {
    script: Script,
}

impl From<Script> for ScriptField {
    fn from(value: Script) -> Self {
        Self { script: value }
    }
}

impl From<&str> for ScriptField {
    fn from(value: &str) -> Self {
        Self {
            script: Script::source(value),
        }
    }
}

impl From<String> for ScriptField {
    fn from(value: String) -> Self {
        Self {
            script: Script::source(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{util::assert_serialize, Search};

    use super::*;

    #[test]
    fn serializes_correctly() {
        let subject = Search::new()
            .script_fields(
                "test1",
                Script::source("doc['price'].value * 2").lang("painless"),
            )
            .script_fields(
                "test2",
                Script::source("doc['price'].value * params.factor")
                    .lang("painless")
                    .param("factor", 2.0),
            )
            .script_fields("test3", "params['_source']['message']");

        let expectation = json!({
            "script_fields": {
                "test1": {
                    "script": {
                        "lang": "painless",
                        "source": "doc['price'].value * 2"
                    }
                },
                "test2": {
                    "script": {
                        "lang": "painless",
                        "source": "doc['price'].value * params.factor",
                        "params": {
                            "factor": 2.0
                        }
                    }
                },
                "test3": {
                    "script": {
                        "source": "params['_source']['message']"
                    }
                }
            }
        });

        assert_serialize(subject, expectation);
    }
}
