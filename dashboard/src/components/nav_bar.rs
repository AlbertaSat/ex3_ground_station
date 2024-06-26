use yew::{function_component, html, Html};
use yew_router::components::Link;

use crate::routes::Route;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <nav class="bg-blue-900 p-4 flex justify-between items-center">
            <div class="text-white text-xl font-bold">
                { "ALBERTASAT" }
            </div>            
            <ul class="flex space-x-4">
                <li><Link<Route> to={Route::Home} classes="text-white hover:text-gray-300">{ "Home" }</Link<Route>></li>
                <li><Link<Route> to={Route::SendCommand} classes="text-white hover:text-gray-300">{ "Send Command" }</Link<Route>></li>
                <li><Link<Route> to={Route::CommandHistory} classes="text-white hover:text-gray-300">{ "Command History" }</Link<Route>></li>
            </ul>
        </nav>
    }
}
