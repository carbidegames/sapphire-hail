use std::net::{ToSocketAddrs/*, SocketAddr*/};
use std::time::Duration;
use hyper::server::{Server, Request, Response, Listening, Handler};
use ::Routes;

pub struct Clockwork {
    routes: Routes
}

impl Clockwork {
    pub fn new(routes: Routes) -> Self {
        Clockwork {
            routes: routes
        }
    }

    pub fn http<A: ToSocketAddrs>(self, addr: A) -> ClockworkHandle {
        // Set up the handler
        let handler = ClockworkHandler {
            routes: self.routes
        };

        // Start the HTTP server
        let mut server = Server::http(addr).unwrap();
        server.keep_alive(Some(Duration::from_secs(5)));
        server.set_read_timeout(Some(Duration::from_secs(30)));
        server.set_write_timeout(Some(Duration::from_secs(1)));
        let listening = server.handle_threads(handler, ::num_cpus::get() * 8).unwrap();

        // Return a handle for the caller to wait on
        ClockworkHandle {
            listening: listening
        }
    }
}

/// A handle referring to a Clockwork listening server. Can be used to close the connection and
/// stop Clockwork from running.
pub struct ClockworkHandle {
    listening: Listening
}

impl ClockworkHandle {
    /// Warning: This function doesn't work. The server remains listening after you called it.
    /// See https://github.com/hyperium/hyper/issues/338 for more details.
    ///
    /// Stop the server from listening to its socket address, allowing the guard to be dropped
    /// without blocking.
    pub fn close(mut self) {
        self.listening.close().unwrap();
    }
}

struct ClockworkHandler {
    routes: Routes
}

impl Handler for ClockworkHandler {
    fn handle(&self, _req: Request, res: Response) {
        //let route = req.uri.path.join("/");
        let response = self.routes.handle("/");
        res.send(response.as_bytes()).unwrap();
    }
}
