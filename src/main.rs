extern crate dotenv;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate rustc_serialize;
extern crate webutil;
extern crate clockwork;
extern crate clockwork_handlebars;

mod controllers;
mod models;

use std::env;
use std::net::{SocketAddr, IpAddr};
use std::str::FromStr;
use clockwork::{Clockwork, Modules};
use clockwork::routes::Routes;
use clockwork_handlebars::ViewRenderer;

fn main() {
    // This allows us to set dev data in .env, while allowing the environment to send us a port
    dotenv::dotenv().unwrap();
    log4rs::init_file("Log4rs.toml", Default::default()).unwrap();

    // Load in the modules
    let mut modules = Modules::new();
    modules.register(ViewRenderer::new("views", "_layout"));

    // Load in the controllers
    let mut routes = Routes::new();
    controllers::register(&mut routes);

    // Start the server
    let addr = get_addr();
    let guard = Clockwork::new(modules, routes).http(&addr);
    info!("Listening on {}", addr);
    guard.join();
}

fn get_addr() -> SocketAddr {
    let port = u16::from_str(&env::var("PORT").unwrap()).unwrap();
    SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), port)
}
