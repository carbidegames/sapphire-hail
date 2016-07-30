use clockwork::routes::{RouteModel, UrlParams};

#[derive(Serialize)]
pub struct HelloViewModel {
    pub text: String,
}

pub struct NumberModel {
    pub num: String,
}

impl RouteModel for NumberModel {
    fn from(url: UrlParams) -> Self {
        NumberModel {
            num: url.get("num").unwrap()
        }
    }
}

#[derive(Serialize)]
pub struct NumberViewModel {
    pub num: String,
    pub loneliest: bool,
}

#[derive(Serialize)]
pub struct RowTestModel {
    pub rows: Vec<RowTestEntry>
}

#[derive(Serialize)]
pub struct RowTestEntry {
    pub name: String,
    pub coolness: i32,
    pub dopeness: i32,
}

#[derive(Serialize)]
pub struct ErrorModel {
    pub error: String,
}
