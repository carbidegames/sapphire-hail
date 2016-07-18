use std::collections::BTreeMap;
use clockwork::{Routes, UrlParams, Modules};
use clockwork::route_model::{self, RouteModel};
use clockwork_handlebars::ViewRenderer;
use webutil::HtmlString;
use rustc_serialize::json::{Json, ToJson};

pub fn register(routes: &mut Routes) {
    routes.register("/", index);
    routes.register("/about", about);
    routes.register("/number/:num", route_model::wrap(number));
}

fn index(_: &Modules, _: UrlParams) -> HtmlString {
    HtmlString::bless("<html><body><h1>Index</h1></body></html>")
}

fn about(_: &Modules, _: UrlParams) -> HtmlString {
    HtmlString::bless("<html><body><h1>About</h1></body></html>")
}

fn number(modules: &Modules, model: NumberModel) -> HtmlString {
    let views: &ViewRenderer = modules.get().unwrap();
    views.render("number", &model)
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

impl ToJson for NumberModel {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("num".into(), self.num.to_json());
        m.to_json()
    }
}
