use std::collections::VecDeque;
use std::fmt::{self, Display};
use std::mem;

use super::ast::*;
use super::token::Token;

/*-------------------------------------*/

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    Lowest = 0,
    Or,      //`||`
    And,     //`&&`
    Cmp,     //`==`, `!=`, `<`, `>`, `>=`, `<=`
    Sum,     //`+`, `-`
    Product, //`*`, `/`, `%`, `**`
    Unary,   //`-`, `!`
    Call,    //`(`, `[`
}

fn lookup_precedence(token: &Token) -> Precedence {
    match token {
        Token::Or => Precedence::Or,
        Token::And => Precedence::And,
        Token::Eq => Precedence::Cmp,
        Token::NotEq => Precedence::Cmp,
        Token::Lt => Precedence::Cmp,
        Token::Gt => Precedence::Cmp,
        Token::LtEq => Precedence::Cmp,
        Token::GtEq => Precedence::Cmp,
        Token::Plus => Precedence::Sum,
        Token::Minus => Precedence::Sum,
        Token::Asterisk => Precedence::Product,
        Token::Slash => Precedence::Product,
        Token::Percent => Precedence::Product,
        Token::Power => Precedence::Product,
        Token::Lparen => Precedence::Call,
        Token::Lbracket => Precedence::Call,
        Token::Rparen => Precedence::Lowest,
        Token::Rbracket => Precedence::Lowest,
        _ => Precedence::Lowest,
    }
}

/*-------------------------------------*/

type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    Eof,
    Error(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Eof => "eof",
                Self::Error(s) => s,
            }
        )
    }
}

