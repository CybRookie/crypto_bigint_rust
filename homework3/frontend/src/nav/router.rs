use yew_router::prelude::*;
use yew_router::switch::Permissive;

// Define a Yew router, which will control switching of the pages in an SPA based on the URL path.
#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/{.}"]
    PageNotFound(Permissive<String>),
    #[to = "/"]
    Home,
}

// Define custom types for convenience.
pub type AppRouter = Router<AppRoute>;
pub type AppAnchor = RouterAnchor<AppRoute>;
