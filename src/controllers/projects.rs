use clockwork::Modules;
use clockwork::routes::{self, Routes, UriParams, BodyParams, RouteModel};
use clockwork_handlebars::ViewRenderer;
use models::NewSubmit;

pub fn register(routes: &mut Routes) {
    routes.get("/projects/new", new);
    routes.post("/projects/new", routes::model_handler(new_submit));
}

fn new(modules: &Modules, _: UriParams, _: BodyParams) -> Vec<u8> {
    let views: &ViewRenderer = modules.get().unwrap();

    views.render("projects/new", &()).into()
}

fn new_submit(modules: &Modules, params: NewModel) -> Vec<u8> {
    let views: &ViewRenderer = modules.get().unwrap();

    views.render("projects/new_submit", &NewSubmit {title: params.title}).into()
}

pub struct NewModel {
    pub title: String,
}

impl RouteModel for NewModel {
    fn from(_url: UriParams, body: BodyParams) -> Self {
        let body = body.as_form();

        NewModel {
            title: body.get("title").unwrap()
        }
    }
}
