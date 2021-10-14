use super::app_route::AppRoute;
use crate::pages::home::Home;
use crate::pages::login::Login;
use yew::prelude::*;
use yew_router::prelude::Router;

pub struct Routes;

impl Component for Routes {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Home => html! {
                           <Home />
                        },
                        AppRoute::Login => html! {
                           <Login />
                        }
                    }
                })
            />
        }
    }
}
