import * as wasm from "web-interface";

const displayTerm = document.getElementsByClassName("terminal");
const history = document.getElementById("history")
const inputTerm = document.getElementById("input");

for (var i = 0; i < displayTerm.length; i++) {
    displayTerm[i].addEventListener("click", _ => {
        inputTerm.focus()
    });
}

inputTerm.onkeyup = e => {
    if (e.which == 13) {
        history.append(inputTerm.value + '\n');
        inputTerm.value = '';
        wasm.greet();
    }};
