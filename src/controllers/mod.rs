use clockwork::{Routes, UrlParams};
use clockwork::route_model::{self, RouteModel};

pub fn register(routes: &mut Routes) {
    routes.register("/", index);
    routes.register("/about", about);
    routes.register("/number/:num", route_model::wrap(number));
}

fn index(_: UrlParams) -> String {
    "<html><body><h1>Index</h1></body></html>".into()
}

fn about(_: UrlParams) -> String {
    "<html><body><h1>About</h1></body></html>".into()
}

fn number(model: NumberModel) -> String {
    format!("<html><body><h1>Number #{}</h1></body></html>", model.num)
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
