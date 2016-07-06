#[macro_use] extern crate log;
extern crate log4rs;
extern crate clockwork;

use clockwork::Clockwork;

fn main() {
    log4rs::init_file("Log4rs.toml", Default::default()).unwrap();

    let _server = Clockwork::new().http("0.0.0.0:8080");
    info!("Listening on 0.0.0.0:8080");
}
