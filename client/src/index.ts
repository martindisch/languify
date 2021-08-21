import { LanguifyTerminal } from "./terminal";
import "xterm/css/xterm.css";

const api_url = "http://127.0.0.1:8080";

async function start() {
  const container = document.getElementById("terminal-container");
  const terminal = new LanguifyTerminal(container!, api_url);

  await terminal.start();
}

start();
