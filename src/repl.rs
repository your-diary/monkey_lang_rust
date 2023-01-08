use rustyline;

use super::environment::Environment;
use super::evaluator::Evaluator;
use super::lexer::{Lexer, LexerResult};
use super::parser::Parser;
use super::token::Token;

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
                        println!("\u{001B}[091m{}\u{001B}[0m", e);
                        continue;
                    }
                    Ok(v) => {
                        println!("{:?}", v);
                        v
                    }
                };
                let mut parser = Parser::new(tokens);

                match parser.parse() {
                    Err(e) => println!("\u{001B}[091m{}\u{001B}[0m", e),
                    Ok(e) => {
                        // println!("{:#?}", e);
                        match evaluator.eval(&e, &mut env) {
                            Ok(e) => println!("\u{001B}[095m{}\u{001B}[0m", e),
                            Err(e) => println!("\u{001B}[091m{}\u{001B}[0m", e),
                        }
                    }
                }
            }
        }
    }

    rl.save_history(history_file)
}
