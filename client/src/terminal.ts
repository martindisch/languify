import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import { LanguifyApi } from "./api";

class LanguifyTerminal {
    readonly terminal: Terminal;
    readonly api: LanguifyApi;

    constructor(container: HTMLElement, api_url: string) {
        this.terminal = new Terminal();
        this.api = new LanguifyApi(api_url);

        const fitAddon = new FitAddon();
        this.terminal.loadAddon(fitAddon);
        this.terminal.open(container);
        fitAddon.fit();
    }

    async start(): Promise<void> {
        await this.greeting();
    }

    async greeting(): Promise<void> {
        this.terminal.writeln(texts.greeting);
        this.terminal.writeln(texts.instruction);
        this.terminal.writeln("");
        this.terminal.writeln(texts.languages);
        this.terminal.writeln("");
        this.terminal.writeln(texts.unclear_explanation);
        this.terminal.writeln(texts.unclear_examples);
        this.terminal.writeln("");
        this.terminal.writeln(texts.first_text);

        await this.next_text();
    }

    async next_text(): Promise<void> {
        const unclassifiedText = await this.api.get_unclassified_text();
        this.terminal.writeln(unclassifiedText.text);
    }
}

const texts = {
    greeting: `\
Thank you for participating. You will be shown a series of texts.`,
    instruction: `\
Classify each one by pressing the key for the appropriate language.`,
    languages: `\
Deutsch: d, Français: f, Italiano: i, English: e, Unclear: u`,
    unclear_explanation: `\
Be sure to mark texts that are impossible to assign to any single language as \
unclear (u).`,
    unclear_examples: `Examples are "Super", "👍 ✨ " or "Ok".`,
    first_text: `Here comes your first text:`
}

export { LanguifyTerminal };
