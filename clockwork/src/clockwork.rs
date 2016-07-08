use std::net::SocketAddr;
use std::time::Duration;
use std::thread::{self, JoinHandle};
use std::sync::Arc;
use crossbeam::sync::MsQueue;
use hyper::Next;
use hyper::net::HttpListener;
use hyper::server::Server;
use routes::Routes;
use hyper_handler::HyperHandler;
use worker::WorkerCommand;

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
            let queue = queue.clone();
            let handle = thread::spawn(move || {
                Self::server_thread(listener, queue);
            });

            handles.push(handle);
        }

        // Return a handle for the caller to wait on
        ClockworkJoinHandle {
            handles: handles
        }
    }

    fn worker_thread(queue: Arc<MsQueue<WorkerCommand>>) {
        // TODO: Move to worker module
        loop {
            match queue.pop() {
                WorkerCommand::HandleRequest{ctrl, response} => {
                    // Write a response back to the hyper handler
                    // TODO: Refactor this into a nice wrapper
                    ctrl.ready(Next::write()).unwrap();
                    response.send("HELLO!".into()).unwrap();
                }
            }
        }
    }

    fn server_thread(listener: HttpListener, queue: Arc<MsQueue<WorkerCommand>>) {
        //TODO: Move to hyper_server module along with HyperHandler
        let factory = move |ctrl| {
            let queue = queue.clone();
            HyperHandler::new(ctrl, queue)
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
