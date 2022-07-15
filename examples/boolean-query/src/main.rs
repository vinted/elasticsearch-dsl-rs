use elasticsearch_dsl::*;

fn main() {
    let search = Search::new()
        .source(false)
        .from(0)
        .size(10)
        .stats("boolean-query")
        .query(
            Query::bool()
                .must(Query::term("user.id", "kimchy"))
                .filter(Query::term("tags", "production"))
                .must_not(Query::range("age").gte(10).lte(10))
                .should([Query::term("tags", "env1"), Query::term("tags", "deployed")])
                .minimum_should_match("1")
                .boost(1),
        )
        .rescore(Rescore::new(Query::term("field", 1)));

    println!("{}", serde_json::to_string_pretty(&search).unwrap());
}
