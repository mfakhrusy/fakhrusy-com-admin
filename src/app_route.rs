use yew_router::components::RouterAnchor;
use yew_router::prelude::Switch;

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
