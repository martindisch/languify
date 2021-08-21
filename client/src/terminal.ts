import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import { LanguifyApi } from "./api";

class LanguifyTerminal {
    readonly terminal: Terminal;
    readonly api: LanguifyApi;
    currentTextId: number;

    constructor(container: HTMLElement, api: LanguifyApi) {
        this.terminal = new Terminal();
        this.api = api;
        this.currentTextId = -1;

        const fitAddon = new FitAddon();
        this.terminal.loadAddon(fitAddon);
        this.terminal.open(container);
        fitAddon.fit();
    }

    start = async () => {
        this.terminal.onKey(this.onKey);
        greetingLines.forEach((l) => this.terminal.writeln(l));

        await this.nextText();
    };

    nextText = async () => {
        const unclassifiedText = await this.api.getUnclassifiedText();
        this.currentTextId = unclassifiedText.id;
        this.terminal.writeln(unclassifiedText.text);
        this.prompt();
    };

    prompt = () => {
        this.terminal.write("> ");
    };

    onKey = async ({ key }: { key: string }) => {
        const language = keyToLanguage[key];

        if (language !== undefined) {
            this.terminal.writeln(language);
            this.terminal.writeln("");

            await this.api.addClassifiedText(this.currentTextId, language);

            await this.nextText();
        }
    };
}

const keyToLanguage: { [index: string]: string } = {
    d: "de",
    f: "fr",
    i: "it",
    e: "en",
    u: "??",
};

const greetingLines = [
    `\
Thank you for participating. You will be shown a series of texts.`,
    `\
Classify each one by pressing the key for the appropriate language.`,
    "",
    `\
Deutsch: d, Français: f, Italiano: i, English: e, Unclear: u`,
    "",
    `\
Be sure to mark texts that are impossible to assign to any single language as \
unclear (u).`,
    `Examples are "Super", "👍 ✨ " or "Ok".`,
    "",
    `Here comes your first text:`,
];

export { LanguifyTerminal };
