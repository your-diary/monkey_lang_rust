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
    //✅
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, index: 0 }
    }

    //✅
    pub fn parse(&mut self) -> Option<RootNode> {
        let mut root = RootNode::new();
        loop {
            let current_token = self.tokens.get(self.index);
            if (current_token.is_none() || (current_token.unwrap() == &Token::Eof)) {
                break;
            }
            //empty statement
            if (current_token.unwrap() == &Token::Semicolon) {
                self.index += 1;
                continue;
            }
            root.statements_mut().push(self.parse_statement()?);
            self.index += 1
        }
        Some(root)
    }

    //✅
    fn parse_statement(&mut self) -> Option<Box<dyn StatementNode>> {
        match self.tokens.get(self.index) {
            None => None,
            Some(Token::Lbrace) => self.parse_block_statement().map(|e| Box::new(e) as _),
            Some(Token::Let) => self.parse_let_statement().map(|e| Box::new(e) as _),
            Some(Token::Return) => self.parse_return_statement().map(|e| Box::new(e) as _),
            Some(_) => self.parse_expression_statement().map(|e| Box::new(e) as _),
        }
    }

    //✅
    //asserts the variant of the next token without caring about its value,
    // and advances to it if true while staying at the same position if false
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

    //✅
    //{<statement(s)>}
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

    //✅
    //let <identifier> = <expression>;
    fn parse_let_statement(&mut self) -> Option<LetStatementNode> {
        if (!self.expect_and_peek(Token::Ident(String::new()))) {
            return None;
        }
        let identifier = IdentifierNode::new(self.tokens[self.index].clone());
        if (!self.expect_and_peek(Token::Assign)) {
            return None;
        }
        self.index += 1;
        let expr = self.parse_expression(Precedence::Lowest)?;
        if (!self.expect_and_peek(Token::Semicolon)) {
            return None;
        }
        Some(LetStatementNode::new(identifier, expr))
    }

    //✅
    //return <expression>;
    fn parse_return_statement(&mut self) -> Option<ReturnStatementNode> {
        self.index += 1;
        let expr = self.parse_expression(Precedence::Lowest)?;
        if (!self.expect_and_peek(Token::Semicolon)) {
            return None;
        }
        Some(ReturnStatementNode::new(expr))
    }

    //✅
    fn parse_expression_statement(&mut self) -> Option<ExpressionStatementNode> {
        let token = self.tokens.get(self.index)?.clone();
        let expr = self.parse_expression(Precedence::Lowest)?;
        let ret = ExpressionStatementNode::new(token, expr);
        self.expect_and_peek(Token::Semicolon); //we ignore the result as semicolon is optional
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
            Some(Token::Function) => self.parse_function_literal().map(|e| Box::new(e) as _),
            Some(Token::True) => self.parse_boolean_literal().map(|e| Box::new(e) as _),
            Some(Token::False) => self.parse_boolean_literal().map(|e| Box::new(e) as _),
            _ => None,
        }?;
        loop {
            let next_token = self.tokens.get(self.index + 1)?;
            if !((next_token != &Token::Semicolon)
                && (next_token != &Token::Eof)
                && (precedence < lookup_precedence(next_token)))
            {
                break;
            }
            self.index += 1;
            expr = Box::new(self.parse_binary_expression(expr)?) as _;
        }
        Some(expr)
    }

    //✅
    //(<expression>)
    //
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

    //✅
    fn parse_identifier(&self) -> Option<IdentifierNode> {
        self.tokens
            .get(self.index)
            .map(|e| IdentifierNode::new(e.clone()))
    }

    //✅
    fn parse_integer_literal(&self) -> Option<IntegerLiteralNode> {
        self.tokens
            .get(self.index)
            .map(|e| IntegerLiteralNode::new(e.clone()))
    }

    //✅
    fn parse_boolean_literal(&self) -> Option<BooleanLiteralNode> {
        self.tokens
            .get(self.index)
            .map(|e| BooleanLiteralNode::new(e.clone()))
    }

    //✅
    //<operator> <expression>
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

    //✅
    //<expression> <operator> <expression>
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

    //✅
    //if (<expression>) { <statement(s)> } [else { <statement(s)> }]
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
        let if_value = self.parse_block_statement()?;

        //else
        let else_value = match self.expect_and_peek(Token::Else) {
            false => None,
            true => match self.expect_and_peek(Token::Lbrace) {
                false => return None,
                true => Some(self.parse_block_statement()?),
            },
        };
        Some(IfExpressionNode::new(condition, if_value, else_value))
    }

    //fn (<parameter(s)>) { <statement(s)> }
    //
    //Examples of parameters:
    // ()
    // (a)
    // (a, b)
    fn parse_function_literal(&mut self) -> Option<FunctionExpressionNode> {
        if !self.expect_and_peek(Token::Lparen) {
            return None;
        }
        let mut parameters = Vec::new();
        loop {
            self.index += 1;
            match self.tokens.get(self.index)? {
                Token::Rparen => break,
                Token::Ident(_) => {
                    parameters.push(self.parse_identifier()?);
                    self.expect_and_peek(Token::Comma); //consumes a comma if exists
                }
                _ => return None,
            }
        }
        if !self.expect_and_peek(Token::Lbrace) {
            return None;
        }
        Some(FunctionExpressionNode::new(
            parameters,
            self.parse_block_statement()?,
        ))
    }
}

