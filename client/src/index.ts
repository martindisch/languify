import { Terminal } from "xterm";
import "xterm/css/xterm.css";

function startTerminal(container) {
  const terminal = new Terminal();
  terminal.open(container);

  terminal.write("Terminal connected");
}

function start() {
  const container = document.getElementById("terminal-container");
  startTerminal(container);
}

start();
