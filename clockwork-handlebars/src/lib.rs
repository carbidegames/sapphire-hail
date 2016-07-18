extern crate clockwork;
extern crate handlebars;
extern crate rustc_serialize;
extern crate webutil;

use clockwork::Module;
use handlebars::Handlebars;
use rustc_serialize::json::ToJson;
use webutil::HtmlString;

pub struct ViewRenderer {
}

impl ViewRenderer {
    pub fn new() -> Self {
        ViewRenderer {
        }
    }

    pub fn render<S: ToString, M: ToJson>(&self, _view: S, model: &M) -> HtmlString {
        // TODO: Register views on new

        // Set up the template
        let source = "<html><body><h1>Number #{{num}}</h1></body></html>";
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("number", source.to_string())
            .ok().unwrap();

        // Render the template with the model's data
        let html = handlebars.render("number", model).unwrap();
        HtmlString::bless(html)
    }
}

impl Module for ViewRenderer {
}
