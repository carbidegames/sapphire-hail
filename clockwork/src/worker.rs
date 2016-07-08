use std::sync::mpsc::Sender;
use hyper::Control;

pub enum WorkerCommand {
    // mpsc is also optimized for oneshot messages, so we use it to return data
    // TODO: Refactor this into a nice wrapper
    HandleRequest{ctrl: Control, response: Sender<String>}
}
