use clockwork::Modules;
use clockwork::routes::{Routes, UrlParams};
use clockwork_handlebars::ViewRenderer;

pub fn register(routes: &mut Routes) {
    routes.register("/projects/new", index);
}

fn index(modules: &Modules, _: UrlParams) -> Vec<u8> {
    let views: &ViewRenderer = modules.get().unwrap();

    views.render("projects/new", &()).into()
}
