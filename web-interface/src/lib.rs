mod utils;

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
