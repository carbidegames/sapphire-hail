use std::net::SocketAddr;
use std::time::Duration;
use std::thread::{self, JoinHandle};
use hyper::net::HttpListener;
use hyper::server::Server;
use ::Routes;
use hyper_handler::HyperHandler;

pub struct Clockwork {
    _routes: Routes,
    server_threads: usize,
}

impl Clockwork {
    pub fn new(routes: Routes) -> Self {
        Clockwork {
            _routes: routes,
            server_threads: ::num_cpus::get() * 8,
        }
    }

    pub fn server_threads(mut self, value: usize) -> Self {
        self.server_threads = value;
        self
    }

    pub fn http(self, addr: &SocketAddr) -> ClockworkJoinHandle {
        // Start the HTTP servers
        let listener = HttpListener::bind(addr).unwrap();
        let mut handles = Vec::new();
        for _ in 0..self.server_threads {
            let listener = listener.try_clone().unwrap();
            let handle = thread::spawn(move || {
                Self::server_thread(listener);
            });

            handles.push(handle);
        }

        // Return a handle for the caller to wait on
        ClockworkJoinHandle {
            handles: handles
        }
    }

    fn server_thread(listener: HttpListener) {
        let factory = |ctrl| {
            HyperHandler::new(ctrl)
        };

        // Set up the server itself
        let server = Server::new(listener)
            .keep_alive(true)
            .idle_timeout(Duration::from_secs(10))
            .max_sockets(4096);
        let (_listening, server_loop) = server.handle(factory).unwrap();

        // Run the HTTP server loop
        server_loop.run();
    }
}

/// A handle referring to a Clockwork listening server.
pub struct ClockworkJoinHandle {
    handles: Vec<JoinHandle<()>>
}

impl ClockworkJoinHandle {
    pub fn join(self) {
        for handle in self.handles {
            handle.join().unwrap();
        }
    }
}
