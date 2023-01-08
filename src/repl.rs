use rustyline;

use super::environment::Environment;
use super::evaluator::Evaluator;
use super::lexer::{Lexer, LexerResult};
use super::parser::Parser;
use super::token::Token;

const COLOR_END: &'static str = "\u{001B}[0m";
const COLOR_RED: &'static str = "\u{001B}[091m";
const COLOR_PURPLE: &'static str = "\u{001B}[095m";

fn get_tokens(s: &str) -> LexerResult<Vec<Token>> {
    let mut lexer = Lexer::new(s);
    let mut v = vec![];
    loop {
        let token = lexer.get_next_token()?;
        if (token == Token::Eof) {
            break;
        }
        v.push(token);
    }
    v.push(Token::Eof);
    Ok(v)
}

pub fn start(history_file: &str) -> rustyline::Result<()> {
    let mut rl = rustyline::Editor::<()>::with_config(
        rustyline::Config::builder()
            .edit_mode(rustyline::EditMode::Vi)
            .auto_add_history(true)
            .build(),
    )?;
    if let Err(e) = rl.load_history(history_file) {
        println!("Falied to load the history file `{}`: {}", history_file, e);
    }

    let evaluator = Evaluator::new();
    let mut env = Environment::new(None);

    loop {
        match rl.readline("\n>> ") {
            Err(_) => break,
            Ok(line) => {
                if (line.trim().is_empty()) {
                    continue;
                }

                let tokens = match get_tokens(&line) {
                    Err(e) => {
                        println!("{}{}{}", COLOR_RED, e, COLOR_END);
                        continue;
                    }
                    Ok(v) => {
                        println!("{:?}", v);
                        v
                    }
                };
                let mut parser = Parser::new(tokens);

                match parser.parse() {
                    Err(e) => println!("{}{}{}", COLOR_RED, e, COLOR_END),
                    Ok(e) => {
                        // println!("{:#?}", e);
                        match evaluator.eval(&e, &mut env) {
                            Ok(e) => println!("{}{}{}", COLOR_PURPLE, e, COLOR_END),
                            Err(e) => println!("{}{}{}", COLOR_RED, e, COLOR_END),
                        }
                    }
                }
            }
        }
    }

    rl.save_history(history_file)
}
