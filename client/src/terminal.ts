import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";

class LanguifyTerminal {
    readonly terminal: Terminal;
    readonly api_url: string;

    constructor(container: HTMLElement, api_url: string) {
        this.terminal = new Terminal();
        this.api_url = api_url;

        const fitAddon = new FitAddon();
        this.terminal.loadAddon(fitAddon);
        this.terminal.open(container);
        fitAddon.fit();

        this.greeting();
    }

    greeting(): void {
        this.terminal.writeln(texts.greeting);
        this.terminal.writeln(texts.instruction);
        this.terminal.writeln("");
        this.terminal.writeln(texts.languages);
        this.terminal.writeln("");
        this.terminal.writeln(texts.unclear_explanation);
        this.terminal.writeln(texts.unclear_examples);
        this.terminal.writeln("");
        this.terminal.writeln(texts.first_text);
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
