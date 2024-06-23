import { sendForm } from "./request.js";

export class Command {
  constructor() {
    this.allCmdCards = document.querySelectorAll(".cmd-card");
    this.activateSendForm();
  }

  activateSendForm() {
    const sendForm = document.querySelector(".send-cmd-card form");
    new SendCommandForm(sendForm);
  }
}

class SendCommandForm {
  constructor(el) {
    this.form = el;
    this.sendButton = el.querySelector("button[data-action='send']");
    this.sendButton.addEventListener(
      "click",
      this.handleSendClick.bind(this)
    );
  }

  handleSendClick(event) {
    event.preventDefault();
    sendForm(this.form, "POST", "/api/cmd", this.sendCmd);
    this.form.reset();
  }

  sendCmd(rawData) {
    const data = JSON.parse(rawData);

    const commandCard = document.querySelector(".command-card").cloneNode(true);
    const commadContent = commandCard.querySelector(".command-content");

    const cmdPayload = commandContent.querySelector("[data-cmd-payload]");
    cmdPayload.textContent = data.payload;
    cmdPayload.setAttribute("data-cmd-payload", data.payload);

    const cmdOp = commandContent.querySelector("[data-cmd-op]");
    cmdOp.textContent = data.cmd;
    cmdOp.setAttribute("data-cmd-op", data.cmd);

    const cmdArg = commandContent.querySelector("[data-cmd-arg]");
    cmdArg.textContent = data.data;
    cmdArg.setAttribute("data-cmd-arg", data.data);
  }
}
