import { LanguifyTerminal } from "./terminal";
import { LanguifyApi } from "./api";
import "xterm/css/xterm.css";

const api_url = "https://languify.me";

async function start() {
    const container = document.getElementById("terminal-container");

    const api = new LanguifyApi(api_url);
    const terminal = new LanguifyTerminal(container!, api);

    await terminal.start();
}

start();
