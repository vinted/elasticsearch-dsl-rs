use crate::search::*;
use crate::util::*;

/// Highlighter settings
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Highlighter {
    /// Default highlighter
    Default(DefaultHighlighter),

    /// Fast vector highlighter
    Fvh(FastVectorHighlighter),

    /// Plain highlighter
    Plain(PlainHighlighter),

    /// Unified highlighter
    Unified(UnifiedHighlighter),
}

/// Highlighting settings can be set on a global level and overridden at the field level
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct DefaultHighlighter {
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boundary_chars: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boundary_max_scan: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    encoder: Option<Encoder>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    force_source: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fragment_size: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    highlight_query: Option<Query>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    no_match_size: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    number_of_fragments: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    order: Option<Order>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pre_tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    post_tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    require_field_match: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip", flatten)]
    tags: Option<Tags>,
}

/// The `fvh` highlighter uses the Lucene Fast Vector highlighter. This highlighter can be used on
/// fields with `term_vector` set to `with_positions_offsets` in the mapping. The fast vector
/// highlighter:
///
/// - Can be customized with a
/// [boundary_scanner](https://www.elastic.co/guide/en/elasticsearch/reference/current/highlighting.html#boundary-scanners).
/// - Requires setting `term_vector` to `with_positions_offsets` which increases the size of the
/// index
/// - Can combine matches from multiple fields into one result. See
/// [`matched_fields`](FastVectorHighlighter::matched_fields)
/// - Can assign different weights to matches at different positions allowing for things like
/// phrase matches being sorted above term matches when highlighting a Boosting Query that boosts
/// phrase matches over term matches
///
/// > **Warning**</br>
/// > The `fvh` highlighter does not support span queries. If you need support for span queries,
/// try an alternative highlighter, such as the `unified` highlighter.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/highlighting.html#fast-vector-highlighter>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FastVectorHighlighter {
    // Common
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boundary_chars: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boundary_max_scan: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    encoder: Option<Encoder>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    force_source: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fragment_size: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    highlight_query: Option<Query>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    no_match_size: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    number_of_fragments: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    order: Option<Order>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pre_tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    post_tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    require_field_match: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip", flatten)]
    tags: Option<Tags>,

    // Highlighter specific
    r#type: &'static str,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boundary_scanner: Option<FvhBoundaryScanner>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fragment_offset: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    matched_fields: Option<MatchedFields>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    phrase_limit: Option<u32>,
}

/// The `plain` highlighter uses the standard Lucene highlighter. It attempts to reflect the query
/// matching logic in terms of understanding word importance and any word positioning criteria in
/// phrase queries.
///
/// > **Warning**<br/>
/// > The `plain` highlighter works best for highlighting simple query matches in a single field.
/// To accurately reflect query logic, it creates a tiny in-memory index and re-runs the original
/// query criteria through Lucene’s query execution planner to get access to low-level match
/// information for the current document. This is repeated for every field and every document that
/// needs to be highlighted. If you want to highlight a lot of fields in a lot of documents with
/// complex queries, we recommend using the `unified` highlighter on `postings` or `term_vector`
/// fields.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/highlighting.html#plain-highlighter>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlainHighlighter {
    // Common
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boundary_chars: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boundary_max_scan: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    encoder: Option<Encoder>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    force_source: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fragment_size: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    highlight_query: Option<Query>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    no_match_size: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    number_of_fragments: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    order: Option<Order>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pre_tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    post_tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    require_field_match: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip", flatten)]
    tags: Option<Tags>,

    // Highlighter specific
    r#type: &'static str,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fragmenter: Option<Fragmenter>,
}

/// The `unified` highlighter uses the Lucene Unified Highlighter. This highlighter breaks the text
/// into sentences and uses the BM25 algorithm to score individual sentences as if they were
/// documents in the corpus. It also supports accurate phrase and multi-term (fuzzy, prefix, regex)
/// highlighting. This is the default highlighter.
///
/// <https://www.elastic.co/guide/en/elasticsearch/reference/current/highlighting.html#unified-highlighter>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UnifiedHighlighter {
    // Common
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boundary_chars: Option<String>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    boundary_max_scan: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    encoder: Option<Encoder>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    force_source: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    fragment_size: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    highlight_query: Option<Query>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    no_match_size: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    number_of_fragments: Option<u32>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    order: Option<Order>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pre_tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    post_tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    require_field_match: Option<bool>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip", flatten)]
    tags: Option<Tags>,

    // Highlighter specific
    r#type: &'static str,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip", flatten)]
    boundary_scanner: Option<UnifiedBoundaryScanner>,
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::Default(Default::default())
    }
}

impl Highlighter {
    /// Creates a new instance of [DefaultHighlighter](DefaultHighlighter)
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DefaultHighlighter {
        Default::default()
    }

