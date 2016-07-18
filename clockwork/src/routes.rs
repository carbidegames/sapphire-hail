use route_recognizer::{Router, Params};
use webutil::{HtmlString, UriValue};

pub struct Routes {
    handlers: Router<HandlerEntry>
}

impl Routes {
    pub fn new() -> Self {
        Routes {
            handlers: Router::new()
        }
    }

    pub fn register<H: RouteHandler + 'static>(&mut self, route: &str, handler: H) {
        self.handlers.add(route, HandlerEntry {
            callback: Box::new(handler),
        });
    }

    pub fn handle(&self, route: &str) -> HtmlString {
        let response = if let Ok(matc) = self.handlers.recognize(route) {
            let params = matc.params;
            let entry = matc.handler;

            let url = UrlParams {
                internal: params
            };

            entry.callback.handle(url)
        } else {
            HtmlString::bless("<html><body><h1>404</h1></body></html>")
        };

        response
    }
}

struct HandlerEntry {
    callback: Box<RouteHandler>,
}

pub struct UrlParams {
    internal: Params
}

impl UrlParams {
    pub fn get(&self, key: &str) -> Option<String> {
        let raw = try_opt!(self.internal.find(key));
        let val = UriValue::bless(raw);
        Some(val.unescape())
    }
}

pub trait RouteHandler: Send + Sync {
    fn handle(&self, url: UrlParams) -> HtmlString;
}

impl<F: Fn(UrlParams) -> HtmlString + Send + Sync> RouteHandler for F {
    fn handle(&self, url: UrlParams) -> HtmlString {
        self(url)
    }
}
