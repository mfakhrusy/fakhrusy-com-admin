use yew::prelude::*;
use crate::components::shared::input_field::InputField;

pub struct LoginForm {}

impl Component for LoginForm {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
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
        html! {
            <div class="flex flex-col">
                <h1 class="font-semibold mb-2">
                    { "Login" }
                </h1>
                <form>
                    <InputField label="Username" />
                    <div class="h-2" />
                    <InputField label="Password" />
                </form>
            </div>
        }
    }
}

