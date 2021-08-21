import { LanguifyTerminal } from "./terminal";
import { LanguifyApi } from "./api";
import "xterm/css/xterm.css";

const api_url = "http://127.0.0.1:8080";

async function start() {
    const container = document.getElementById("terminal-container");

    const api = new LanguifyApi(api_url);
    const terminal = new LanguifyTerminal(container!, api);

    await terminal.start();
}

start();
