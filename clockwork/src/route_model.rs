use webutil::HtmlString;
use routes::{RouteHandler, UrlParams};
use modules::Modules;

pub fn wrap<M: RouteModel, H: ModelRouteHandler<M>>(handler: H) -> ModelHandlerWrapper<M, H> {
    ModelHandlerWrapper {
        handler: handler,
        _model: ::std::marker::PhantomData,
    }
}

pub trait RouteModel: Send + Sync {
    fn from(url: UrlParams) -> Self;
}

pub struct ModelHandlerWrapper<M: RouteModel, H: ModelRouteHandler<M>> {
    handler: H,
    _model: ::std::marker::PhantomData<M>,
}

impl<M: RouteModel, H: ModelRouteHandler<M>> RouteHandler for ModelHandlerWrapper<M, H> {
    fn handle(&self, modules: &Modules, url: UrlParams) -> HtmlString {
        let model = M::from(url);
        self.handler.handle(modules, model)
    }
}

pub trait ModelRouteHandler<M: RouteModel>: Send + Sync {
    fn handle(&self, modules: &Modules, model: M) -> HtmlString;
}

impl<M: RouteModel, F: Fn(&Modules, M) -> HtmlString + Send + Sync> ModelRouteHandler<M> for F {
    fn handle(&self, modules: &Modules, model: M) -> HtmlString {
        self(modules, model)
    }
}
