# Strongly typed Elasticsearch DSL written in Rust

This is an unofficial library and doesn't yet support all the DSL, it's still work in progress.

## Features

- Strongly typed queries
- Strongly typed aggregations
- Automatically skips empty queries making DSL pleasant to use
- Crate doesn't depend on [elasticsearch-rs](https://github.com/elastic/elasticsearch-rs) and can be used as a standalone library with any HTTP client to call Elasticsearch

## Installation

Add `elasticsearch-dsl` crate and version to Cargo.toml

```toml
[dependencies]
elasticsearch-dsl = "0.1"
```

## Documentation

Documentation for the library is available on [docs.rs](https://docs.rs/elasticsearch-dsl)

## Quick start

```rust
use elasticsearch_dsl::*;

let query = Search::new()
    .source(false)
    .stats("statistics")
    .from(0)
    .size(30)
    .query(
        Query::bool()
            .must(Query::multi_match(["title", "description"], "you know, for search"))
            .filter(Query::terms("tags", ["elasticsearch"]))
            .should(Query::term("verified", true).boost(10)),
    );
```

See examples for more.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
