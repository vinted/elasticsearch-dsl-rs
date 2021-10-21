use elasticsearch_dsl::search::*;

fn main() {
    let search = Search::new()
        .size(10u64)
        .query(Query::term("user.id", "kimchy").boost(1));

    println!("{}", serde_json::to_string_pretty(&search).unwrap());
}
