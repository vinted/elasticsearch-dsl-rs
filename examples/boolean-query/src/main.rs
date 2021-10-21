use elasticsearch_dsl::search::*;

fn main() {
    let search = Search::new()
        .from(0u64)
        .size(10u64)
        .stats("boolean-query")
        .query(
            Query::bool()
                .must(Query::term("user.id", "kimchy"))
                .filter(Query::term("tags", "production"))
                .must_not(Query::range("age").gte(10).lte(10))
                .shoulds([Query::term("tags", "env1"), Query::term("tags", "deployed")])
                .minimum_should_match("1")
                .boost(1),
        );

    println!("{}", serde_json::to_string_pretty(&search).unwrap());
}
