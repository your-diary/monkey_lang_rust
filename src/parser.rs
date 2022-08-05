use std::mem;

use super::ast::*;
use super::token::Token;

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    Lowest = 0,
    Equals,      //`==`
    LessGreater, //`<`, `>`
    Sum,         //`+`
    Product,     //`*`
    Unary,       //`-`, `!`
    Call,        //`f()`
}

fn lookup_precedence(token: &Token) -> Precedence {
    match token {
        // Token::Assign => Precedence::Sum,
        Token::Plus => Precedence::Sum,
        Token::Minus => Precedence::Sum,
        Token::Asterisk => Precedence::Product,
        Token::Slash => Precedence::Product,
        // Token::Invert => Precedence::Sum,
        Token::Eq => Precedence::Equals,
        Token::NotEq => Precedence::Equals,
        Token::Lt => Precedence::LessGreater,
        Token::Gt => Precedence::LessGreater,
        // Token::Comma => Precedence::Sum,
        // Token::Semicolon => Precedence::Sum,
        // Token::Lparen => Precedence::Sum,
        Token::Rparen => Precedence::Lowest,
        // Token::Lbrace => Precedence::Sum,
        // Token::Rbrace => Precedence::Sum,
        // Token::Function => Precedence::Sum,
        // Token::Let => Precedence::Sum,
        // Token::Return => Precedence::Sum,
        // Token::True => Precedence::Sum,
        // Token::False => Precedence::Sum,
        // Token::If => Precedence::Sum,
        // Token::Else => Precedence::Sum,
        _ => Precedence::Lowest,
    }
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
        let mut expr: Box<dyn ExpressionNode> = match self.tokens.get(self.index) {
            Some(Token::Lparen) => self.parse_grouped_expression(),
            Some(Token::Ident(_)) => self.parse_identifier().map(|e| Box::new(e) as _),
            Some(Token::Int(_)) => self.parse_integer_literal().map(|e| Box::new(e) as _),
            Some(Token::Invert) => self.parse_unary_expression().map(|e| Box::new(e) as _),
            Some(Token::Minus) => self.parse_unary_expression().map(|e| Box::new(e) as _),
            Some(Token::If) => self.parse_if_expression().map(|e| Box::new(e) as _),
            Some(Token::True) => self.parse_boolean_literal().map(|e| Box::new(e) as _),
            Some(Token::False) => self.parse_boolean_literal().map(|e| Box::new(e) as _),
            _ => None,
        }?;
        loop {
            let next_token = match self.tokens.get(self.index + 1) {
                None => return None,
                Some(e) => e,
            };
            if !((next_token != &Token::Semicolon)
                && (next_token != &Token::Eof)
                && (precedence < lookup_precedence(next_token)))
            {
                break;
            }
            self.index += 1;
            expr = match self.parse_binary_expression(expr) {
                None => return None,
                Some(e) => Box::new(e) as _,
            };
        }
        Some(expr)
    }

    //Note `Token::Rparen` has the lowest `Precedence`.
    //That's why this simple method works.
    fn parse_grouped_expression(&mut self) -> Option<Box<dyn ExpressionNode>> {
        self.index += 1;
        match self.parse_expression(Precedence::Lowest) {
            None => None,
            Some(e) => {
                if self.expect_and_peek(Token::Rparen) {
                    Some(e)
                } else {
                    None
                }
            }
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

    fn parse_boolean_literal(&self) -> Option<BooleanLiteralNode> {
        //TODO
        self.tokens
            .get(self.index)
            .map(|e| BooleanLiteralNode::new(e.clone()))
    }

    fn parse_unary_expression(&mut self) -> Option<UnaryExpressionNode> {
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

    fn parse_binary_expression(
        &mut self,
        left: Box<dyn ExpressionNode>,
    ) -> Option<BinaryExpressionNode> {
        match self.tokens.get(self.index) {
            None => None,
            Some(e) => {
                let operator = e.clone();
                self.index += 1;
                self.parse_expression(lookup_precedence(&operator))
                    .map(|right| BinaryExpressionNode::new(operator, left, right))
            }
        }
    }

    fn parse_if_expression(&mut self) -> Option<IfExpressionNode> {
        //if
        if !self.expect_and_peek(Token::Lparen) {
            return None;
        }
        self.index += 1;
        let condition = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_and_peek(Token::Rparen) {
            return None;
        }
        if !self.expect_and_peek(Token::Lbrace) {
            return None;
        }
        let ifValue = self.parse_block_statement()?;

        //else
        let elseValue = match self.expect_and_peek(Token::Else) {
            false => None,
            true => match self.expect_and_peek(Token::Lbrace) {
                false => None,
                true => self.parse_block_statement(),
            },
        };
        Some(IfExpressionNode::new(condition, ifValue, elseValue))
    }

    fn parse_block_statement(&mut self) -> Option<BlockStatementNode> {
        let mut ret = BlockStatementNode::new();
        loop {
            self.index += 1;
            let current_token = self.tokens.get(self.index)?;
            if (current_token == &Token::Eof) {
                return None;
            }
            if (current_token == &Token::Rbrace) {
                break;
            }
            ret.statements_mut().push(self.parse_statement()?);
        }
        Some(ret)
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
                true;
                false;
            "#;

        let mut parser = Parser::new(get_tokens(input));

        let l = vec![Token::True, Token::False];

        let root = parser.parse();

        assert_eq!(2, root.statements().len());

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ast::ExpressionStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_eq!(s.token(), &l[i]);
            let v = s
                .expression()
                .as_any()
                .downcast_ref::<ast::BooleanLiteralNode>();
            assert!(v.is_some());
            let v = v.unwrap();
            assert_eq!(v.token(), &l[i]);
        }
    }

    #[test]
    fn test06() {
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

    #[test]
    fn test07() {
        let input = r#"
                    1 + 2;
                    1 - 2;
                    1 * 2;
                    1 / 2;
                    1 > 2;
                    1 < 2;
                    1 == 2;
                    1 != 2;
                "#;

        struct T {
            operator: Token,
            left: i32,
            right: i32,
        }
        impl T {
            fn new(operator: Token) -> Self {
                T {
                    operator,
                    left: 1,
                    right: 2,
                }
            }
        }
        let l = vec![
            T::new(Token::Plus),
            T::new(Token::Minus),
            T::new(Token::Asterisk),
            T::new(Token::Slash),
            T::new(Token::Gt),
            T::new(Token::Lt),
            T::new(Token::Eq),
            T::new(Token::NotEq),
        ];

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();

        assert_eq!(8, root.statements().len());

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ast::ExpressionStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_eq!(s.token(), &Token::Int(l[i].left));
            let v = s
                .expression()
                .as_any()
                .downcast_ref::<ast::BinaryExpressionNode>();
            assert!(v.is_some());
            let v = v.unwrap();
            assert_eq!(v.operator(), &l[i].operator);
            let left = v.left().as_any().downcast_ref::<ast::IntegerLiteralNode>();
            assert!(left.is_some());
            let left = left.unwrap();
            assert_eq!(left.token(), &Token::Int(l[i].left));
            let right = v.right().as_any().downcast_ref::<ast::IntegerLiteralNode>();
            assert!(right.is_some());
            let right = right.unwrap();
            assert_eq!(right.token(), &Token::Int(l[i].right));
        }
    }

    #[test]
    fn test08() {
        let input = r#"
                    1 + 2 * 3;
                "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        println!("{:#?}", root);

        assert_eq!(1, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ast::ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_eq!(s.token(), &Token::Int(1));
        let v = s
            .expression()
            .as_any()
            .downcast_ref::<ast::BinaryExpressionNode>();
        assert!(v.is_some());
        let v = v.unwrap();
        assert_eq!(v.operator(), &Token::Plus);
        let left = v.left().as_any().downcast_ref::<ast::IntegerLiteralNode>();
        assert!(left.is_some());
        let left = left.unwrap();
        assert_eq!(left.token(), &Token::Int(1));
        let right = v
            .right()
            .as_any()
            .downcast_ref::<ast::BinaryExpressionNode>();
        assert!(right.is_some());
        let right = right.unwrap();
        assert_eq!(right.operator(), &Token::Asterisk);
        let left = right
            .left()
            .as_any()
            .downcast_ref::<ast::IntegerLiteralNode>();
        let right = right
            .right()
            .as_any()
            .downcast_ref::<ast::IntegerLiteralNode>();
        assert!(left.is_some());
        assert!(right.is_some());
        assert_eq!(left.unwrap().token(), &Token::Int(2));
        assert_eq!(right.unwrap().token(), &Token::Int(3));
    }

    #[test]
    fn test09() {
        let input = r#"
                    (1 + 2) * 3;
                "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        println!("{:#?}", root);

        assert_eq!(1, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ast::ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_eq!(s.token(), &Token::Lparen);
        let v = s
            .expression()
            .as_any()
            .downcast_ref::<ast::BinaryExpressionNode>();
        assert!(v.is_some());
        let v = v.unwrap();
        assert_eq!(v.operator(), &Token::Asterisk);
        let left = v.right().as_any().downcast_ref::<ast::IntegerLiteralNode>();
        assert!(left.is_some());
        let left = left.unwrap();
        assert_eq!(left.token(), &Token::Int(3));
        let right = v
            .left()
            .as_any()
            .downcast_ref::<ast::BinaryExpressionNode>();
        assert!(right.is_some());
        let right = right.unwrap();
        assert_eq!(right.operator(), &Token::Plus);
        let left = right
            .left()
            .as_any()
            .downcast_ref::<ast::IntegerLiteralNode>();
        let right = right
            .right()
            .as_any()
            .downcast_ref::<ast::IntegerLiteralNode>();
        assert!(left.is_some());
        assert!(right.is_some());
        assert_eq!(left.unwrap().token(), &Token::Int(1));
        assert_eq!(right.unwrap().token(), &Token::Int(2));
    }

    #[test]
    fn test10() {
        let input = r#"
                    if (x < y) { x }; 5;
                "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        println!("{:#?}", root);

        assert_eq!(2, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ast::ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_eq!(s.token(), &Token::If);

        let v = s
            .expression()
            .as_any()
            .downcast_ref::<ast::IfExpressionNode>();
        assert!(v.is_some());
        let v = v.unwrap();
        assert_eq!(v.token(), &Token::If);

        let condition = v
            .condition()
            .as_any()
            .downcast_ref::<ast::BinaryExpressionNode>();
        assert!(condition.is_some());
        let condition = condition.unwrap();
        assert_eq!(condition.operator(), &Token::Lt);
        let left = condition
            .left()
            .as_any()
            .downcast_ref::<ast::IdentifierNode>();
        assert!(left.is_some());
        let left = left.unwrap();
        assert_eq!(left.token(), &Token::Ident("x".to_string()));
        let right = condition
            .right()
            .as_any()
            .downcast_ref::<ast::IdentifierNode>();
        assert!(right.is_some());
        let right = right.unwrap();
        assert_eq!(right.token(), &Token::Ident("y".to_string()));

        assert!(v.elseValue().is_none());

        let ifValue = v.ifValue();
        assert_eq!(ifValue.token(), &Token::Lbrace);
        let l = ifValue.statements();
        assert_eq!(1, l.len());
        let n = l[0].as_any().downcast_ref::<ast::ExpressionStatementNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.token(), &Token::Ident("x".to_string()));
        let e = n
            .expression()
            .as_any()
            .downcast_ref::<ast::IdentifierNode>();
        assert!(e.is_some());
        let e = e.unwrap();
        assert_eq!(e.token(), &Token::Ident("x".to_string()));

        let s = root.statements()[1]
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

    #[test]
    fn test11() {
        let input = r#"
                        if (x < y) {
                            x
                        } else {
                            3;
                            4;
                        };
                    "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        println!("{:#?}", root);

        assert_eq!(1, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ast::ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_eq!(s.token(), &Token::If);

        let v = s
            .expression()
            .as_any()
            .downcast_ref::<ast::IfExpressionNode>();
        assert!(v.is_some());
        let v = v.unwrap();
        assert_eq!(v.token(), &Token::If);

        let condition = v
            .condition()
            .as_any()
            .downcast_ref::<ast::BinaryExpressionNode>();
        assert!(condition.is_some());
        let condition = condition.unwrap();
        assert_eq!(condition.operator(), &Token::Lt);
        let left = condition
            .left()
            .as_any()
            .downcast_ref::<ast::IdentifierNode>();
        assert!(left.is_some());
        let left = left.unwrap();
        assert_eq!(left.token(), &Token::Ident("x".to_string()));
        let right = condition
            .right()
            .as_any()
            .downcast_ref::<ast::IdentifierNode>();
        assert!(right.is_some());
        let right = right.unwrap();
        assert_eq!(right.token(), &Token::Ident("y".to_string()));

        let ifValue = v.ifValue();
        assert_eq!(ifValue.token(), &Token::Lbrace);
        let l = ifValue.statements();
        assert_eq!(1, l.len());
        let n = l[0].as_any().downcast_ref::<ast::ExpressionStatementNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.token(), &Token::Ident("x".to_string()));
        let e = n
            .expression()
            .as_any()
            .downcast_ref::<ast::IdentifierNode>();
        assert!(e.is_some());
        let e = e.unwrap();
        assert_eq!(e.token(), &Token::Ident("x".to_string()));

        let elseValue = v.elseValue();
        assert!(elseValue.is_some());
        match elseValue {
            None => (),
            Some(elseValue) => {
                assert_eq!(elseValue.token(), &Token::Lbrace);
                let l = elseValue.statements();
                assert_eq!(2, l.len());

                let n = l[0].as_any().downcast_ref::<ast::ExpressionStatementNode>();
                assert!(n.is_some());
                let n = n.unwrap();
                assert_eq!(n.token(), &Token::Int(3));
                let e = n
                    .expression()
                    .as_any()
                    .downcast_ref::<ast::IntegerLiteralNode>();
                assert!(e.is_some());
                let e = e.unwrap();
                assert_eq!(e.token(), &Token::Int(3));

                let n = l[1].as_any().downcast_ref::<ast::ExpressionStatementNode>();
                assert!(n.is_some());
                let n = n.unwrap();
                assert_eq!(n.token(), &Token::Int(4));
                let e = n
                    .expression()
                    .as_any()
                    .downcast_ref::<ast::IntegerLiteralNode>();
                assert!(e.is_some());
                let e = e.unwrap();
                assert_eq!(e.token(), &Token::Int(4));
            }
        }
    }
}
