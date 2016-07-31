mod projects;

use clockwork::Modules;
use clockwork::routes::{Routes, UrlParams};
use clockwork_handlebars::ViewRenderer;
use models::{RowTestModel, RowTestEntry};

pub fn register(routes: &mut Routes) {
    routes.get("/", index);
    routes.get("/rowtest", rowtest);

    projects::register(routes);
}

fn index(modules: &Modules, _: UrlParams) -> Vec<u8> {
    let views: &ViewRenderer = modules.get().unwrap();

    views.render("home", &()).into()
}

fn rowtest(modules: &Modules, _: UrlParams) -> Vec<u8> {
    let views: &ViewRenderer = modules.get().unwrap();

    let mut rows = Vec::new();
    for i in 0..1000 {
        rows.push(RowTestEntry {
            name: format!("User #{}", i),
            coolness: (i + 20) * 59 % 100,
            dopeness: (i + 20) * 43 % 100,
        });
    }

    views.render("rowtest", &RowTestModel {rows: rows}).into()
}
