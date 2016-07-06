extern crate hyper;
extern crate num_cpus;

use std::net::{ToSocketAddrs/*, SocketAddr*/};
use std::time::Duration;
use hyper::server::{Server, Request, Response, Listening};

pub struct Clockwork {
}

impl Clockwork {
    pub fn new() -> Self {
        Clockwork {
        }
    }

    pub fn http<A: ToSocketAddrs>(self, addr: A) -> ClockworkGuard {
        fn hello(_req: Request, res: Response) {
            res.send(b"Hello World!").unwrap();
        }

        // Start the HTTP server
        let mut server = Server::http(addr).unwrap();
        server.keep_alive(Some(Duration::from_secs(5)));
        server.set_read_timeout(Some(Duration::from_secs(30)));
        server.set_write_timeout(Some(Duration::from_secs(1)));
        let listening = server.handle_threads(hello, num_cpus::get() * 8).unwrap();

        ClockworkGuard {
            _listening: listening
        }
    }
}

pub struct ClockworkGuard {
    _listening: Listening
}
