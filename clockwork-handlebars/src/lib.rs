extern crate clockwork;
extern crate handlebars;
extern crate rustc_serialize;
extern crate webutil;

use clockwork::Module;
use handlebars::Handlebars;
use rustc_serialize::json::ToJson;
use webutil::HtmlString;

pub struct ViewRenderer {
    registry: Handlebars,
}

impl ViewRenderer {
    pub fn new() -> Self {
        let mut registry = Handlebars::new();

        //registry.register_template_string("number", source.to_string())
        //    .ok().unwrap();

        // Scan the views directory for files
        for entry in ::std::fs::read_dir("./views").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let metadata = entry.metadata().unwrap();

            // Skip non-files
            if !metadata.is_file() {
                continue;
            }

            // Add the template
            let template_name = path.file_stem().unwrap();
            registry.register_template_file(template_name.to_str().unwrap(), &path).unwrap();
        }

        ViewRenderer {
            registry: registry,
        }
    }

    pub fn render<S: ToString, M: ToJson>(&self, _view: S, model: &M) -> HtmlString {
        // TODO: Register views on new

        let html = self.registry.render("number", model).unwrap();
        HtmlString::bless(html)
    }
}

impl Module for ViewRenderer {
}
