use yew_router::prelude::Switch;
use yew_router::components::RouterAnchor;

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/forget-password"]
    ForgetPassword,
    #[to = "/register"]
    Register,
    #[to = "/"]
    Home,
}

pub type AppAnchor = RouterAnchor<AppRoute>;
