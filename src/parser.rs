use std::mem;

use super::ast::{Identifier, LetStatement, Root, Statement};
use super::lexer::Lexer;
use super::token::{Token, TokenType};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    next_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut ret = Parser {
            lexer,
            current_token: Token::new(TokenType::Eof, None),
            next_token: Token::new(TokenType::Eof, None),
            errors: Vec::new(),
        };
        ret.parse_next_token();
        ret.parse_next_token();
        ret
    }

    fn parse_next_token(&mut self) {
        mem::swap(&mut self.current_token, &mut self.next_token);
        self.next_token = self.lexer.get_next_token();
    }

    fn parse_statement(&mut self) -> Option<impl Statement> {
        match self.current_token.tp() {
            TokenType::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn expect_and_peek(&mut self, tp: TokenType) -> bool {
        if (self.next_token.tp() == &tp) {
            self.parse_next_token();
            true
        } else {
            self.errors.push(format!(
                "expected next token to be {:?}, got {:?} instead",
                tp,
                self.next_token.tp()
            ));
            false
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        if (!self.expect_and_peek(TokenType::Ident)) {
            return None;
        }
        let left = Identifier::new(
            self.current_token.clone(),
            self.current_token.literal().clone().unwrap(),
        );
        if (!self.expect_and_peek(TokenType::Assign)) {
            return None;
        }
        while (self.current_token.tp() != &TokenType::Semicolon) {
            self.parse_next_token();
        }
        Some(LetStatement::new(
            left,
            Box::new(Identifier::new(
                Token::new(TokenType::Illegal, None),
                String::new(),
            )),
        ))
    }

    pub fn parse(&mut self) -> Root {
        let mut root = Root::new();
        while (self.current_token.tp() != &TokenType::Eof) {
            if let Some(e) = self.parse_statement() {
                root.statements_mut().push(Box::new(e));
            }
            self.parse_next_token();
        }
        root
    }
}

#[cfg(test)]
mod tests {

    use super::super::ast;
    use super::super::lexer::Lexer;
    use super::super::token::{Token, TokenType};
    use super::Parser;

    #[test]
    fn test01() {
        let input = r#"
            let x = 5;
            let ab = 10;
        "#;

        let mut parser = Parser::new(Lexer::new(&input));

        let root = parser.parse();

        assert_eq!(2, root.statements().len());
        assert_eq!(0, parser.errors.len());

        let names = vec!["x", "ab"];

        for i in 0..names.len() {
            let name = names[i];
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ast::LetStatement>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_eq!(s.token(), &Token::new(TokenType::Let, None));
            let left = s.left();
            assert_eq!(left.token(), &Token::new(TokenType::Ident, Some(name)));
            assert_eq!(left.value(), name);
        }
    }
}
