use elasticsearch_dsl::*;

fn main() {
    let search = Search::new()
        .size(10)
        .query(Query::term("user.id", "kimchy").boost(1));

    println!("{}", serde_json::to_string_pretty(&search).unwrap());
}
