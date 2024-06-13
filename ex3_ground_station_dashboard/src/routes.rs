use yew::{ html, Html};
use yew_router::Routable;

use crate::pages::{
    home_page::HomePage, 
    send_command_page::SendCommandPage, 
    command_history_page::CommandHistoryPage
};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
        Home,
    #[at("/send_command")]
        SendCommand,
    #[at("/command_history")]
        CommandHistory
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html!{<HomePage />},
        Route::SendCommand => html!{<SendCommandPage />},
        Route::CommandHistory => html!{<CommandHistoryPage />}
    }
}