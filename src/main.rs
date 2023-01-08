use monkey_lang::repl;

const HISTORY_FILE: &str = "./.history";

fn main() -> rustyline::Result<()> {
    repl::start(HISTORY_FILE)
}
