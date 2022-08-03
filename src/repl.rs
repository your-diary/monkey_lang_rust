use std::io::{self, Write};

use super::lexer::Lexer;
use super::token::Token;

pub fn start() {
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
        let mut lexer = Lexer::new(&buf);
        loop {
            let token = lexer.get_next_token();
            println!("{:?}", token);
            if let Token::Eof = token {
                break;
            }
        }
    }
}
