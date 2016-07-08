use std::io::Write;
use hyper::{Control, Decoder, Encoder, Next};
use hyper::net::HttpStream;
use hyper::server::{Handler, Request, Response};

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
    _ctrl: Control
}

impl HyperHandler {
    pub fn new(ctrl: Control) -> Self {
        HyperHandler {
            _ctrl: ctrl
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
        Next::write()
    }

    fn on_response(&mut self, response: &mut Response) -> Next {
        use hyper::header::ContentLength;
        response.headers_mut().set(ContentLength(b"HELLO".len() as u64));
        Next::write()
    }

    fn on_response_writable(&mut self, response: &mut Encoder<HttpStream>) -> Next {
        response.write(b"HELLO").unwrap();
        Next::end()
    }
}
