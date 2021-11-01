// copied from: https://github.com/ericandre615/yewi/blob/feature/ex-ss/src/components/routes/redirect.rs
// with a few changes (delete unused imports)
// relevant discussion: https://github.com/yewstack/yew/issues/1526#issuecomment-794140031
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use yew_router::{
    agent::{RouteAgentDispatcher, RouteRequest},
    route::Route,
};

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct RedirectProps {
    pub to: String,
}

#[derive(Debug)]
pub struct Redirect {
    props: RedirectProps,
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<()>,
}

pub enum RedirectMsg {
    Redirecting,
}

impl Component for Redirect {
    type Message = RedirectMsg;
    type Properties = RedirectProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgentDispatcher::new();

        Self {
            props,
            router,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            RedirectMsg::Redirecting => {
                let route = Route::from(self.props.to.clone());
                self.router.send(RouteRequest::ChangeRoute(route));
            }
        }

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        self.link.send_message(RedirectMsg::Redirecting);
        html! { <></> }
    }
}
