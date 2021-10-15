use yew::{prelude::*, Children};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}


pub struct FormContainer {
    props: Props,
}

impl Component for FormContainer {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props: props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {

        html! {
            <div class="flex flex-col border-2 border-purple-600 py-12 px-8 rounded-md bg-purple-400">
                <h1 class="font-semibold mb-2 text-xl">
                    { &self.props.title }
                </h1>
                { for self.props.children.iter() }
            </div>
        }
    }
}

