use std::mem;

use super::ast;
use super::lexer::Lexer;
use super::token::Token;

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    next_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut ret = Parser {
            lexer,
            current_token: None,
            next_token: None,
        };
        ret.parse_next_token();
        ret.parse_next_token();
        ret
    }

    pub fn parse_next_token(&mut self) {
        mem::swap(&mut self.current_token, &mut self.next_token);
        self.next_token = Some(self.lexer.get_next_token());
    }
}
