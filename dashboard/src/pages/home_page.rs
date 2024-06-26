use yew::{function_component, html, Html};

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div>
            <p>{"home"}</p>
        </div>
    }
}
