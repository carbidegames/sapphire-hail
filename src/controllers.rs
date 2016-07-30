use clockwork::Modules;
use clockwork::routes::{self, Routes, UrlParams};
use clockwork_handlebars::ViewRenderer;
use models::{HelloViewModel, NumberModel, NumberViewModel, RowTestModel, RowTestEntry};

pub fn register(routes: &mut Routes) {
    routes.register("/", index);
    routes.register("/about", about);
    routes.register("/number/:num", routes::model_handler(number));
    routes.register("/rowtest", rowtest);
}

fn index(modules: &Modules, _: UrlParams) -> Vec<u8> {
    let views: &ViewRenderer = modules.get().unwrap();

    views.render("home", &()).into()
}

fn about(modules: &Modules, _: UrlParams) -> Vec<u8> {
    let views: &ViewRenderer = modules.get().unwrap();

    let model = HelloViewModel {text: "About".into()};

    views.render("hello", &model).into()
}

fn number(modules: &Modules, model: NumberModel) -> Vec<u8> {
    let views: &ViewRenderer = modules.get().unwrap();

    let view_model = NumberViewModel {
        num: model.num.clone(),
        loneliest: model.num == "1"
    };

    views.render("number", &view_model).into()
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
