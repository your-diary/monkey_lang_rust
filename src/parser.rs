use std::mem;

use super::ast::*;
use super::token::Token;

enum Precedence {
    Lowest = 0,
    Equals,      //`==`
    LessGreater, //`<`, `>`
    Sum,         //`+`
    Product,     //`*`
    Unary,       //`-`, `!`
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
        let identifier = IdentifierNode::new(self.tokens[self.index].clone());
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
            identifier,
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
            Some(e) => e.clone(),
        };
        let expr = match self.parse_expression(Precedence::Lowest) {
            None => {
                return None;
            }
            Some(e) => e,
        };
        let ret = ExpressionStatementNode::new(current_token, expr);
        let next_token = self.tokens.get(self.index + 1);
        if (next_token.is_some() && (next_token.unwrap() == &Token::Semicolon)) {
            self.index += 1
        }
        Some(ret)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn ExpressionNode>> {
        //TODO
        let current_token = self.tokens.get(self.index);
        match current_token {
            Some(Token::Ident(_)) => self.parse_identifier().map(|e| Box::new(e) as _),
            Some(Token::Int(_)) => self.parse_integer_literal().map(|e| Box::new(e) as _),
            Some(Token::Invert) => self.parse_prefix_expression().map(|e| Box::new(e) as _),
            Some(Token::Minus) => self.parse_prefix_expression().map(|e| Box::new(e) as _),
            _ => None,
        }
    }

    fn parse_identifier(&self) -> Option<IdentifierNode> {
        //TODO
        self.tokens
            .get(self.index)
            .map(|e| IdentifierNode::new(e.clone()))
    }

    fn parse_integer_literal(&self) -> Option<IntegerLiteralNode> {
        //TODO
        self.tokens
            .get(self.index)
            .map(|e| IntegerLiteralNode::new(e.clone()))
    }

    fn parse_prefix_expression(&mut self) -> Option<UnaryExpressionNode> {
        match self.tokens.get(self.index) {
            None => None,
            Some(e) => {
                let operator = e.clone();
                self.index += 1;
                self.parse_expression(Precedence::Unary)
                    .map(|e| UnaryExpressionNode::new(operator, e))
            }
        }
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
            let identifier = s.identifier();
            assert!(matches!(identifier.token(), Token::Ident(s) if (s == name)));
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
            let v = s
                .expression()
                .as_any()
                .downcast_ref::<ast::IdentifierNode>();
            assert!(v.is_some());
            let v = v.unwrap();
            assert_eq!(v.token(), &Token::Ident(String::from("foobar")));
        }
    }

    #[test]
    fn test04() {
        let input = r#"
                5;
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
            assert_eq!(s.token(), &Token::Int(5));
            let v = s
                .expression()
                .as_any()
                .downcast_ref::<ast::IntegerLiteralNode>();
            assert!(v.is_some());
            let v = v.unwrap();
            assert_eq!(v.token(), &Token::Int(5));
        }
    }

    #[test]
    fn test05() {
        let input = r#"
                    !5;
                    -15;
                "#;

        struct T {
            operator: Token,
            value: i32,
        }
        let l = vec![
            T {
                operator: Token::Invert,
                value: 5,
            },
            T {
                operator: Token::Minus,
                value: 15,
            },
        ];

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();

        assert_eq!(2, root.statements().len());

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ast::ExpressionStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_eq!(s.token(), &l[i].operator);
            let v = s
                .expression()
                .as_any()
                .downcast_ref::<ast::UnaryExpressionNode>();
            assert!(v.is_some());
            let v = v.unwrap();
            assert_eq!(v.operator(), &l[i].operator);
            let v = v
                .expression()
                .as_any()
                .downcast_ref::<ast::IntegerLiteralNode>();
            assert!(v.is_some());
            let v = v.unwrap();
            assert_eq!(v.token(), &Token::Int(l[i].value));
        }
    }
}
