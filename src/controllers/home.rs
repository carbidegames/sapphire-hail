use clockwork::Routes;

pub fn register(routes: &mut Routes) {
    routes.get("/", home_index);
    routes.get("/about", home_about);
}

fn home_index() -> String {
    "Home/Index".into()
}

fn home_about() -> String {
    "Home/About".into()
}
