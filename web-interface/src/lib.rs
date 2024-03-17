mod utils;

use std::net::TcpStream;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, AlbertaSat!");
}

#[wasm_bindgen]
pub fn string_parse(s: &str) {
    match s {
        "testing" => alert("This was the testing word"),
        "adcs off" => alert("Turning ADCS off"),
        _ => (),
    }
}

#[wasm_bindgen]
pub fn init_connection() {
    alert("Clicked button");
    match TcpStream::connect("localhost:9688") {
        Ok(mut stream) => {
            alert("Connected to localhost:9688");
        }
        Err(e) => alert(&format!("Failed to connect. Error: {}", e)),
    }
}