#[cfg(test)]
mod tests {

    use super::super::ast::*;
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

    fn assert_integer_literal(n: &dyn ExpressionNode, i: i32) {
        let n = n.as_any().downcast_ref::<IntegerLiteralNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.token(), &Token::Int(i));
    }

    fn assert_boolean_literal(n: &dyn ExpressionNode, b: bool) {
        let n = n.as_any().downcast_ref::<BooleanLiteralNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.token(), &(if b { Token::True } else { Token::False }));
    }

    fn assert_identifier(n: &dyn ExpressionNode, s: &str) {
        let n = n.as_any().downcast_ref::<IdentifierNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.token(), &Token::Ident(s.to_string()));
    }

    fn assert_unary_expression(n: &dyn ExpressionNode, op: &Token, i: i32) {
        let n = n.as_any().downcast_ref::<UnaryExpressionNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.operator(), op);
        let e = n.expression().as_any().downcast_ref::<IntegerLiteralNode>();
        assert!(e.is_some());
        let e = e.unwrap();
        assert_eq!(e.token(), &Token::Int(i));
    }

    fn assert_binary_expression(n: &dyn ExpressionNode, op: &Token, i: i32, j: i32) {
        let n = n.as_any().downcast_ref::<BinaryExpressionNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.operator(), op);

        let e = n.left().as_any().downcast_ref::<IntegerLiteralNode>();
        assert!(e.is_some());
        let e = e.unwrap();
        assert_eq!(e.token(), &Token::Int(i));

        let e = n.right().as_any().downcast_ref::<IntegerLiteralNode>();
        assert!(e.is_some());
        let e = e.unwrap();
        assert_eq!(e.token(), &Token::Int(j));
    }

    fn assert_binary_expression_2(n: &dyn ExpressionNode, op: &Token, s1: &str, s2: &str) {
        let n = n.as_any().downcast_ref::<BinaryExpressionNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.operator(), op);

        let e = n.left().as_any().downcast_ref::<IdentifierNode>();
        assert!(e.is_some());
        let e = e.unwrap();
        assert_eq!(e.token(), &Token::Ident(s1.to_string()));

        let e = n.right().as_any().downcast_ref::<IdentifierNode>();
        assert!(e.is_some());
        let e = e.unwrap();
        assert_eq!(e.token(), &Token::Ident(s2.to_string()));
    }

    #[test]
    fn test01() {
        let input = r#"
            let x = 5;
            let ab = 10;
        "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_some());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(2, root.statements().len());

        let names = vec!["x", "ab"];
        let values = vec![5, 10];

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<LetStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_eq!(s.token(), &Token::Let);
            assert_identifier(s.identifier(), names[i]);
            assert_integer_literal(s.expression(), values[i]);
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
        assert!(root.is_some());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(2, root.statements().len());

        let values = vec![5, 10];

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ReturnStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_eq!(s.token(), &Token::Return);
            assert_integer_literal(s.expression(), values[i]);
        }
    }

    #[test]
    fn test03() {
        let input = r#"
                foo;;
                bar;
            "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_some());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(2, root.statements().len());

        let names = vec!["foo", "bar"];

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ExpressionStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_eq!(s.token(), &Token::Ident(names[i].to_string()));
            assert_identifier(s.expression(), names[i]);
        }
    }

    #[test]
    fn test04() {
        let input = r#"
                5;
            "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_some());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(1, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_eq!(s.token(), &Token::Int(5));
        assert_integer_literal(s.expression(), 5);
    }

    #[test]
    fn test05() {
        let input = r#"
                true;
                false;
            "#;

        let mut parser = Parser::new(get_tokens(input));

        let l = vec![true, false];

        let root = parser.parse();
        assert!(root.is_some());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(2, root.statements().len());

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ExpressionStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_eq!(s.token(), &(if l[i] { Token::True } else { Token::False }));
            assert_boolean_literal(s.expression(), l[i]);
        }
    }

    #[test]
    fn test06() {
        let input = r#"
                    !5;
                    -15;
                "#;

        let operators = vec![Token::Invert, Token::Minus];
        let values = vec![5, 15];

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_some());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(2, root.statements().len());

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ExpressionStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_eq!(s.token(), &operators[i]);
            assert_unary_expression(s.expression(), &operators[i], values[i]);
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

        let operators = vec![
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::Slash,
            Token::Gt,
            Token::Lt,
            Token::Eq,
            Token::NotEq,
        ];

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_some());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(8, root.statements().len());

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ExpressionStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_eq!(s.token(), &Token::Int(1));
            assert_binary_expression(s.expression(), &operators[i], 1, 2);
        }
    }

    #[test]
    fn test08() {
        let input = r#"
                    1 + 2 * 3;
                "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_some());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(1, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_eq!(s.token(), &Token::Int(1));
        let v = s
            .expression()
            .as_any()
            .downcast_ref::<BinaryExpressionNode>();
        assert!(v.is_some());
        let v = v.unwrap();
        assert_eq!(v.operator(), &Token::Plus);
        assert_integer_literal(v.left(), 1);
        assert_binary_expression(v.right(), &Token::Asterisk, 2, 3);
    }

    #[test]
    fn test09() {
        let input = r#"
                    (1 + 2) * 3;
                "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_some());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(1, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_eq!(s.token(), &Token::Lparen);
        let v = s
            .expression()
            .as_any()
            .downcast_ref::<BinaryExpressionNode>();
        assert!(v.is_some());
        let v = v.unwrap();
        assert_eq!(v.operator(), &Token::Asterisk);
        assert_binary_expression(v.left(), &Token::Plus, 1, 2);
        assert_integer_literal(v.right(), 3);
    }

    #[test]
    fn test10() {
        let input = r#"
                    if (x < y) { x }; 5;
                "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_some());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(2, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_eq!(s.token(), &Token::If);

        let v = s.expression().as_any().downcast_ref::<IfExpressionNode>();
        assert!(v.is_some());
        let v = v.unwrap();
        assert_eq!(v.token(), &Token::If);

        assert_binary_expression_2(v.condition(), &Token::Lt, "x", "y");

        assert!(v.else_value().is_none());

        let if_value = v.if_value();
        assert_eq!(if_value.token(), &Token::Lbrace);
        let l = if_value.statements();
        assert_eq!(1, l.len());
        let n = l[0].as_any().downcast_ref::<ExpressionStatementNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.token(), &Token::Ident("x".to_string()));
        assert_identifier(n.expression(), "x");

        let s = root.statements()[1]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_eq!(s.token(), &Token::Int(5));
        assert_integer_literal(s.expression(), 5);
    }

    #[test]
    fn test11() {
        let input = r#"
                        if (x != y) {
                            x
                        } else {
                            3;
                            4;
                        };
                    "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_some());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(1, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_eq!(s.token(), &Token::If);

        let v = s.expression().as_any().downcast_ref::<IfExpressionNode>();
        assert!(v.is_some());
        let v = v.unwrap();
        assert_eq!(v.token(), &Token::If);

        assert_binary_expression_2(v.condition(), &Token::NotEq, "x", "y");

        let if_value = v.if_value();
        assert_eq!(if_value.token(), &Token::Lbrace);
        let l = if_value.statements();
        assert_eq!(1, l.len());
        let n = l[0].as_any().downcast_ref::<ExpressionStatementNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.token(), &Token::Ident("x".to_string()));
        assert_identifier(n.expression(), "x");

        match v.else_value() {
            None => assert!(v.else_value().is_some()),
            Some(else_value) => {
                assert_eq!(else_value.token(), &Token::Lbrace);
                let l = else_value.statements();
                assert_eq!(2, l.len());

                let n = l[0].as_any().downcast_ref::<ExpressionStatementNode>();
                assert!(n.is_some());
                let n = n.unwrap();
                assert_eq!(n.token(), &Token::Int(3));
                assert_integer_literal(n.expression(), 3);

                let n = l[1].as_any().downcast_ref::<ExpressionStatementNode>();
                assert!(n.is_some());
                let n = n.unwrap();
                assert_eq!(n.token(), &Token::Int(4));
                assert_integer_literal(n.expression(), 4);
            }
        }
    }

    #[test]
    fn test12() {
        let input = r#"
                    fn () {
                        return x + y;
                    }
                    fn (x) {
                        return x + y;
                    }
                    fn (x, y) {
                        return x + y;
                    }
                "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_some());
        let root = root.unwrap();
        println!("{:#?}", root);

        let parameters = vec!["x", "y"];

        assert_eq!(3, root.statements().len());

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ExpressionStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_eq!(s.token(), &Token::Function);

            let v = s
                .expression()
                .as_any()
                .downcast_ref::<FunctionExpressionNode>();
            assert!(v.is_some());
            let v = v.unwrap();
            assert_eq!(v.token(), &Token::Function);

            assert_eq!(i, v.parameters().len());
            if (i >= 1) {
                assert_identifier(&v.parameters()[0], "x");
            }
            if (i == 2) {
                assert_identifier(&v.parameters()[1], "y");
            }

            let s = v.body();
            assert_eq!(s.token(), &Token::Lbrace);
            assert_eq!(1, s.statements().len());
            let s = v.body().statements()[0]
                .as_any()
                .downcast_ref::<ReturnStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_eq!(s.token(), &Token::Return);
            assert_binary_expression_2(s.expression(), &Token::Plus, "x", "y");
        }
    }
}
