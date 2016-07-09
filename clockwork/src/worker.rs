use std::sync::mpsc::Sender;
use std::sync::Arc;
use crossbeam::sync::MsQueue;
use hyper::{Control, Next};

pub fn run_worker(queue: Arc<MsQueue<WorkerCommand>>) {
    // TODO: Move to worker module
    loop {
        match queue.pop() {
            WorkerCommand::HandleRequest{ctrl, response} => {
                // TODO: Timeout connections if we receive them 5 seconds after they're queued

                // Write a response back to the hyper handler
                // TODO: Refactor this into a nice wrapper
                response.send("HELLO!".into()).unwrap();
                ctrl.ready(Next::write()).unwrap();
            }
        }
    }
}

pub enum WorkerCommand {
    // mpsc is also optimized for oneshot messages, so we use it to return data
    // TODO: Refactor this into a nice wrapper
    HandleRequest{ctrl: Control, response: Sender<String>}
}
