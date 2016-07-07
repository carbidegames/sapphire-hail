use clockwork::Routes;

pub fn register(routes: &mut Routes) {
    routes.get("/", home_index);
    routes.get("/about", home_about);
}

fn home_index() -> String {
    "<html><body><h1>Index</h1></body></html>".into()
}

fn home_about() -> String {
    "About".into()
}
