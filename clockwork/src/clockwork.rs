use std::net::SocketAddr;
use std::thread::{self, JoinHandle};
use std::sync::Arc;
use crossbeam::sync::MsQueue;
use hyper::net::HttpListener;
use routes::Routes;
use listener;
use worker;

pub struct Clockwork {
    routes: Routes,
    worker_threads: usize,
    listener_threads: usize,
}

impl Clockwork {
    pub fn new(routes: Routes) -> Self {
        let cpus = ::num_cpus::get();
        Clockwork {
            routes: routes,
            worker_threads: cpus,
            listener_threads: cpus,
        }
    }

    pub fn worker_threads(mut self, value: usize) -> Self {
        self.worker_threads = value;
        self
    }

    pub fn listener_threads(mut self, value: usize) -> Self {
        self.listener_threads = value;
        self
    }

    pub fn http(self, addr: &SocketAddr) -> ClockworkJoinHandle {
        let mut handles = Vec::new();
        let routes = Arc::new(self.routes);

        // Create the worker threads
        let queue = Arc::new(MsQueue::new());
        for _ in 0..self.worker_threads {
            let queue = queue.clone();
            let routes = routes.clone();
            let handle = thread::spawn(move || {
                worker::run_worker(queue, routes);
            });

            handles.push(handle);
        }

        // Start the listener threads
        let listener = HttpListener::bind(addr).unwrap();
        for _ in 0..self.listener_threads {
            let listener = listener.try_clone().unwrap();
            let queue = queue.clone();
            let handle = thread::spawn(move || {
                listener::run_listener(listener, queue);
            });

            handles.push(handle);
        }

        // Return a handle for the caller to wait on
        ClockworkJoinHandle {
            handles: handles
        }
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
