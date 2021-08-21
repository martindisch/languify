import { LanguifyTerminal } from "./terminal";
import "xterm/css/xterm.css";

function start() {
  const container = document.getElementById("terminal-container");
  const terminal = new LanguifyTerminal(container!);
}

start();