/*-------------------------------------*/

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        assert!(!tokens.is_empty());
        assert_eq!(tokens.last().unwrap(), &Token::Eof);
        Parser {
            tokens: VecDeque::from(tokens),
        }
    }

    fn get_next(&mut self) -> ParseResult<Token> {
        match self.tokens.pop_front() {
            None => unreachable!(), //at least `Eof` is assumed to exist as a guardian
            Some(Token::Eof) => Err(ParseError::Eof),
            Some(t) => Ok(t),
        }
    }

    fn peek_next(&self) -> ParseResult<&Token> {
        match self.tokens.get(0) {
            None => unreachable!(), //at least `Eof` is assumed to exist as a guardian
            Some(Token::Eof) => Err(ParseError::Eof),
            Some(t) => Ok(t),
        }
    }

    pub fn parse(&mut self) -> ParseResult<RootNode> {
        let mut statements = vec![];
        //reads the next statement
        loop {
            if (self.tokens[0] == Token::Eof) {
                break;
            }
            //empty statement
            if (self.expect_next(Token::Semicolon)) {
                self.get_next().unwrap();
                continue;
            }
            let statement = match self.parse_statement() {
                Err(ParseError::Eof) => {
                    return Err(ParseError::Error(
                        "unexpected eof in the middle of a statement".to_string(),
                    ))
                }
                Err(e) => return Err(e),
                Ok(e) => e,
            };
            statements.push(statement);
        }
        Ok(RootNode::new(statements))
    }

    fn parse_statement(&mut self) -> ParseResult<Box<dyn StatementNode>> {
        match self.peek_next()? {
            Token::Let => self.parse_let_statement().map(|e| Box::new(e) as _),
            Token::Return => self.parse_return_statement().map(|e| Box::new(e) as _),
            _ => self.parse_expression_statement().map(|e| Box::new(e) as _),
        }
    }

    //asserts the variant of the next token without caring about its value,
    // and advances to it if true while staying at the same position if false
    fn expect_next(&mut self, token: Token) -> bool {
        let next = self.peek_next();
        next.is_ok() && (mem::discriminant(next.unwrap()) == mem::discriminant(&token))
    }

    //{<statement(s)>}
    fn parse_block_expression(&mut self) -> ParseResult<BlockExpressionNode> {
        assert_eq!(Token::Lbrace, self.get_next().unwrap());
        let mut statements = vec![];
        loop {
            if (self.peek_next()? == &Token::Rbrace) {
                self.get_next().unwrap();
                break;
            }
            statements.push(self.parse_statement()?.into());
        }
        Ok(BlockExpressionNode::new(statements))
    }

    //let <identifier> = <expression>;
    fn parse_let_statement(&mut self) -> ParseResult<LetStatementNode> {
        assert_eq!(Token::Let, self.get_next().unwrap());

        if (!self.expect_next(Token::Ident(String::new()))) {
            return Err(ParseError::Error(
                "identifier missing or reserved keyword used after `let`".to_string(),
            ));
        }
        let identifier = IdentifierNode::new(self.get_next()?);

        if (!self.expect_next(Token::Assign)) {
            return Err(ParseError::Error("`=` missing in `let`".to_string()));
        }
        self.get_next().unwrap();

        let expr = self.parse_expression(Precedence::Lowest)?;

        if (!self.expect_next(Token::Semicolon)) {
            return Err(ParseError::Error("`;` missing in `let`".to_string()));
        }
        self.get_next().unwrap();

        Ok(LetStatementNode::new(identifier, expr))
    }

    //return [<expression>];
    fn parse_return_statement(&mut self) -> ParseResult<ReturnStatementNode> {
        assert_eq!(Token::Return, self.get_next().unwrap());
        if (self.expect_next(Token::Semicolon)) {
            self.get_next().unwrap();
            return Ok(ReturnStatementNode::new(None));
        }
        let expr = self.parse_expression(Precedence::Lowest)?;
        if (!self.expect_next(Token::Semicolon)) {
            return Err(ParseError::Error("`;` missing in `return`".to_string()));
        }
        self.get_next().unwrap();
        Ok(ReturnStatementNode::new(Some(expr)))
    }

    //<expression>[;]
    fn parse_expression_statement(&mut self) -> ParseResult<ExpressionStatementNode> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        if (self.expect_next(Token::Semicolon)) {
            self.get_next().unwrap();
        }
        Ok(ExpressionStatementNode::new(expr))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> ParseResult<Box<dyn ExpressionNode>> {
        //parses first expression
        let mut expr: Box<dyn ExpressionNode> = match self.peek_next()? {
            Token::Lbrace => self.parse_block_expression().map(|e| Box::new(e) as _),
            Token::Lparen => self.parse_grouped_expression(),
            Token::Ident(_) => self.parse_identifier().map(|e| Box::new(e) as _),
            Token::Int(_) => self.parse_integer_literal().map(|e| Box::new(e) as _),
            Token::Float(_) => self.parse_float_literal().map(|e| Box::new(e) as _),
            Token::True => self.parse_boolean_literal().map(|e| Box::new(e) as _),
            Token::False => self.parse_boolean_literal().map(|e| Box::new(e) as _),
            Token::Char(_) => self.parse_character_literal().map(|e| Box::new(e) as _),
            Token::String(_) => self.parse_string_literal().map(|e| Box::new(e) as _),
            Token::Lbracket => self.parse_array_literal().map(|e| Box::new(e) as _),
            Token::Invert => self.parse_unary_expression().map(|e| Box::new(e) as _),
            Token::Minus => self.parse_unary_expression().map(|e| Box::new(e) as _),
            Token::If => self.parse_if_expression().map(|e| Box::new(e) as _),
            Token::Function => self.parse_function_literal().map(|e| Box::new(e) as _),
            t => Err(ParseError::Error(format!(
                "unexpected start of expression: {:?}",
                t
            ))),
        }?;

        //parses a binary expression or a call/index expression if the next token is a binary operator, `(` or `[`
        loop {
            let next = match self.peek_next() {
                Err(ParseError::Eof) => break,
                Err(_) => unreachable!(),
                Ok(e) => e,
            };
            if ((next == &Token::Semicolon) || (precedence >= lookup_precedence(next))) {
                break;
            }
            expr = match next {
                Token::Lparen => Box::new(self.parse_call_expression(expr)?) as _,
                Token::Lbracket => Box::new(self.parse_index_expression(expr)?) as _,
                _ => Box::new(self.parse_binary_expression(expr)?) as _,
            };
        }

        Ok(expr)
    }

    //(<expression>)
    //
    //Note `Token::Rparen` has the lowest `Precedence`.
    //That's why this simple method works.
    fn parse_grouped_expression(&mut self) -> ParseResult<Box<dyn ExpressionNode>> {
        assert_eq!(Token::Lparen, self.get_next().unwrap());
        let expr = self.parse_expression(Precedence::Lowest)?;
        if (!self.expect_next(Token::Rparen)) {
            return Err(ParseError::Error(
                "`)` missing in grouped expression".to_string(),
            ));
        }
        self.get_next().unwrap();
        Ok(expr)
    }

    fn parse_identifier(&mut self) -> ParseResult<IdentifierNode> {
        Ok(IdentifierNode::new(self.get_next()?))
    }

    fn parse_integer_literal(&mut self) -> ParseResult<IntegerLiteralNode> {
        Ok(IntegerLiteralNode::new(self.get_next()?))
    }

    fn parse_float_literal(&mut self) -> ParseResult<FloatLiteralNode> {
        Ok(FloatLiteralNode::new(self.get_next()?))
    }

    fn parse_boolean_literal(&mut self) -> ParseResult<BooleanLiteralNode> {
        Ok(BooleanLiteralNode::new(self.get_next()?))
    }

    fn parse_character_literal(&mut self) -> ParseResult<CharacterLiteralNode> {
        Ok(CharacterLiteralNode::new(self.get_next()?))
    }

    fn parse_string_literal(&mut self) -> ParseResult<StringLiteralNode> {
        Ok(StringLiteralNode::new(self.get_next()?))
    }

    //[<e1>, <e2>, ...]
    //The last <e> can optionally be followed by a comma (e.g. `[1, 2, 3,]`).
    fn parse_array_literal(&mut self) -> ParseResult<ArrayLiteralNode> {
        assert_eq!(Token::Lbracket, self.get_next().unwrap());
        let mut elements = vec![];
        loop {
            match self.peek_next()? {
                Token::Rbracket => {
                    self.get_next().unwrap();
                    break;
                }
                _ => {
                    elements.push(self.parse_expression(Precedence::Lowest)?);
                    if (self.expect_next(Token::Comma)) {
                        self.get_next().unwrap();
                    }
                }
            }
        }
        Ok(ArrayLiteralNode::new(elements))
    }

    //<operator> <expression>
    fn parse_unary_expression(&mut self) -> ParseResult<UnaryExpressionNode> {
        let operator = self.get_next()?;
        Ok(UnaryExpressionNode::new(
            operator,
            self.parse_expression(Precedence::Unary)?,
        ))
    }

    //<expression> <operator> <expression>
    fn parse_binary_expression(
        &mut self,
        left: Box<dyn ExpressionNode>,
    ) -> ParseResult<BinaryExpressionNode> {
        let operator = self.get_next()?;
        let right = self.parse_expression(lookup_precedence(&operator))?;
        Ok(BinaryExpressionNode::new(operator, left, right))
    }

    //<array name or array literal>[<index>]
    fn parse_index_expression(
        &mut self,
        array: Box<dyn ExpressionNode>,
    ) -> ParseResult<IndexExpressionNode> {
        assert_eq!(Token::Lbracket, self.get_next().unwrap());
        if (self.expect_next(Token::Rbracket)) {
            return Err(ParseError::Error(
                "empty index in array index expression".to_string(),
            ));
        }
        let index = self.parse_expression(Precedence::Lowest)?;
        if (!self.expect_next(Token::Rbracket)) {
            return Err(ParseError::Error(
                "`]` missing in array index expression".to_string(),
            ));
        }
        self.get_next().unwrap();
        Ok(IndexExpressionNode::new(array, index))
    }

    //<function name or function literal>(<argument(s)>)
    //
    //The last <argument> can optionally be followed by a comma (e.g. `(a, b,)`).
    //
    //Examples of arguments:
    // ()
    // (a)
    // (a, b * c)
    fn parse_call_expression(
        &mut self,
        function: Box<dyn ExpressionNode>,
    ) -> ParseResult<CallExpressionNode> {
        assert_eq!(Token::Lparen, self.get_next().unwrap());
        let mut arguments = vec![];
        loop {
            match self.peek_next()? {
                Token::Rparen => {
                    self.get_next().unwrap();
                    break;
                }
                _ => {
                    arguments.push(self.parse_expression(Precedence::Lowest)?);
                    if (self.expect_next(Token::Comma)) {
                        self.get_next().unwrap();
                    }
                }
            }
        }
        Ok(CallExpressionNode::new(function, arguments))
    }

    //if (<expression>) { <statement(s)> } [else { <statement(s)> }]
    fn parse_if_expression(&mut self) -> ParseResult<IfExpressionNode> {
        assert_eq!(Token::If, self.get_next().unwrap());

        //if clause
        if (!self.expect_next(Token::Lparen)) {
            return Err(ParseError::Error(
                "`(` missing in `if` condition".to_string(),
            ));
        }
        self.get_next().unwrap();
        let condition = self.parse_expression(Precedence::Lowest)?;
        if (!self.expect_next(Token::Rparen)) {
            return Err(ParseError::Error(
                "`)` missing in `if` condition".to_string(),
            ));
        }
        self.get_next().unwrap();
        if (!self.expect_next(Token::Lbrace)) {
            return Err(ParseError::Error("`{` missing in `if` block".to_string()));
        }
        let if_value = self.parse_block_expression()?;

        //else clause
        let else_value = match self.expect_next(Token::Else) {
            false => None,
            true => {
                self.get_next().unwrap();
                match self.expect_next(Token::Lbrace) {
                    false => {
                        return Err(ParseError::Error("`{` missing in `else` block".to_string()))
                    }
                    true => Some(self.parse_block_expression()?),
                }
            }
        };

        Ok(IfExpressionNode::new(condition, if_value, else_value))
    }

    //fn (<parameter(s)>) { <statement(s)> }
    //
    //The last <argument> can optionally be followed by a comma (e.g. `(a, b,)`).
    //
    //Examples of parameters:
    // ()
    // (a)
    // (a, b)
    fn parse_function_literal(&mut self) -> ParseResult<FunctionLiteralNode> {
        assert_eq!(Token::Function, self.get_next().unwrap());
        if (!self.expect_next(Token::Lparen)) {
            return Err(ParseError::Error(
                "`(` missing in function parameter list".to_string(),
            ));
        }
        self.get_next().unwrap();
        let mut parameters = vec![];
        loop {
            match self.peek_next()? {
                Token::Rparen => {
                    self.get_next().unwrap();
                    break;
                }
                Token::Ident(_) => {
                    parameters.push(self.parse_identifier()?);
                    if (self.expect_next(Token::Comma)) {
                        self.get_next().unwrap();
                    }
                }
                t => {
                    return Err(ParseError::Error(format!(
                        "expected identifier but found `{:?}` in function parameter list",
                        t
                    )))
                }
            }
        }
        if (!self.expect_next(Token::Lbrace)) {
            return Err(ParseError::Error("function body missing".to_string()));
        }
        Ok(FunctionLiteralNode::new(
            parameters,
            self.parse_block_expression()?,
        ))
    }
}

