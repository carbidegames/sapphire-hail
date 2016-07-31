#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate dotenv;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate webapp;
extern crate clockwork;
extern crate clockwork_handlebars;
extern crate clockwork_server;

mod controllers;
mod models;

use std::env;
use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;
use webapp::status::StatusCode;
use clockwork::{Clockwork, Modules};
use clockwork::routes::{self, Routes};
use clockwork_handlebars::ViewRenderer;
use clockwork_server::Server;
use models::ErrorModel;

fn main() {
    // This allows us to set dev data in .env, while allowing the environment to send us a port
    dotenv::dotenv().unwrap();
    log4rs::init_file("Log4rs.toml", Default::default()).unwrap();

    // Load in the modules
    let mut modules = Modules::new();
    modules.register(ViewRenderer::new("views", "_layout"));

    // Load in the controllers
    let mut routes = Routes::new();
    routes.register("/public/*", routes::file_handler("./public"));
    controllers::register(&mut routes);

    // Start the server
    let addr = get_addr();
    let app = Clockwork::new(modules, routes, error);
    let guard = Server::new(app).http(&addr);
    info!("Listening on {}", addr);
    guard.join();
}

fn get_addr() -> SocketAddr {
    let port = u16::from_str(&env::var("PORT").unwrap()).unwrap();
    SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), port)
}

fn error(modules: &Modules, status: StatusCode) -> Vec<u8> {
    let views: &ViewRenderer = modules.get().unwrap();

    views.render("_error", &ErrorModel {error: status.to_string()}).into()
}
