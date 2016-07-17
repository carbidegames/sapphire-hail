use std::sync::Arc;
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;
use crossbeam::sync::MsQueue;
use hyper::{Control, Decoder, Encoder, Next, RequestUri};
use hyper::net::{HttpStream, HttpListener};
use hyper::server::{Server, Handler, Request, Response};
use hyper::status::StatusCode;
use hyper::header::ContentLength;
use worker::{WorkerCommand, RequestToken};

pub fn run_listener(listener: HttpListener, queue: Arc<MsQueue<WorkerCommand>>) {
    let factory = move |ctrl| {
        let queue = queue.clone();
        HyperHandler::new(ctrl, queue)
    };

    // Set up the server itself
    let server = Server::new(listener)
        .keep_alive(true)
        .idle_timeout(Some(Duration::from_secs(10)))
        .max_sockets(4096);
    let (_listening, server_loop) = server.handle(factory).unwrap();

    // Run the HTTP server loop
    server_loop.run();
}

pub struct HyperHandler {
    // TODO: Consider replacing these Options with a state instead
    uri: Option<RequestUri>,
    ctrl: Option<Control>,
    queue: Arc<MsQueue<WorkerCommand>>,
    receiver: Option<Receiver<String>>,
    data: Option<String>,
}

impl HyperHandler {
    pub fn new(ctrl: Control, queue: Arc<MsQueue<WorkerCommand>>) -> Self {
        HyperHandler {
            uri: None,
            ctrl: Some(ctrl),
            queue: queue,
            receiver: None,
            data: None,
        }
    }
}

impl Handler<HttpStream> for HyperHandler {
    fn on_request(&mut self, req: Request<HttpStream>) -> Next {
        self.uri = Some(req.uri().clone());
        Next::read()
    }

    fn on_request_readable(&mut self, _request: &mut Decoder<HttpStream>) -> Next {
        // TODO: Actually read the data
        // @seanmonstar: So, once read returns WouldBlock, and you don't have all the data, you
        //  can return Next::read() and you'll be notified when it's ready again

        // Queue up a worker task
        // TODO: Refactor task queueing into a nice wrapper
        let (sender, receiver) = mpsc::channel();
        self.receiver = Some(receiver);
        let token = RequestToken::new(
            self.uri.take().unwrap(),
            self.ctrl.take().unwrap(),
            sender
        );
        self.queue.push(WorkerCommand::HandleRequest(token));

        // We need to wait till we get notified by the worker that we're done
        Next::wait()
    }

    fn on_response(&mut self, response: &mut Response) -> Next {
        // We arrived here after being notified, so there should be data in the receiver
        self.data = Some(self.receiver.as_ref().unwrap().recv().unwrap());

        // Build up the response header
        response.set_status(StatusCode::Ok);
        let headers = response.headers_mut();
        headers.set(ContentLength(self.data.as_ref().unwrap().as_bytes().len() as u64));

        Next::write()
    }

    fn on_response_writable(&mut self, response: &mut Encoder<HttpStream>) -> Next {
        let bytes = self.data.as_ref().unwrap().as_bytes();
        response.write(bytes).unwrap();

        Next::end()
    }
}
