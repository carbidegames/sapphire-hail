use std::sync::mpsc::Sender;
use std::sync::Arc;
use crossbeam::sync::MsQueue;
use hyper::{Control, Next, RequestUri};
use routes::Routes;

pub fn run_worker(queue: Arc<MsQueue<WorkerCommand>>, routes: Arc<Routes>) {
    // TODO: Catch panics gracefully
    loop {
        match queue.pop() {
            WorkerCommand::HandleRequest{uri, ctrl, response} => {
                // TODO: Timeout connections if we receive them 5 seconds after they're queued

                // Get the path from the request
                let route = match uri {
                    RequestUri::AbsolutePath(path) => path,
                    other => panic!("Swallowed request uri {:?}, not implemented!", other)
                };

                // Let the appropriate handler handle it
                let response_data = routes.handle(&route);

                // Write a response back to the hyper handler
                // TODO: Refactor this into a nice wrapper
                response.send(response_data).unwrap();
                ctrl.ready(Next::write()).unwrap();
            }
        }
    }
}

pub enum WorkerCommand {
    // mpsc is also optimized for oneshot messages, so we use it to return data
    // TODO: Refactor this into a nice wrapper
    HandleRequest{uri: RequestUri, ctrl: Control, response: Sender<String>}
}
