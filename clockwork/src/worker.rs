use std::sync::mpsc::Sender;
use std::sync::Arc;
use crossbeam::sync::MsQueue;
use hyper::{Control, Next, RequestUri};
use routes::Routes;

pub fn run_worker(queue: Arc<MsQueue<WorkerCommand>>, routes: Arc<Routes>) {
    // TODO: Catch panics gracefully
    loop {
        match queue.pop() {
            WorkerCommand::HandleRequest(request) => handle_request(request, &routes)
        }
    }
}

fn handle_request(request: RequestToken, routes: &Routes) {
    // TODO: Timeout connections if we receive them X amount of time after they're queued

    let response_data = {
        // Get the path from the request
        let route = match request.uri() {
            &RequestUri::AbsolutePath(ref path) => path,
            other => panic!("Swallowed request uri {:?}, not implemented!", other)
        };

        // Let the appropriate handler handle it
        routes.handle(&route)
    };

    // Write a response back to the hyper handler
    request.complete(response_data);
}

pub enum WorkerCommand {
    // mpsc is also optimized for oneshot messages, so we use it to return data
    // TODO: Refactor this into a nice wrapper
    HandleRequest(RequestToken)
    //HandleRequest{uri: RequestUri, ctrl: Control, response: Sender<String>}
}

pub struct RequestToken {
    uri: RequestUri,
    ctrl: Control,
    sender: Sender<String>
}

impl RequestToken {
    pub fn new(uri: RequestUri, ctrl: Control, sender: Sender<String>) -> Self {
        RequestToken {
            uri: uri,
            ctrl: ctrl,
            sender: sender,
        }
    }

    fn uri(&self) -> &RequestUri {
        &self.uri
    }

    fn complete(self, response_data: String) {
        self.sender.send(response_data).unwrap();
        self.ctrl.ready(Next::write()).unwrap();
    }
}
