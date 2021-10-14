use yew_router::prelude::Switch;

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/"]
    Home,
}
