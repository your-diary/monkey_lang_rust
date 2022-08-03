use std::mem;

use super::ast::*;
use super::token::Token;

enum Precedence {
    Lowest = 0,
    Equals,      //`==`
    LessGreater, //`<`, `>`
    Sum,         //`+`
    Product,     //`*`
    Prefix,      //`-`, `!`
    Call,        //`f()`
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> RootNode {
        let mut root = RootNode::new();
        loop {
            let current_token = self.tokens.get(self.index);
            if (current_token.is_none() || (current_token.unwrap() == &Token::Eof)) {
                break;
            }
            if let Some(e) = self.parse_statement() {
                root.statements_mut().push(e);
            }
            self.index += 1
        }
        root
    }

    fn parse_statement(&mut self) -> Option<Box<dyn StatementNode>> {
        match self.tokens.get(self.index) {
            None => None,
            Some(Token::Let) => self.parse_let_statement().map(|e| Box::new(e) as _),
            Some(Token::Return) => self.parse_return_statement().map(|e| Box::new(e) as _),
            Some(_) => self.parse_expression_statement().map(|e| Box::new(e) as _),
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatementNode> {
        if (!self.expect_and_peek(Token::Ident(String::new()))) {
            return None;
        }
        let left = IdentifierNode::new(self.tokens[self.index].clone());
        if (!self.expect_and_peek(Token::Assign)) {
            return None;
        }
        //TODO
        loop {
            let current_token = self.tokens.get(self.index);
            if (current_token.is_some() && (current_token.unwrap() != &Token::Semicolon)) {
                self.index += 1;
            } else {
                break;
            }
        }
        Some(LetStatementNode::new(
            left,
            Box::new(IdentifierNode::new(Token::Ident(String::new()))),
        ))
    }

    fn expect_and_peek(&mut self, token: Token) -> bool {
        let next_token = self.tokens.get(self.index + 1);
        if (next_token.is_some()
            && (mem::discriminant(next_token.unwrap()) == mem::discriminant(&token)))
        {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatementNode> {
        //TODO
        loop {
            let current_token = self.tokens.get(self.index);
            if (current_token.is_none() || (current_token.unwrap() == &Token::Semicolon)) {
                break;
            }
            self.index += 1;
        }
        Some(ReturnStatementNode::new(Box::new(IdentifierNode::new(
            Token::Ident(String::new()),
        ))))
    }

    fn parse_expression_statement(&mut self) -> Option<ExpressionStatementNode> {
        let current_token = match self.tokens.get(self.index) {
            None => {
                return None;
            }
            Some(e) => e,
        };
        let expr = match self.parse_expression(Precedence::Lowest) {
            None => {
                return None;
            }
            Some(e) => e,
        };
        let ret = ExpressionStatementNode::new(current_token.clone(), expr);
        let next_token = self.tokens.get(self.index + 1);
        if (next_token.is_some() && (next_token.unwrap() == &Token::Semicolon)) {
            self.index += 1
        }
        Some(ret)
    }

    fn parse_expression(&self, precedence: Precedence) -> Option<Box<dyn ExpressionNode>> {
        //TODO
        self.parse_identifier().map(|e| Box::new(e) as _)
    }

    fn parse_identifier(&self) -> Option<IdentifierNode> {
        //TODO
        self.tokens
            .get(self.index)
            .map(|e| IdentifierNode::new(e.clone()))
    }
}

#[cfg(test)]
mod tests {

    use super::super::ast;
    use super::super::lexer::Lexer;
    use super::super::token::Token;
    use super::Parser;

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

    #[test]
    fn test01() {
        let input = r#"
            let x = 5;
            let ab = 10;
        "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();

        assert_eq!(2, root.statements().len());

        let names = vec!["x", "ab"];

        for i in 0..root.statements().len() {
            let name = names[i];
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ast::LetStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_eq!(s.token(), &Token::Let);
            let left = s.left();
            assert!(matches!(left.token(), Token::Ident(s) if (s == name)));
        }
    }

    #[test]
    fn test02() {
        let input = r#"
                    return 5;
                    return 10;
                "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();

        assert_eq!(2, root.statements().len());

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ast::ReturnStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_eq!(s.token(), &Token::Return);
        }
    }

    #[test]
    fn test03() {
        let input = r#"
                foobar;
            "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();

        assert_eq!(1, root.statements().len());

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ast::ExpressionStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_eq!(s.token(), &Token::Ident(String::from("foobar")));
            let v = s.value().as_any().downcast_ref::<ast::IdentifierNode>();
            assert!(v.is_some());
            let v = v.unwrap();
            assert_eq!(v.token(), &Token::Ident(String::from("foobar")));
        }
    }
}
