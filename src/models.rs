//use clockwork::routes::{RouteModel, UrlParams};

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

#[derive(Serialize)]
pub struct NewSubmit {
    pub title: String,
}
