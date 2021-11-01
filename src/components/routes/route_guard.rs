use super::redirect::Redirect;
use yew::{prelude::*, Children};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub is_authenticated: bool,
    #[prop_or_default]
    pub children: Children,
}

pub struct RouteGuard {
    props: Props,
}

impl Component for RouteGuard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props: props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let is_authenticated = self.props.is_authenticated;

        if is_authenticated {
            return html! {
                { for self.props.children.iter() }
            };
        } else {
            return html! {
                <Redirect to="/login" />
            };
        }
    }
}
