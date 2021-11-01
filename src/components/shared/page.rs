use crate::components::routes::route_guard::RouteGuard;
use yew::{prelude::*, Children};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    // whether or not the route is protected
    // if false -> redirect to login page
    #[prop_or_default]
    pub is_protected: bool,
}

pub struct Page {
    props: Props,
}

impl Component for Page {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props: props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let is_authenticated = false;

        if self.props.is_protected {
            html! {
                <RouteGuard is_authenticated = is_authenticated>
                    { for self.props.children.iter() }
                </RouteGuard>
            }
        } else {
            html! {
                { for self.props.children.iter() }
            }
        }
    }
}
