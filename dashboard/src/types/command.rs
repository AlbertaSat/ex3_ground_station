use serde::{Deserialize, Serialize};
use serde_value::Value;
use yew::{html, Html};
use yew_custom_components::table;
use yew_custom_components::table::types::TableData;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Command {
    pub payload: String,
    pub cmd: String,
    pub data: String,
    pub timestamp: Option<String>,
}

impl TableData for Command {
    fn get_field_as_html(&self, field_name: &str) -> table::error::Result<Html> {
        match field_name {
            "payload" => Ok(html! {&self.payload}),
            "cmd" => Ok(html! {&self.cmd}),
            "data" => Ok(html! {&self.data}),
            "timestamp" => Ok(html! {
                match &self.timestamp {
                    Some(ts) => ts.clone(),
                    None => "No timestamp".to_string(),
                }
            }),
            _ => Ok(html! {})
        }
    }

    fn get_field_as_value(&self, field_name: &str) -> table::error::Result<Value> {
        match field_name {
            "payload" => Ok(Value::String(self.payload.clone())),
            "cmd" => Ok(Value::String(self.cmd.clone())),
            "data" => Ok(Value::String(self.data.clone())),
            "timestamp" => Ok(Value::String(
                self.timestamp.clone().unwrap_or_else(|| "No timestamp".to_string())
            )),
            _ => Ok(serde_value::to_value(()).unwrap()),
        }
    }

    fn matches_search(&self, needle: Option<String>) -> bool {
        match needle {
            Some(needle) => self.cmd.to_lowercase().contains(&needle.to_lowercase()),
            None => true,
        }
    }
}
