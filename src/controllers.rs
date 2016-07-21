use std::collections::BTreeMap;
use clockwork::Modules;
use clockwork::routes::{self, RouteModel, Routes, UrlParams};
use clockwork_handlebars::ViewRenderer;
use webutil::HtmlString;
use rustc_serialize::json::{Json, ToJson};

pub fn register(routes: &mut Routes) {
    routes.register("/", index);
    routes.register("/about", about);
    routes.register("/number/:num", routes::model_handler(number));
    routes.register("/rowtest", rowtest);
    routes.register("/public/*", routes::file_handler("./public"));
}

fn index(modules: &Modules, _: UrlParams) -> HtmlString {
    let views: &ViewRenderer = modules.get().unwrap();

    views.render("hello", &HelloViewModel {text: "Index".into()})
}

fn about(modules: &Modules, _: UrlParams) -> HtmlString {
    let views: &ViewRenderer = modules.get().unwrap();

    views.render("hello", &HelloViewModel {text: "About".into()})
}

struct HelloViewModel {
    text: String,
}

impl ToJson for HelloViewModel {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("text".into(), self.text.to_json());
        m.to_json()
    }
}

fn number(modules: &Modules, model: NumberModel) -> HtmlString {
    let views: &ViewRenderer = modules.get().unwrap();

    let view_model = NumberViewModel {
        num: model.num.clone(),
        loneliest: model.num == "1"
    };

    views.render("number", &view_model)
}

struct NumberModel {
    num: String,
}

impl RouteModel for NumberModel {
    fn from(url: UrlParams) -> Self {
        NumberModel {
            num: url.get("num").unwrap()
        }
    }
}

struct NumberViewModel {
    num: String,
    loneliest: bool,
}

impl ToJson for NumberViewModel {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("num".into(), self.num.to_json());
        m.insert("loneliest".into(), self.loneliest.to_json());
        m.to_json()
    }
}

fn rowtest(modules: &Modules, _: UrlParams) -> HtmlString {
    let views: &ViewRenderer = modules.get().unwrap();

    let mut rows = Vec::new();
    for i in 0..1000 {
        rows.push(RowTestEntry {
            name: format!("User #{}", i),
            coolness: (i + 20) * 59 % 100,
            dopeness: (i + 20) * 43 % 100,
        });
    }

    views.render("rowtest", &RowTestModel {rows: rows})
}

struct RowTestModel {
    rows: Vec<RowTestEntry>
}

impl ToJson for RowTestModel {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("rows".into(), self.rows.to_json());
        m.to_json()
    }
}

struct RowTestEntry {
    name: String,
    coolness: i32,
    dopeness: i32,
}

impl ToJson for RowTestEntry {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("name".into(), self.name.to_json());
        m.insert("coolness".into(), self.coolness.to_json());
        m.insert("dopeness".into(), self.dopeness.to_json());
        m.to_json()
    }
}
