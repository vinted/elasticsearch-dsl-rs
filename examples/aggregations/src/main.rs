use elasticsearch_dsl::*;

fn main() {
    let search = Search::new()
        .size(0)
        .query(Query::bool().must_not(Query::exists("country_id")))
        .aggregate(
            "country_ids",
            Aggregation::terms("country_id")
                .aggregate("catalog_ids", Aggregation::terms("catalog_id"))
                .aggregate("company_ids", Aggregation::terms("company_id"))
                .aggregate(
                    "top1",
                    Aggregation::top_hits()
                        .size(1)
                        .sort(FieldSort::descending("field")),
                ),
        );

    println!("{}", serde_json::to_string_pretty(&search).unwrap());
}
