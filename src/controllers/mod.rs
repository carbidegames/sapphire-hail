use std::collections::BTreeMap;
use clockwork::{Routes, UrlParams};
use clockwork::route_model::{self, RouteModel};
use webutil::HtmlString;
use handlebars::Handlebars;
use rustc_serialize::json::{Json, ToJson};

pub fn register(routes: &mut Routes) {
    routes.register("/", index);
    routes.register("/about", about);
    routes.register("/number/:num", route_model::wrap(number));
}

fn index(_: UrlParams) -> HtmlString {
    HtmlString::bless("<html><body><h1>Index</h1></body></html>")
}

fn about(_: UrlParams) -> HtmlString {
    HtmlString::bless("<html><body><h1>About</h1></body></html>")
}

fn number(model: NumberModel) -> HtmlString {
    // Set up the template
    let source = "<html><body><h1>Number #{{num}}</h1></body></html>";
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("number", source.to_string())
        .ok().unwrap();

    // Render the template with the model's data
    let html = handlebars.render("number", &model).unwrap();
    HtmlString::bless(html)
}

struct NumberModel {
    num: String,
}

impl RouteModel for NumberModel {
    fn from(url: UrlParams) -> Self {
        NumberModel {
            num: url.get("num").unwrap().into()
        }
    }
}

impl ToJson for NumberModel {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("num".to_string(), self.num.to_json());
        m.to_json()
    }
}
