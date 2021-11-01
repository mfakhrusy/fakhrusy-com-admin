use crate::app_route::{AppAnchor, AppRoute};
use crate::components::shared::{form_container::FormContainer, input_field::InputField};
use yew::prelude::*;

pub struct RegisterForm {}

impl Component for RegisterForm {
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
            <FormContainer title="Register">
                <form>
                    <InputField label="E-mail" />
                    <div class="h-2" />
                    <InputField label="Password" />
                    <div class="h-1" />
                    <div class="flex justify-between text-gray-800 text-xs">
                        <AppAnchor
                            route=AppRoute::ForgetPassword
                        >
                            {"Forget Password"}
                        </AppAnchor>
                        <AppAnchor
                            route=AppRoute::Login
                        >
                            {"Login"}
                        </AppAnchor>
                    </div>
                    <div class="h-5" />
                    <button class="bg-purple-500 hover:bg-opacity-80 transition px-5 py-2 rounded border-2 border-purple-600 w-full">
                        {"Login"}
                    </button>
                    <div class="h-9" />
                </form>
            </FormContainer>
        }
    }
}
