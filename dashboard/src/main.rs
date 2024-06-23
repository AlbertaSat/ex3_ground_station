mod components;
mod pages;
mod routes;
mod types;

use yew::{ html, Html, function_component};
use yew_router::{BrowserRouter, Switch};

use routes::{switch, Route};
use components::nav_bar::NavBar;


#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <NavBar />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}