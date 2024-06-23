import { Command } from "./command.js";
import { DebugForm } from "./debug.js";

function main() {
  new Command();
  if (document.querySelector(".debug-card")) {
    const debug = new DebugForm();
    debug.showResponse("");
  }
}

main();