    /// Creates a new instance of [FastVectorHighlighter](FastVectorHighlighter)
    pub fn fvh() -> FastVectorHighlighter {
        FastVectorHighlighter::default()
    }

    /// Creates a new instance of [PlainHighlighter](PlainHighlighter)
    pub fn plain() -> PlainHighlighter {
        PlainHighlighter::default()
    }

    /// Creates a new instance of [UnifiedHighlighter](UnifiedHighlighter)
    pub fn unified() -> UnifiedHighlighter {
        UnifiedHighlighter::default()
    }
}

impl Default for FastVectorHighlighter {
    fn default() -> Self {
        Self {
            r#type: "fvh",
            boundary_chars: None,
            boundary_max_scan: None,
            encoder: None,
            force_source: None,
            fragment_size: None,
            highlight_query: None,
            no_match_size: None,
            number_of_fragments: None,
            order: None,
            pre_tags: None,
            post_tags: None,
            require_field_match: None,
            tags: None,
            boundary_scanner: None,
            fragment_offset: None,
            matched_fields: None,
            phrase_limit: None,
        }
    }
}

impl Default for PlainHighlighter {
    fn default() -> Self {
        Self {
            r#type: "plain",
            boundary_chars: None,
            boundary_max_scan: None,
            encoder: None,
            force_source: None,
            fragment_size: None,
            highlight_query: None,
            no_match_size: None,
            number_of_fragments: None,
            order: None,
            pre_tags: None,
            post_tags: None,
            require_field_match: None,
            tags: None,
            fragmenter: None,
        }
    }
}

impl Default for UnifiedHighlighter {
    fn default() -> Self {
        Self {
            r#type: "unified",
            boundary_chars: None,
            boundary_max_scan: None,
            encoder: None,
            force_source: None,
            fragment_size: None,
            highlight_query: None,
            no_match_size: None,
            number_of_fragments: None,
            order: None,
            pre_tags: None,
            post_tags: None,
            require_field_match: None,
            tags: None,
            boundary_scanner: None,
        }
    }
}

impl From<DefaultHighlighter> for Highlighter {
    fn from(highlighter: DefaultHighlighter) -> Self {
        Self::Default(highlighter)
    }
}

impl From<FastVectorHighlighter> for Highlighter {
    fn from(highlighter: FastVectorHighlighter) -> Self {
        Self::Fvh(highlighter)
    }
}

impl From<PlainHighlighter> for Highlighter {
    fn from(highlighter: PlainHighlighter) -> Self {
        Self::Plain(highlighter)
    }
}

impl From<UnifiedHighlighter> for Highlighter {
    fn from(highlighter: UnifiedHighlighter) -> Self {
        Self::Unified(highlighter)
    }
}

macro_rules! add_highlighter_methods {
    () => {
        /// A string that contains each boundary character. Defaults to `.,!? \t\n`.
        pub fn boundary_chars(mut self, boundary_chars: impl Into<String>) -> Self {
            self.boundary_chars = Some(boundary_chars.into());
            self
        }

        /// How far to scan for boundary characters. Defaults to `20`.
        pub fn boundary_max_scan(mut self, boundary_max_scan: u32) -> Self {
            self.boundary_max_scan = Some(boundary_max_scan);
            self
        }

        /// Indicates if the snippet should be HTML encoded.
        pub fn encoder(mut self, encoder: Encoder) -> Self {
            self.encoder = Some(encoder);
            self
        }

        /// Highlight based on the source even if the field is stored separately. Defaults to `false`.
        pub fn force_source(mut self, force_source: bool) -> Self {
            self.force_source = Some(force_source);
            self
        }

        /// The size of the highlighted fragment in characters. Defaults to
        /// `100`.
        pub fn fragment_size(mut self, fragment_size: u32) -> Self {
            self.fragment_size = Some(fragment_size);
            self
        }

        /// Highlight matches for a query other than the search query. This is especially useful if you
        /// use a rescore query because those are not taken into account by highlighting by default.
        ///
        /// > **Warning**<br/>
        /// > Elasticsearch does not validate that `highlight_query` contains the search query in any
        /// way so it is possible to define it so legitimate query results are not highlighted.
        /// Generally, you should include the search query as part of the `highlight_query`.
        pub fn highlight_query(mut self, highlight_query: impl Into<Query>) -> Self {
            self.highlight_query = Some(highlight_query.into());
            self
        }

        /// The amount of text you want to return from the beginning of the field if there are no
        /// matching fragments to highlight. Defaults to `0` (nothing is returned).
        pub fn no_match_size(mut self, no_match_size: u32) -> Self {
            self.no_match_size = Some(no_match_size);
            self
        }

        /// The maximum number of fragments to return. If the number of fragments is set to `0`, no
        /// fragments are returned. Instead, the entire field contents are highlighted and returned.
        /// This can be handy when you need to highlight short texts such as a title or address, but
        /// fragmentation is not required. If `number_of_fragments` is `0`, `fragment_size` is ignored.
        /// Defaults to `5`.
        pub fn number_of_fragments(mut self, number_of_fragments: u32) -> Self {
            self.number_of_fragments = Some(number_of_fragments);
            self
        }

        /// Sorts highlighted fragments by score when set to [`score`](Order::Score). By default,
        /// fragments will be output in the order they appear in the field
        /// (order: [`none`](Order::None)). Setting this option to [`score`](Order::Score) will output
        /// the most relevant fragments first. Each highlighter applies its own logic to compute
        /// relevancy scores. See the document
        /// [How highlighters work internally](https://www.elastic.co/guide/en/elasticsearch/reference/current/highlighting.html#how-es-highlighters-work-internally)
        /// for more details how different highlighters find the best fragments.
        pub fn order(mut self, order: Order) -> Self {
            self.order = Some(order);
            self
        }

        /// By default, only fields that contains a query match are highlighted. Set
        /// `require_field_match` to `false` to highlight all fields. Defaults to `true`.
        pub fn require_field_match(mut self, require_field_match: bool) -> Self {
            self.require_field_match = Some(require_field_match);
            self
        }

        /// Set to `styled` to use the built-in tag schema or use custom tags
        pub fn tags(mut self, tags: impl Into<Tags>) -> Self {
            self.tags = Some(tags.into());
            self
        }
    };
}

