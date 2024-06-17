use yew::{classes, function_component, html, Html};
use yew_custom_components::table::Table;
use yew_custom_components::table::types::ColumnBuilder;

use crate::types::command::Command;


#[function_component(CommandHistoryPage)]
pub fn command_history_page() -> Html {
    // mock data
    let commands = vec![
        Command { payload: "Payload 1".into(), cmd: "Cmd 1".into(), data: "Data 1".into(), timestamp:  Some("Time 1".into()) },
        Command { payload: "Payload 3".into(), cmd: "Cmd 3".into(), data: "Data 3".into(), timestamp:  Some("Time 1".into()) },
        Command { payload: "Payload 2".into(), cmd: "Cmd 2".into(), data: "Data 2".into(), timestamp:  Some("Time 1".into()) },
    ];

    let columns = vec![
        ColumnBuilder::new("payload").orderable(true).short_name("Payload").data_property("payload").header_class("user-select-none").build(),
        ColumnBuilder::new("cmd").orderable(true).short_name("Cmd").data_property("cmd").header_class("user-select-none").build(),
        ColumnBuilder::new("data").orderable(true).short_name("Data").data_property("data").header_class("user-select-none").build(),
    ];

    html! {
        <>
            <Table <Command> 
                columns={columns.clone()} 
                data={commands.clone()}              
                classes={classes!("table", "table-hover")}
            />
        </>
    }
}