use std::io::Write;
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver};
use crossbeam::sync::MsQueue;
use hyper::{Control, Decoder, Encoder, Next};
use hyper::net::HttpStream;
use hyper::server::{Handler, Request, Response};
use hyper::header::ContentLength;
use worker::WorkerCommand;

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

pub struct HyperHandler {
    ctrl: Option<Control>,
    queue: Arc<MsQueue<WorkerCommand>>,
    receiver: Option<Receiver<String>>,
    data: Option<String>,
}

impl HyperHandler {
    pub fn new(ctrl: Control, queue: Arc<MsQueue<WorkerCommand>>) -> Self {
        HyperHandler {
            ctrl: Some(ctrl),
            queue: queue,
            receiver: None,
            data: None,
        }
    }
}

impl Handler<HttpStream> for HyperHandler {
    fn on_request(&mut self, _: Request<HttpStream>) -> Next {
        // TODO: Actually read the header
        Next::read()
    }

    fn on_request_readable(&mut self, _request: &mut Decoder<HttpStream>) -> Next {
        // TODO: Actually read the data
        // @seanmonstar: So, once read returns WouldBlock, and you don't have all the data, you
        //  can return Next::read() and you'll be notified when it's ready again

        // Queue up a worker task
        // TODO: Refactor this into a nice wrapper
        let (sender, receiver) = mpsc::channel();
        self.receiver = Some(receiver);
        self.queue.push(WorkerCommand::HandleRequest{
            ctrl: self.ctrl.take().unwrap(),
            response: sender,
        });

        // We need to wait till we get notified by the worker that we're done
        Next::wait()
    }

    fn on_response(&mut self, response: &mut Response) -> Next {
        // We arrived here after being notified, so there should be data in the receiver
        self.data = Some(self.receiver.as_ref().unwrap().recv().unwrap());

        let headers = response.headers_mut();
        headers.set(ContentLength(self.data.as_ref().unwrap().len() as u64));

        Next::write()
    }

    fn on_response_writable(&mut self, response: &mut Encoder<HttpStream>) -> Next {
        response.write(self.data.as_ref().unwrap().as_bytes()).unwrap();

        Next::end()
    }
}
