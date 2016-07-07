use route_recognizer::Router;

pub struct Routes {
    handlers: Router<HandlerEntry>
}

struct HandlerEntry {
    callback: Box<RouteHandler>,
}

impl Routes {
    pub fn new() -> Self {
        Routes {
            handlers: Router::new()
        }
    }

    pub fn get<H: RouteHandler + 'static>(&mut self, route: &str, handler: H) {
        self.handlers.add(route, HandlerEntry {
            callback: Box::new(handler),
        });
    }

    pub fn handle(&self, route: &str) -> String {
        let response = if let Ok(matc) = self.handlers.recognize(route) {
            matc.handler.callback.handle()
        } else {
            "404".into()
        };
        
        response
    }
}

pub trait RouteHandler: Send + Sync {
    fn handle(&self) -> String;
}

impl<F: Fn() -> String + Send + Sync> RouteHandler for F {
    fn handle(&self) -> String {
        self()
    }
}
