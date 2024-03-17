import * as wasm from "web-interface";

const displayTerm = document.getElementsByClassName("terminal");
const history = document.getElementById("history");
const inputTerm = document.getElementById("input");
const connectBtn = document.getElementById("connect_btn");

for (var i = 0; i < displayTerm.length; i++) {
    displayTerm[i].addEventListener("click", _ => {
        inputTerm.focus();
    });
}

inputTerm.onkeyup = e => {
    if (e.which == 13) {
        history.innerHTML = "";
        history.append(inputTerm.value);
        history.append("\n");
        wasm.string_parse(inputTerm.value);
        inputTerm.value = '';
    }};

// Button for connecting
connectBtn.addEventListener("click", _ => {
    wasm.init_connection();
});
