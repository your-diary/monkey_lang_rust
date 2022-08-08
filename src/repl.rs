use rustyline;

use super::environment::Environment;
use super::evaluator;
use super::lexer::Lexer;
use super::parser::Parser;
use super::token::Token;

const HISTORY_FILE: &str = "./.history";

fn get_tokens(s: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(s);
    let mut v = Vec::new();
    loop {
        let token = lexer.get_next_token();
        if (token == Token::Eof) {
            break;
        }
        v.push(token);
    }
    v.push(Token::Eof);
    v
}

pub fn start() -> rustyline::Result<()> {
    let mut env = Environment::new(None);
    let mut rl = rustyline::Editor::<()>::with_config(
        rustyline::Config::builder()
            .edit_mode(rustyline::EditMode::Vi)
            .auto_add_history(true)
            .build(),
    )?;
    rl.load_history(HISTORY_FILE)?;
    loop {
        match rl.readline("\n>> ") {
            Err(_) => break,
            Ok(line) => {
                if (line.trim().is_empty()) {
                    continue;
                }
                let mut parser = Parser::new(get_tokens(&line));
                match parser.parse() {
                    None => {
                        println!("parse error");
                    }
                    Some(e) => {
                        // println!("{:#?}", e);
                        match evaluator::eval(&e, &mut env) {
                            Ok(e) => println!("{}", e),
                            Err(e) => println!("\u{001B}[091m{}\u{001B}[0m", e),
                        }
                    }
                }
            }
        }
    }
    rl.save_history(HISTORY_FILE)
}
