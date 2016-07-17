use routes::{RouteHandler, UrlParams};

pub fn wrap<M: RouteModel, H: ModelRouteHandler<M>>(handler: H) -> ModelHandlerWrapper<M, H> {
    ModelHandlerWrapper {
        handler: handler,
        _model: ::std::marker::PhantomData,
    }
}

pub trait RouteModel: Send + Sync /* TODO: Find a way to eliminate Send and Sync */ {
    fn from(url: UrlParams) -> Self;
}

pub struct ModelHandlerWrapper<M: RouteModel, H: ModelRouteHandler<M>> {
    handler: H,
    _model: ::std::marker::PhantomData<M>,
}

impl<M: RouteModel, H: ModelRouteHandler<M>> RouteHandler for ModelHandlerWrapper<M, H> {
    fn handle(&self, url: UrlParams) -> String {
        let model = M::from(url);
        self.handler.handle(model)
    }
}

pub trait ModelRouteHandler<M: RouteModel>: Send + Sync {
    fn handle(&self, model: M) -> String;
}

impl<M: RouteModel, F: Fn(M) -> String + Send + Sync> ModelRouteHandler<M> for F {
    fn handle(&self, model: M) -> String {
        self(model)
    }
}
