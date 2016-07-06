pub struct Routes {
    tmp_handler: Option<Box<RouteHandler>>
}

impl Routes {
    pub fn new() -> Self {
        Routes {
            tmp_handler: None
        }
    }

    pub fn get<S: ToString, H: RouteHandler + 'static>(&mut self, _route: S, handler: H) {
        // TODO: Actually register instead of just keeping the last one
        self.tmp_handler = Some(Box::new(handler));
    }

    pub fn handle(&self) -> String {
        self.tmp_handler.as_ref().unwrap().handle()
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
