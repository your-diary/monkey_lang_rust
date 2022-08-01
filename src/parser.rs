use std::mem;

use super::ast::Root;
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

    pub fn parse(&mut self) -> Root {
        Root::new()
    }
}

#[cfg(test)]
mod tests {

    use super::super::ast;
    use super::super::lexer::Lexer;
    use super::super::token::{Token, TokenType};
    use super::super::util;
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
