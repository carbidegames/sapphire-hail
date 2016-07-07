use std::net::SocketAddr;
use std::time::Duration;
use std::io::Write;
use std::thread::{self, JoinHandle};
use hyper::{Decoder, Encoder, Next};
use hyper::net::{HttpStream, HttpListener};
use hyper::server::{Server, Handler, Request, Response};
//use hyper::uri::RequestUri;
use ::Routes;

pub struct Clockwork {
    _routes: Routes
}

impl Clockwork {
    pub fn new(routes: Routes) -> Self {
        Clockwork {
            _routes: routes
        }
    }

    pub fn http(self, addr: &SocketAddr) -> ClockworkHandle {
        let listener = HttpListener::bind(addr).unwrap();

        // Start the HTTP servers across the cores
        let cpus = ::num_cpus::get() * 8;
        let mut handles = Vec::new();
        for _ in 0..cpus {
            let listener = listener.try_clone().unwrap();
            let handle = thread::spawn(move || {
                let factory = |_| {
                    ClockworkHandler
                };

                // Set up the server itself
                let server = Server::new(listener)
                    .keep_alive(true)
                    .idle_timeout(Duration::from_secs(10))
                    .max_sockets(4096);
                let (_listening, server_loop) = server.handle(factory).unwrap();

                // Run the HTTP server loop
                server_loop.run();
            });

            handles.push(handle);
        }

        // Return a handle for the caller to wait on
        ClockworkHandle {
            handles: handles
        }
    }
}

/// A handle referring to a Clockwork listening server.
pub struct ClockworkHandle {
    handles: Vec<JoinHandle<()>>
}

impl ClockworkHandle {
    pub fn join(self) {
        for handle in self.handles {
            handle.join().unwrap();
        }
    }
}

/*struct ClockworkHandler {
    routes: Routes
}

impl Handler for ClockworkHandler {
    fn handle(&self, req: Request, res: Response) {
        // TODO: Catch panics, responding an internal error

        // Get a route to pass into the router
        let route = match req.uri {
            RequestUri::AbsolutePath(path) => path,
            other => panic!("Swallowed request uri {:?}, not implemented!", other)
        };

        // Let the router handle the route
        let response = self.routes.handle(&route);
        res.send(response.as_bytes()).unwrap();
    }
}*/

struct ClockworkHandler;

impl Handler<HttpStream> for ClockworkHandler {
    fn on_request(&mut self, _: Request<HttpStream>) -> Next {
        Next::write()
    }

    fn on_request_readable(&mut self, _: &mut Decoder<HttpStream>) -> Next {
        Next::write()
    }

    fn on_response(&mut self, response: &mut Response) -> Next {
        use hyper::header::ContentLength;
        response.headers_mut().set(ContentLength(b"HELLO".len() as u64));
        Next::write()
    }

    fn on_response_writable(&mut self, encoder: &mut Encoder<HttpStream>) -> Next {
        encoder.write(b"HELLO").unwrap();
        Next::end()
    }
}