macro_rules! convert_to_highlighter {
    ($method:tt, $struct:tt) => {
        /// Converts [Highlighter](Highlighter) to specific highlighter
        pub fn $method(self) -> $struct {
            $struct {
                boundary_chars: self.boundary_chars,
                boundary_max_scan: self.boundary_max_scan,
                encoder: self.encoder,
                force_source: self.force_source,
                fragment_size: self.fragment_size,
                highlight_query: self.highlight_query,
                no_match_size: self.no_match_size,
                number_of_fragments: self.number_of_fragments,
                order: self.order,
                pre_tags: self.pre_tags,
                post_tags: self.post_tags,
                require_field_match: self.require_field_match,
                tags: self.tags,
                ..Default::default()
            }
        }
    };
}

impl DefaultHighlighter {
    /// Creates a new [Highlighter](Highlighter) instance
    pub fn new() -> Self {
        Default::default()
    }

    add_highlighter_methods!();
    convert_to_highlighter!(fvh, FastVectorHighlighter);
    convert_to_highlighter!(plain, PlainHighlighter);
    convert_to_highlighter!(unified, UnifiedHighlighter);
}

impl FastVectorHighlighter {
    /// Creates a new [FastVectorHighlighter](FastVectorHighlighter) instance
    pub fn new() -> Self {
        Default::default()
    }

    add_highlighter_methods!();

    /// Specifies how to break the highlighted fragments.
    pub fn boundary_scanner(mut self, boundary_scanner: FvhBoundaryScanner) -> Self {
        self.boundary_scanner = Some(boundary_scanner);
        self
    }

    /// Controls the margin from which you want to start highlighting.
    pub fn fragment_offset(mut self, fragment_offset: u32) -> Self {
        self.fragment_offset = Some(fragment_offset);
        self
    }

    /// Combine matches on multiple fields to highlight a single field. This is most intuitive for
    /// multi-fields that analyze the same string in different ways. All `matched_fields` must have
    /// `term_vector` set to `with_positions_offsets`, but only the field to which the matches are
    /// combined is loaded so only that field benefits from having store set to yes.
    pub fn matched_fields(mut self, matched_fields: impl Into<MatchedFields>) -> Self {
        self.matched_fields = Some(matched_fields.into());
        self
    }

    /// Controls the number of matching phrases in a document that are considered. Prevents the
    /// highlighter from analyzing too many phrases and consuming too much memory. When using
    /// `matched_fields`, `phrase_limit` phrases per matched field are considered. Raising the
    /// limit increases query time and consumes more memory. Defaults to 256.
    pub fn phrase_limit(mut self, phrase_limit: u32) -> Self {
        self.phrase_limit = Some(phrase_limit);
        self
    }
}

impl PlainHighlighter {
    /// Creates a new [PlainHighlighter](PlainHighlighter) instance
    pub fn new() -> Self {
        Default::default()
    }

    add_highlighter_methods!();

    /// Specifies how text should be broken up in highlight snippets.
    pub fn fragmenter(mut self, fragmenter: Fragmenter) -> Self {
        self.fragmenter = Some(fragmenter);
        self
    }
}

impl UnifiedHighlighter {
    /// Creates a new [UnifiedHighlighter](UnifiedHighlighter) instance
    pub fn new() -> Self {
        Default::default()
    }

    add_highlighter_methods!();

    /// Specifies how to break the highlighted fragments.
    pub fn boundary_scanner(mut self, boundary_scanner: UnifiedBoundaryScanner) -> Self {
        self.boundary_scanner = Some(boundary_scanner);
        self
    }
}
