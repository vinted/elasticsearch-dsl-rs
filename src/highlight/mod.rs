//! Highlighters enable you to get highlighted snippets from one or more fields in your search
//! results so you can show users where the query matches are.
//!
//! When you request highlights, the
//! response contains an additional `highlight` element for each search hit that includes the
//! highlighted fields and the highlighted fragments.
//!
//! # Offsets Strategy
//!
//! To create meaningful search snippets from the terms being queried, the highlighter needs to
//! know the start and end character offsets of each word in the original text. These offsets can
//! be obtained from:
//!
//! - The postings list. If `index_options` is set to `offsets` in the mapping, the
//! [`unified` highlighter](UnifiedHighlighter) uses this information to highlight documents
//! without re-analyzing the text. It re-runs the original query directly on the postings and
//! extracts the matching offsets from the index, limiting the collection to the highlighted
//! documents. This is important if you have large fields because it doesn’t require reanalyzing
//! the text to be highlighted. It also requires less disk space than using `term_vectors`.
//! - Term vectors. If `term_vector` information is provided by setting `term_vector` to
//! `with_positions_offsets` in the mapping, the [`unified` highlighter](UnifiedHighlighter)
//! automatically uses the `term_vector` to highlight the field. It’s fast especially for large
//! fields (> `1MB`) and for highlighting multi-term queries like `prefix` or `wildcard` because it
//! can access the dictionary of terms for each document. The
//! [`fvh` highlighter](FastVectorHighlighter) always uses term vectors.
//! - Plain highlighting. This mode is used by the [`unified`](UnifiedHighlighter) when there is no
//! other alternative. It creates a tiny in-memory index and re-runs the original query criteria
//! through Lucene’s query execution planner to get access to low-level match information on the
//! current document. This is repeated for every field and every document that needs highlighting.
//! The [`plain` highlighter](PlainHighlighter) always uses plain highlighting.
//!
//! > **Warning**</br>
//! > Plain highlighting for large texts may require substantial amount of time and memory. To
//! protect against this, the maximum number of text characters that will be analyzed has been
//! limited to 1000000. This default limit can be changed for a particular index with the index
//! setting `index.highlight.max_analyzed_offset`.
//!
//! <https://www.elastic.co/guide/en/elasticsearch/reference/current/highlighting.html>

mod boundary_scanner;
mod encoder;
mod fragmenter;
mod highlighter;
mod matched_fields;
mod order;
mod tags;

use crate::ShouldSkip;
use std::collections::HashMap;

pub use self::boundary_scanner::*;
pub use self::encoder::*;
pub use self::fragmenter::*;
pub use self::highlighter::*;
/// Reexports
pub use self::matched_fields::*;
pub use self::order::*;
pub use self::tags::*;

/// Highlight structure
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Highlight {
    #[serde(flatten, skip_serializing_if = "ShouldSkip::should_skip")]
    highlighter: Option<Highlighter>,
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fields: HashMap<String, Highlighter>,
}

impl Highlight {
    /// Creates a new instance of [Highlight](Highlight)
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets highlighter settings
    pub fn highlighter(mut self, highlighter: impl Into<Highlighter>) -> Self {
        self.highlighter = Some(highlighter.into());
        self
    }

    /// Adds field or field pattern to highlighter
    pub fn field(mut self, field: impl Into<String>) -> Self {
        let _ = self.fields.insert(field.into(), Default::default());
        self
    }

    /// Adds field or field pattern to highlighter
    pub fn field_highlighter(
        mut self,
        field: impl Into<String>,
        highlighter: impl Into<Highlighter>,
    ) -> Self {
        let _ = self.fields.insert(field.into(), highlighter.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_serialization! {
        default(Highlight::new(), json!({}));

        with_multiple_fields(
            Highlight::new().field("field1").field("field2").field("field.*"),
            json!({
                "fields": {
                    "field1": {},
                    "field2": {},
                    "field.*": {},
                }
            })
        );

        with_global_highlighter_settings(
            Highlight::new()
                .highlighter(Highlighter::new().tags((["<eim>"], ["</eim>"])))
                .field("field1").field("field2").field("field.*"),
            json!({
                "pre_tags": ["<eim>"],
                "post_tags": ["</eim>"],
                "fields": {
                    "field1": {},
                    "field2": {},
                    "field.*": {},
                }
            })
        );

        with_highlighter_settings(
            Highlight::new()
                .highlighter(Highlighter::new()
                    .tags((["<eim>"], ["</eim>"]))
                    .fvh()
                    .matched_fields(["one", "two", "three"])
                )
                .field("field1")
                .field("field2")
                .field_highlighter("field.*", Highlighter::new().plain().no_match_size(2u32)),
            json!({
                "pre_tags": ["<eim>"],
                "post_tags": ["</eim>"],
                "matched_fields": ["one", "two", "three"],
                "type": "fvh",
                "fields": {
                    "field1": {},
                    "field2": {},
                    "field.*": {
                        "type": "plain",
                        "no_match_size": 2,
                    },
                }
            })
        );
    }
}
