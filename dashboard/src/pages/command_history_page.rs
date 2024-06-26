use reqwasm::http::Request;
 use yew::{classes, function_component, html, use_effect_with, use_state, Html};
use yew_custom_components::table::Table;
use yew_custom_components::table::types::ColumnBuilder;

use crate::types::command::Command;


#[function_component(CommandHistoryPage)]
pub fn command_history_page() -> Html {
    let commands = use_state(|| Vec::new());

    {
        let commands = commands.clone();
        use_effect_with((), move |_| {
            let commands = commands.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_commands: Vec<Command> = Request::get("http://127.0.0.1:8000/api/cmd")
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                    .expect("Failed to fetch commands")
                    .json()
                    .await
                    .expect("Failed to parse commands");
                
                commands.set(fetched_commands);
            });

            || ()
        });
    }

    let columns = vec![
        ColumnBuilder::new("payload").orderable(true).short_name("Payload").data_property("payload").header_class("user-select-none").build(),
        ColumnBuilder::new("cmd").orderable(true).short_name("Cmd").data_property("cmd").header_class("user-select-none").build(),
        ColumnBuilder::new("data").orderable(true).short_name("Data").data_property("data").header_class("user-select-none").build(),
        ColumnBuilder::new("timestamp").orderable(true).short_name("Timestamp").data_property("timestamp").header_class("user-select-none").build(),
    ];

    html! {
        <>
            <Table <Command> 
                columns={columns.clone()} 
                data={(*commands).clone()}              
                classes={classes!("table", "table-hover")}
            />
        </>
    }
}