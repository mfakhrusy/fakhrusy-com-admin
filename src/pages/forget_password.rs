use crate::components::forget_password::forget_password_form::ForgetPasswordForm;
use crate::components::shared::page::Page;
use yew::prelude::*;

pub struct ForgetPasswordPage;

impl Component for ForgetPasswordPage {
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
            <Page>
                <main class="min-w-full min-h-screen flex justify-center items-center bg-purple-500">
                    <ForgetPasswordForm />
                </main>
            </Page>
        }
    }
}
