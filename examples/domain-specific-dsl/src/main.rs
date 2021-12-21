use elasticsearch_dsl::*;

fn main() {
    let request = Request {
        category_id: 1,
        company_id: 2,
        brand_ids: vec![1, 2],
        user: User {
            id: 1,
            name: String::from("kimchy"),
        },
        country: Country {
            id: 1,
            name: String::from("Gondor"),
        },
    };

    let query = RequestQuery::new(request)
        .category_id(|x| x.category_id)
        .company_id(|x| x.company_id)
        .brand_ids(|x| x.brand_ids.clone())
        .my_country_documents_only()
        .exclude_user_items()
        .finish();

    let search = Search::new().size(10).query(query);

    println!("{}", serde_json::to_string_pretty(&search).unwrap());
}

struct RequestQuery {
    request: Request,
    query: BoolQuery,
}

impl RequestQuery {
    fn new(request: Request) -> Self {
        Self {
            request,
            query: Query::bool(),
        }
    }

    fn exclude_user_items(mut self) -> Self {
        self.query = self
            .query
            .must_not(Query::term("user_id", self.request.user.id));

        self
    }

    fn my_country_documents_only(mut self) -> Self {
        self.query = self
            .query
            .filter(Query::term("country_id", self.request.country.id));

        self
    }

    fn category_id<F, T>(mut self, func: F) -> Self
    where
        F: Fn(&Request) -> T,
        T: Into<Term>,
    {
        self.query = self
            .query
            .filter(Query::term("category_id", func(&self.request)));

        self
    }

    fn brand_ids<F, T>(mut self, func: F) -> Self
    where
        F: Fn(&Request) -> T,
        T: IntoIterator,
        T::Item: Into<Term>,
    {
        self.query = self
            .query
            .filter(Query::terms("brand_id", func(&self.request)));

        self
    }

    fn company_id<F, T>(mut self, func: F) -> Self
    where
        F: Fn(&Request) -> T,
        T: Into<Term>,
    {
        self.query = self
            .query
            .filter(Query::term("company_id", func(&self.request)));

        self
    }

    fn finish(self) -> BoolQuery {
        self.query
    }
}

struct User {
    id: i32,
    name: String,
}

struct Country {
    id: i32,
    name: String,
}

struct Request {
    category_id: i32,
    company_id: i32,
    brand_ids: Vec<i32>,
    user: User,
    country: Country,
}
