use super::app_route::AppRoute;
use crate::components::routes::route_guard::RouteGuard;
use crate::pages::forget_password::ForgetPasswordPage;
use crate::pages::home::HomePage;
use crate::pages::login::LoginPage;
use crate::pages::register::RegisterPage;
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
        let render_fn = Router::render(|switch: AppRoute| match switch {
            AppRoute::Home => html! {
                <HomePage />
            },
            AppRoute::Login => html! {
                <LoginPage />
            },
            AppRoute::Register => html! {
                <RegisterPage />
            },
            AppRoute::ForgetPassword => html! {
                <ForgetPasswordPage />
            },
        });

        html! {
            <Router<AppRoute>
                render = render_fn
            />
        }
    }
}
