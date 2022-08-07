use std::io::{self, Write};

use super::environment::Environment;
use super::evaluator;
use super::lexer::Lexer;
use super::parser::Parser;
use super::token::Token;

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

pub fn start() {
    let mut env = Environment::new();
    loop {
        print!("\n>> ");
        io::stdout().flush().unwrap();

        let mut buf: String = String::new();
        if let Err(e) = io::stdin().read_line(&mut buf) {
            println!("{}", e);
            break;
        }
        if (buf.trim().is_empty()) {
            continue;
        }
        if ((buf.trim() == "q") || (buf.trim() == "exit")) {
            break;
        }
        let mut parser = Parser::new(get_tokens(&buf));
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
