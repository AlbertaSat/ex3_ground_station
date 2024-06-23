use yew::{function_component, html, Html};
use crate::components::command_form::CommandForm;

#[function_component(SendCommandPage)]
pub fn send_command_page() -> Html {
    html! {
        <div class="min-h-screen flex items-center justify-center bg-gray-100">
            <div class="max-w-lg w-full">
                <CommandForm />
            </div>
        </div>
    }
}
