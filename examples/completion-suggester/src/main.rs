use elasticsearch_dsl::*;

fn main() {
    let search = Search::new().source("suggest").suggest(
        "song-suggest",
        Suggester::completion("suggest", "nir")
            .size(5)
            .skip_duplicates(true)
            .fuzzy(
                SuggestFuzziness::new()
                    .transpositions(true)
                    .fuzziness(2..4)
                    .min_length(4)
                    .prefix_length(2)
                    .unicode_aware(false),
            ),
    );

    println!("{}", serde_json::to_string_pretty(&search).unwrap());
}
