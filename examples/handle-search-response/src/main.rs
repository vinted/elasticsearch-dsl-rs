use elasticsearch_dsl::*;

fn main() {
    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct Message {
        #[serde(rename = "@timestamp")]
        timestamp: String,

        message: String,

        user_id: String,
    }

    let json = serde_json::json!({
      "took": 5,
      "timed_out": false,
      "_shards": {
        "total": 1,
        "successful": 1,
        "skipped": 0,
        "failed": 0
      },
      "hits": {
        "total": {
          "value": 20,
          "relation": "eq"
        },
        "max_score": 1.3862942,
        "hits": [
          {
            "_index": "my-index-000001",
            "_id": "0",
            "_score": 1.3862942,
            "_source": {
              "@timestamp": "2099-11-15T14:12:12",
              "message": "GET /search HTTP/1.1 200 1070000",
              "user_id": "kimchy"
            }
          }
        ]
      }
    });

    let response: SearchResponse = serde_json::from_value(json).unwrap();

    let documents = response.documents::<Message>().unwrap();

    assert_eq!(
        documents,
        vec![Message {
            timestamp: "2099-11-15T14:12:12".to_string(),
            message: "GET /search HTTP/1.1 200 1070000".to_string(),
            user_id: "kimchy".to_string(),
        }]
    );
}
