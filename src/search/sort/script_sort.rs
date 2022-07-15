use super::SortOrder;
use crate::util::ShouldSkip;
use crate::{Script, ScriptSortType};
use serde::Serialize;

/// Sorts search hits by script result
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#script-based-sorting>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(remote = "Self")]
pub struct ScriptSort {
    script: Script,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    order: Option<SortOrder>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    r#type: Option<ScriptSortType>,
}

impl ScriptSort {
    /// Creates an instance of [ScriptSort]
    pub fn new(script: Script) -> Self {
        Self {
            script,
            order: None,
            r#type: None,
        }
    }

    /// Creates an instance of [ScriptSort] by ascending order
    pub fn ascending(script: Script) -> Self {
        Self::new(script).order(SortOrder::Asc)
    }

    /// Creates an instance of [ScriptSort] by descending order
    pub fn descending(script: Script) -> Self {
        Self::new(script).order(SortOrder::Desc)
    }

    /// Explicit order
    ///
    /// <https://www.elastic.co/guide/en/elasticsearch/reference/current/sort-search-results.html#_sort_order>
    pub fn order(mut self, order: SortOrder) -> Self {
        self.order = Some(order);
        self
    }

    /// Sort type for script result
    pub fn r#type(mut self, r#type: ScriptSortType) -> Self {
        self.r#type = Some(r#type);
        self
    }
}

impl IntoIterator for ScriptSort {
    type Item = Self;

    type IntoIter = std::option::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Some(self).into_iter()
    }
}

serialize_with_root!("_script": ScriptSort);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::assert_serialize;

    #[test]
    fn serialization() {
        assert_serialize(
            ScriptSort::ascending(
                Script::source("doc['numberOfCommits'].value * params.factor").param("factor", 1.1),
            )
            .r#type(ScriptSortType::Number),
            json!({
                "_script": {
                    "order": "asc",
                    "type": "number",
                    "script": {
                        "source": "doc['numberOfCommits'].value * params.factor",
                        "params": {
                            "factor": 1.1
                        }
                    }
                }
            }),
        );
    }
}
