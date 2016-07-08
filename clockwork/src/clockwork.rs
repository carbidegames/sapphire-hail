use std::net::SocketAddr;
use std::time::Duration;
use std::thread::{self, JoinHandle};
use std::sync::Arc;
use crossbeam::sync::MsQueue;
use hyper::net::HttpListener;
use hyper::server::Server;
use Routes;
use hyper_handler::HyperHandler;

pub struct Clockwork {
    _routes: Routes,
    worker_threads: usize,
    server_threads: usize,
}

impl Clockwork {
    pub fn new(routes: Routes) -> Self {
        let cpus = ::num_cpus::get();
        Clockwork {
            _routes: routes,
            worker_threads: cpus * 8,
            server_threads: cpus * 8,
        }
    }

    pub fn worker_threads(mut self, value: usize) -> Self {
        self.worker_threads = value;
        self
    }

    pub fn server_threads(mut self, value: usize) -> Self {
        self.server_threads = value;
        self
    }

    pub fn http(self, addr: &SocketAddr) -> ClockworkJoinHandle {
        let mut handles = Vec::new();

        // Create the workers
        let queue = Arc::new(MsQueue::new());
        for _ in 0..self.worker_threads {
            let queue = queue.clone();
            let handle = thread::spawn(move || {
                Self::worker_thread(queue);
            });

            handles.push(handle);
        }

        // Start the HTTP servers
        let listener = HttpListener::bind(addr).unwrap();
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

    fn worker_thread(queue: Arc<MsQueue<WorkerCommand>>) {
        loop {
            let _work = queue.pop();
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

enum WorkerCommand {
    Request
}
