use chrono::Utc;
use reqwasm::http::Request;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement};
use yew::{function_component, html, use_state, Callback, Html, InputEvent, SubmitEvent, TargetCast};

use crate::types::command::Command;

async fn send_command(command: Command) -> Result<(), Box<dyn std::error::Error>> {
    let params = json!({
        "payload": command.payload,
        "cmd": command.cmd,
        "data": command.data,
        "timestamp": command.timestamp, 
    });
    
    let response = Request::post("http://127.0.0.1:8000/api/cmd")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&params)?)
        .send()
        .await?;
        
    if !response.ok() {
        let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        console::error_1(&error_message.clone().into());
        return Err(Box::from(error_message));
    }

    Ok(())
}

#[function_component(CommandForm)]
pub fn command_form() -> Html {
    let command = use_state(|| Command::default());

    let on_change = {
        let command = command.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let name = input.name();
            let value = input.value();
            
            command.set(Command {
                payload: if name == "payload" { value.clone() } else { command.payload.clone() },
                cmd: if name == "cmd" { value.clone() } else { command.cmd.clone() },
                data: if name == "data" { value.clone() } else { command.data.clone() },
                timestamp: command.timestamp.clone(),
            });
        })
    };


    let on_submit = {
        let command = command.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let mut command_ = (*command).clone();
            command_.timestamp = Some(Utc::now().to_rfc3339());            
            spawn_local(async move {
                if let Err(e) = send_command(command_).await {
                    console::error_1(&e.to_string().into());
                }
            });
            command.set(Command::default());
        })
    };

    html! {  
        <form onsubmit={on_submit} class="space-y-6 bg-white p-6 rounded-lg shadow-md">
            <label class="block">
                <span class="text-gray-700">{"Payload:"}</span>
                <input 
                    name="payload" 
                    type="text" 
                    autocomplete="off"
                    value={command.payload.clone()} 
                    oninput={on_change.clone()} 
                    class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                />
            </label>
            <label class="block">
                <span class="text-gray-700">{"Command:"}</span>
                <input 
                    name="cmd" 
                    type="text" 
                    autocomplete="off"
                    value={command.cmd.clone()} 
                    oninput={on_change.clone()} 
                    class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                />
            </label>
            <label class="block">
                <span class="text-gray-700">{"Data:"}</span>
                <input 
                    name="data" 
                    type="text" 
                    autocomplete="off"
                    value={command.data.clone()} 
                    oninput={on_change.clone()} 
                    class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                />
            </label>
            <button 
                type="submit" 
                data-action="send" 
                class="bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500"
            >
                {"✨ Send Command"}
            </button>
        </form>
    }
}