/*-------------------------------------*/

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
            let token = lexer.get_next_token().unwrap();
            if (token == Token::Eof) {
                break;
            }
            v.push(token);
        }
        v.push(Token::Eof);
        v
    }

    fn assert_integer_literal(n: &dyn ExpressionNode, i: i64) {
        let n = n.as_any().downcast_ref::<IntegerLiteralNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.get_value(), i);
    }

    fn assert_float_literal(n: &dyn ExpressionNode, v: f64) {
        let n = n.as_any().downcast_ref::<FloatLiteralNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.get_value(), v);
    }

    fn assert_boolean_literal(n: &dyn ExpressionNode, b: bool) {
        let n = n.as_any().downcast_ref::<BooleanLiteralNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.get_value(), b);
    }

    fn assert_character_literal(n: &dyn ExpressionNode, c: char) {
        let n = n.as_any().downcast_ref::<CharacterLiteralNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.get_value(), c);
    }

    fn assert_string_literal(n: &dyn ExpressionNode, s: &str) {
        let n = n.as_any().downcast_ref::<StringLiteralNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.get_value(), s);
    }

    fn assert_array_literal(n: &dyn ExpressionNode, v: &Vec<i64>) {
        let n = n.as_any().downcast_ref::<ArrayLiteralNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(v.len(), n.elements().len());
        for i in 0..v.len() {
            let e = n.elements()[i]
                .as_any()
                .downcast_ref::<IntegerLiteralNode>();
            assert!(e.is_some());
            let e = e.unwrap();
            assert_eq!(v[i], e.get_value());
        }
    }

    fn assert_identifier(n: &dyn ExpressionNode, s: &str) {
        let n = n.as_any().downcast_ref::<IdentifierNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.get_name(), s);
    }

    fn assert_unary_expression(n: &dyn ExpressionNode, op: &Token, i: i64) {
        let n = n.as_any().downcast_ref::<UnaryExpressionNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.operator(), op);
        let e = n.expression().as_any().downcast_ref::<IntegerLiteralNode>();
        assert!(e.is_some());
        let e = e.unwrap();
        assert_eq!(e.get_value(), i);
    }

    fn assert_binary_expression(n: &dyn ExpressionNode, op: &Token, i: i64, j: i64) {
        let n = n.as_any().downcast_ref::<BinaryExpressionNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.operator(), op);

        let e = n.left().as_any().downcast_ref::<IntegerLiteralNode>();
        assert!(e.is_some());
        let e = e.unwrap();
        assert_eq!(e.get_value(), i);

        let e = n.right().as_any().downcast_ref::<IntegerLiteralNode>();
        assert!(e.is_some());
        let e = e.unwrap();
        assert_eq!(e.get_value(), j);
    }

    fn assert_binary_expression_2(n: &dyn ExpressionNode, op: &Token, s1: &str, s2: &str) {
        let n = n.as_any().downcast_ref::<BinaryExpressionNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_eq!(n.operator(), op);

        let e = n.left().as_any().downcast_ref::<IdentifierNode>();
        assert!(e.is_some());
        let e = e.unwrap();
        assert_eq!(e.get_name(), s1);

        let e = n.right().as_any().downcast_ref::<IdentifierNode>();
        assert!(e.is_some());
        let e = e.unwrap();
        assert_eq!(e.get_name(), s2);
    }

    #[test]
    // #[ignore]
    fn test01() {
        let input = r#"
            let x = 5;
            let ab = 10;
        "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_ok());
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
            assert_identifier(s.identifier(), names[i]);
            assert_integer_literal(s.expression(), values[i]);
        }
    }

    #[test]
    // #[ignore]
    fn test02() {
        let input = r#"
                    return 5;
                    return 10;
                "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_ok());
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
            assert!(s.expression().is_some());
            assert_integer_literal(s.expression().as_ref().unwrap().as_ref(), values[i]);
        }
    }

    #[test]
    // #[ignore]
    fn test03() {
        let input = r#"
                foo;;
                bar;
            "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_ok());
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
            assert_identifier(s.expression(), names[i]);
        }
    }

    #[test]
    // #[ignore]
    fn test04() {
        let input = r#"
                5;
                3.14;
            "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_ok());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(2, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_integer_literal(s.expression(), 5);

        let s = root.statements()[1]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_float_literal(s.expression(), 3.14);
    }

    #[test]
    // #[ignore]
    fn test05() {
        let input = r#"
                true;
                false;
            "#;

        let mut parser = Parser::new(get_tokens(input));

        let l = vec![true, false];

        let root = parser.parse();
        assert!(root.is_ok());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(2, root.statements().len());

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ExpressionStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_boolean_literal(s.expression(), l[i]);
        }
    }

    #[test]
    // #[ignore]
    fn test06() {
        let input = r#"
                'あ';
                "こんにちは";
            "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_ok());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(2, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_character_literal(s.expression(), 'あ');

        let s = root.statements()[1]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_string_literal(s.expression(), "こんにちは");
    }

    #[test]
    // #[ignore]
    fn test07() {
        let input = r#"
                    !5;
                    -15;
                "#;

        let operators = vec![Token::Invert, Token::Minus];
        let values = vec![5, 15];

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_ok());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(2, root.statements().len());

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ExpressionStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_unary_expression(s.expression(), &operators[i], values[i]);
        }
    }

    #[test]
    // #[ignore]
    fn test08() {
        let input = r#"
                    1 + 2;
                    1 - 2;
                    1 * 2;
                    1 / 2;
                    1 % 2;
                    1 ** 2;
                    1 > 2;
                    1 < 2;
                    1 >= 2;
                    1 <= 2;
                    1 == 2;
                    1 != 2;
                    1 && 2;
                    1 || 2;
                "#;

        let operators = vec![
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::Slash,
            Token::Percent,
            Token::Power,
            Token::Gt,
            Token::Lt,
            Token::GtEq,
            Token::LtEq,
            Token::Eq,
            Token::NotEq,
            Token::And,
            Token::Or,
        ];

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_ok());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(operators.len(), root.statements().len());

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ExpressionStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            assert_binary_expression(s.expression(), &operators[i], 1, 2);
        }
    }

    #[test]
    // #[ignore]
    fn test09() {
        let input = r#"
                    1 + 2 * 3;
                "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_ok());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(1, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
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
    // #[ignore]
    fn test10() {
        let input = r#"
                    (1 + 2) * 3;
                "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_ok());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(1, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
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
    // #[ignore]
    fn test11() {
        let input = r#"
                    if (x < y) { x }; 5;
                "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        assert!(root.is_ok());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(2, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();

        let v = s.expression().as_any().downcast_ref::<IfExpressionNode>();
        assert!(v.is_some());
        let v = v.unwrap();

        assert_binary_expression_2(v.condition(), &Token::Lt, "x", "y");

        assert!(v.else_value().is_none());

        let if_value = v.if_value();
        let l = if_value.statements();
        assert_eq!(1, l.len());
        let n = l[0].as_any().downcast_ref::<ExpressionStatementNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_identifier(n.expression(), "x");

        let s = root.statements()[1]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_integer_literal(s.expression(), 5);
    }

    #[test]
    // #[ignore]
    fn test12() {
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
        assert!(root.is_ok());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(1, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();

        let v = s.expression().as_any().downcast_ref::<IfExpressionNode>();
        assert!(v.is_some());
        let v = v.unwrap();

        assert_binary_expression_2(v.condition(), &Token::NotEq, "x", "y");

        let if_value = v.if_value();
        let l = if_value.statements();
        assert_eq!(1, l.len());
        let n = l[0].as_any().downcast_ref::<ExpressionStatementNode>();
        assert!(n.is_some());
        let n = n.unwrap();
        assert_identifier(n.expression(), "x");

        match v.else_value() {
            None => assert!(v.else_value().is_some()),
            Some(else_value) => {
                let l = else_value.statements();
                assert_eq!(2, l.len());

                let n = l[0].as_any().downcast_ref::<ExpressionStatementNode>();
                assert!(n.is_some());
                let n = n.unwrap();
                assert_integer_literal(n.expression(), 3);

                let n = l[1].as_any().downcast_ref::<ExpressionStatementNode>();
                assert!(n.is_some());
                let n = n.unwrap();
                assert_integer_literal(n.expression(), 4);
            }
        }
    }

    #[test]
    // #[ignore]
    fn test13() {
        let input = r#"
                    fn () {
                        return;
                    }
                    fn (x) {
                        return x + y;
                    }
                    fn (x, y,) {
                        return x + y;
                    }
                "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        println!("{:?}", root);
        assert!(root.is_ok());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(3, root.statements().len());

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ExpressionStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();

            let v = s
                .expression()
                .as_any()
                .downcast_ref::<FunctionLiteralNode>();
            assert!(v.is_some());
            let v = v.unwrap();

            assert_eq!(i, v.parameters().len());
            if (i >= 1) {
                assert_identifier(&v.parameters()[0], "x");
            }
            if (i == 2) {
                assert_identifier(&v.parameters()[1], "y");
            }

            let s = v.body();
            assert_eq!(1, s.statements().len());
            let s = v.body().statements()[0]
                .as_any()
                .downcast_ref::<ReturnStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();
            if (i == 0) {
                assert!(s.expression().is_none());
            } else {
                assert!(s.expression().is_some());
                assert_binary_expression_2(
                    s.expression().as_ref().unwrap().as_ref(),
                    &Token::Plus,
                    "x",
                    "y",
                );
            }
        }
    }

    #[test]
    // #[ignore]
    fn test14() {
        let input = r#"
            f()
            f(a)
            f(a, 3 + 4,)
                "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        println!("{:?}", root);
        assert!(root.is_ok());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(3, root.statements().len());

        for i in 0..root.statements().len() {
            let s = root.statements()[i]
                .as_any()
                .downcast_ref::<ExpressionStatementNode>();
            assert!(s.is_some());
            let s = s.unwrap();

            let v = s.expression().as_any().downcast_ref::<CallExpressionNode>();
            assert!(v.is_some());
            let v = v.unwrap();
            assert_identifier(v.function(), "f");

            assert_eq!(i, v.arguments().len());
            if (i >= 1) {
                assert_identifier(v.arguments()[0].as_ref(), "a");
            }
            if (i == 2) {
                assert_binary_expression(v.arguments()[1].as_ref(), &Token::Plus, 3, 4);
            }
        }
    }

    #[test]
    // #[ignore]
    fn test15() {
        let input = r#"
            [];
            [1];
            [1,2,];
            [1,][0];
            a[8];
                "#;

        let mut parser = Parser::new(get_tokens(input));

        let root = parser.parse();
        println!("{:?}", root);
        assert!(root.is_ok());
        let root = root.unwrap();
        println!("{:#?}", root);

        assert_eq!(5, root.statements().len());

        let s = root.statements()[0]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_array_literal(s.expression(), &vec![]);

        let s = root.statements()[1]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_array_literal(s.expression(), &vec![1]);

        let s = root.statements()[2]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_array_literal(s.expression(), &vec![1, 2]);

        let s = root.statements()[3]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s
            .unwrap()
            .expression()
            .as_any()
            .downcast_ref::<IndexExpressionNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_integer_literal(s.index(), 0);
        assert_array_literal(s.array(), &vec![1]);

        let s = root.statements()[4]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s
            .unwrap()
            .expression()
            .as_any()
            .downcast_ref::<IndexExpressionNode>();
        assert!(s.is_some());
        let s = s.unwrap();
        assert_integer_literal(s.index(), 8);
        assert_identifier(s.array(), "a");
    }
}
