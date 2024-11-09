use std::collections::VecDeque;
use std::fmt::{self, Display};
use std::mem;
use std::rc::Rc;

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

#[derive(Debug, PartialEq)]
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
            if self.tokens[0] == Token::Eof {
                break;
            }
            //empty statement
            if self.expect_next(Token::Semicolon) {
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
            if self.peek_next()? == &Token::Rbrace {
                self.get_next().unwrap();
                break;
            }
            statements.push(self.parse_statement()?);
        }
        Ok(BlockExpressionNode::new(statements))
    }

    //let <identifier> = <expression>;
    fn parse_let_statement(&mut self) -> ParseResult<LetStatementNode> {
        assert_eq!(Token::Let, self.get_next().unwrap());

        if !self.expect_next(Token::Ident(String::new())) {
            return Err(ParseError::Error(
                "identifier missing or reserved keyword used after `let`".to_string(),
            ));
        }
        let identifier = IdentifierNode::new(self.get_next()?);

        if !self.expect_next(Token::Assign) {
            return Err(ParseError::Error("`=` missing in `let`".to_string()));
        }
        self.get_next().unwrap();

        let expr = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_next(Token::Semicolon) {
            return Err(ParseError::Error("`;` missing in `let`".to_string()));
        }
        self.get_next().unwrap();

        Ok(LetStatementNode::new(identifier, expr))
    }

    //return [<expression>];
    fn parse_return_statement(&mut self) -> ParseResult<ReturnStatementNode> {
        assert_eq!(Token::Return, self.get_next().unwrap());
        if self.expect_next(Token::Semicolon) {
            self.get_next().unwrap();
            return Ok(ReturnStatementNode::new(None));
        }
        let expr = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_next(Token::Semicolon) {
            return Err(ParseError::Error("`;` missing in `return`".to_string()));
        }
        self.get_next().unwrap();
        Ok(ReturnStatementNode::new(Some(expr)))
    }

    //<expression>[;]
    fn parse_expression_statement(&mut self) -> ParseResult<ExpressionStatementNode> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.expect_next(Token::Semicolon) {
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
            if (next == &Token::Semicolon) || (precedence >= lookup_precedence(next)) {
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
        if !self.expect_next(Token::Rparen) {
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
                    match self.peek_next()? {
                        Token::Rbracket => {
                            self.get_next().unwrap();
                            break;
                        }
                        Token::Comma => {
                            self.get_next().unwrap();
                        }
                        _ => {
                            return Err(ParseError::Error(
                                "`,` expected but not found in array literal".to_string(),
                            ))
                        }
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
        if self.expect_next(Token::Rbracket) {
            return Err(ParseError::Error(
                "empty index in array index expression".to_string(),
            ));
        }
        let index = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_next(Token::Rbracket) {
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
                    match self.peek_next()? {
                        Token::Rparen => {
                            self.get_next().unwrap();
                            break;
                        }
                        Token::Comma => {
                            self.get_next().unwrap();
                        }
                        _ => {
                            return Err(ParseError::Error(
                                "`,` expected but not found in argument list".to_string(),
                            ))
                        }
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
        if !self.expect_next(Token::Lparen) {
            return Err(ParseError::Error(
                "`(` missing in `if` condition".to_string(),
            ));
        }
        self.get_next().unwrap();
        let condition = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_next(Token::Rparen) {
            return Err(ParseError::Error(
                "`)` missing in `if` condition".to_string(),
            ));
        }
        self.get_next().unwrap();
        if !self.expect_next(Token::Lbrace) {
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
        if !self.expect_next(Token::Lparen) {
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
                    match self.peek_next()? {
                        Token::Rparen => {
                            self.get_next().unwrap();
                            break;
                        }
                        Token::Comma => {
                            self.get_next().unwrap();
                        }
                        _ => {
                            return Err(ParseError::Error(
                                "`,` expected but not found in parameter list".to_string(),
                            ))
                        }
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
        if !self.expect_next(Token::Lbrace) {
            return Err(ParseError::Error("function body missing".to_string()));
        }
        Ok(FunctionLiteralNode::new(
            Rc::new(parameters),
            Rc::new(self.parse_block_expression()?),
        ))
    }
}

/*-------------------------------------*/

#[cfg(test)]
mod tests {

    use itertools::Itertools;

    use super::super::lexer::Lexer;
    use super::*;

    fn get_tokens(s: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(s);
        let mut v = vec![];
        loop {
            let token = lexer.get_next_token().unwrap();
            if token == Token::Eof {
                break;
            }
            v.push(token);
        }
        v.push(Token::Eof);
        v
    }

    fn test(input: &str, expected: &str) {
        let mut parser = Parser::new(get_tokens(input));
        let root = parser.parse();
        if root.is_err() {
            println!("{:#?}", root);
        }
        assert!(root.is_ok());
        let root = root.unwrap();
        println!("{:#?}", root);
        if expected.split_whitespace().join("")
            != format!("{:#?}", root).split_whitespace().join("")
        {
            assert_eq!(
                expected.split_whitespace().join(" "),
                format!("{:#?}", root).split_whitespace().join(" ")
            );
        }
    }

    fn test_error(input: &str, expected: &str) {
        let mut parser = Parser::new(get_tokens(input));
        let root = parser.parse();
        if root.is_ok() {
            println!("{:#?}", root.as_ref().unwrap());
        }
        assert!(root.is_err());
        match root {
            Ok(_) => unreachable!(),
            Err(e) => assert_eq!(e, ParseError::Error(expected.to_string())),
        }
    }

    #[test]
    // #[ignore]
    fn test_empty_input() {
        let input = r#""#;
        let expected = r#"
            RootNode {
                statements: [],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_single_statement() {
        let input = r#"
            3
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: IntegerLiteralNode {
                            token: Int(
                                3,
                            ),
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_two_statements() {
        let input = r#"
            3; ; 4
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: IntegerLiteralNode {
                            token: Int(
                                3,
                            ),
                        },
                    },
                    ExpressionStatementNode {
                        expression: IntegerLiteralNode {
                            token: Int(
                                4,
                            ),
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_error_propagation_01() {
        let input = r#"
            let
        "#;
        let expected = "identifier missing or reserved keyword used after `let`";
        test_error(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_error_propagation_02() {
        let input = r#"
            3 +
        "#;
        let expected = "unexpected eof in the middle of a statement";
        test_error(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_block_expression_01() {
        let input = r#"
            {} { 3 } { 3; 3 + 4; }
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: BlockExpressionNode {
                            statements: [],
                        },
                    },
                    ExpressionStatementNode {
                        expression: BlockExpressionNode {
                            statements: [
                                ExpressionStatementNode {
                                    expression: IntegerLiteralNode {
                                        token: Int(
                                            3,
                                        ),
                                    },
                                },
                            ],
                        },
                    },
                    ExpressionStatementNode {
                        expression: BlockExpressionNode {
                            statements: [
                                ExpressionStatementNode {
                                    expression: IntegerLiteralNode {
                                        token: Int(
                                            3,
                                        ),
                                    },
                                },
                                ExpressionStatementNode {
                                    expression: BinaryExpressionNode {
                                        operator: Plus,
                                        left: IntegerLiteralNode {
                                            token: Int(
                                                3,
                                            ),
                                        },
                                        right: IntegerLiteralNode {
                                            token: Int(
                                                4,
                                            ),
                                        },
                                    },
                                },
                            ],
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_let_statement_01() {
        let input = r#"
            let a = 1;
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    LetStatementNode {
                        identifier: IdentifierNode {
                            token: Ident(
                                "a",
                            ),
                        },
                        expression: IntegerLiteralNode {
                            token: Int(
                                1,
                            ),
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_let_statement_02() {
        let input = r#"
            let = 1;
        "#;
        let expected = "identifier missing or reserved keyword used after `let`";
        test_error(input, expected);

        let input = r#"
            let a * 1;
        "#;
        let expected = "`=` missing in `let`";
        test_error(input, expected);

        let input = r#"
            let a = ;
        "#;
        let expected = "unexpected start of expression: Semicolon";
        test_error(input, expected);

        let input = r#"
            let a = 3
        "#;
        let expected = "`;` missing in `let`";
        test_error(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_return_statement_01() {
        let input = r#"
            return;
            return 3;
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ReturnStatementNode {
                        expression: None,
                    },
                    ReturnStatementNode {
                        expression: Some(
                            IntegerLiteralNode {
                                token: Int(
                                    3,
                                ),
                            },
                        ),
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_return_statement_02() {
        let input = r#"
            return 3
        "#;
        let expected = "`;` missing in `return`";
        test_error(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_expression_statement_01() {
        let input = r#"
            3; 4
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: IntegerLiteralNode {
                            token: Int(
                                3,
                            ),
                        },
                    },
                    ExpressionStatementNode {
                        expression: IntegerLiteralNode {
                            token: Int(
                                4,
                            ),
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_expression_01() {
        let input = r#"
            1 + 2 * 3; 1 * 2 + 3;
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: BinaryExpressionNode {
                            operator: Plus,
                            left: IntegerLiteralNode {
                                token: Int(
                                    1,
                                ),
                            },
                            right: BinaryExpressionNode {
                                operator: Asterisk,
                                left: IntegerLiteralNode {
                                    token: Int(
                                        2,
                                    ),
                                },
                                right: IntegerLiteralNode {
                                    token: Int(
                                        3,
                                    ),
                                },
                            },
                        },
                    },
                    ExpressionStatementNode {
                        expression: BinaryExpressionNode {
                            operator: Plus,
                            left: BinaryExpressionNode {
                                operator: Asterisk,
                                left: IntegerLiteralNode {
                                    token: Int(
                                        1,
                                    ),
                                },
                                right: IntegerLiteralNode {
                                    token: Int(
                                        2,
                                    ),
                                },
                            },
                            right: IntegerLiteralNode {
                                token: Int(
                                    3,
                                ),
                            },
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_grouped_expression_01() {
        let input = r#"
            3 * (1 + 2)
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: BinaryExpressionNode {
                            operator: Asterisk,
                            left: IntegerLiteralNode {
                                token: Int(
                                    3,
                                ),
                            },
                            right: BinaryExpressionNode {
                                operator: Plus,
                                left: IntegerLiteralNode {
                                    token: Int(
                                        1,
                                    ),
                                },
                                right: IntegerLiteralNode {
                                    token: Int(
                                        2,
                                    ),
                                },
                            },
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_grouped_expression_02() {
        let input = r#"
            (3 + 4
        "#;
        let expected = "`)` missing in grouped expression";
        test_error(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_identifier_01() {
        let input = r#"
            a
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: IdentifierNode {
                            token: Ident(
                                "a",
                            ),
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_integer_literal_01() {
        let input = r#"
            -1; 0
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: UnaryExpressionNode {
                            operator: Minus,
                            expression: IntegerLiteralNode {
                                token: Int(
                                    1,
                                ),
                            },
                        },
                    },
                    ExpressionStatementNode {
                        expression: IntegerLiteralNode {
                            token: Int(
                                0,
                            ),
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_float_literal_01() {
        let input = r#"
            -1.; .0; 3.14
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: UnaryExpressionNode {
                            operator: Minus,
                            expression: FloatLiteralNode {
                                token: Float(
                                    1.0,
                                ),
                            },
                        },
                    },
                    ExpressionStatementNode {
                        expression: FloatLiteralNode {
                            token: Float(
                                0.0,
                            ),
                        },
                    },
                    ExpressionStatementNode {
                        expression: FloatLiteralNode {
                            token: Float(
                                3.14,
                            ),
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_boolean_literal_01() {
        let input = r#"
            true; false;
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: BooleanLiteralNode {
                            token: True,
                        },
                    },
                    ExpressionStatementNode {
                        expression: BooleanLiteralNode {
                            token: False,
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_character_literal_01() {
        let input = r#"
            'a' '\0'
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: CharacterLiteralNode {
                            token: Char(
                                'a',
                            ),
                        },
                    },
                    ExpressionStatementNode {
                        expression: CharacterLiteralNode {
                            token: Char(
                                '\0',
                            ),
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_string_literal_01() {
        let input = r#"
            "" "abc\ndef"
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: StringLiteralNode {
                            token: String(
                                "",
                            ),
                        },
                    },
                    ExpressionStatementNode {
                        expression: StringLiteralNode {
                            token: String(
                                "abc\ndef",
                            ),
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_array_literal_01() {
        let input = r#"
            []; [a]; [a,]; [a, b]; [a, b, c]
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: ArrayLiteralNode {
                            elements: [],
                        },
                    },
                    ExpressionStatementNode {
                        expression: ArrayLiteralNode {
                            elements: [
                                IdentifierNode {
                                    token: Ident(
                                        "a",
                                    ),
                                },
                            ],
                        },
                    },
                    ExpressionStatementNode {
                        expression: ArrayLiteralNode {
                            elements: [
                                IdentifierNode {
                                    token: Ident(
                                        "a",
                                    ),
                                },
                            ],
                        },
                    },
                    ExpressionStatementNode {
                        expression: ArrayLiteralNode {
                            elements: [
                                IdentifierNode {
                                    token: Ident(
                                        "a",
                                    ),
                                },
                                IdentifierNode {
                                    token: Ident(
                                        "b",
                                    ),
                                },
                            ],
                        },
                    },
                    ExpressionStatementNode {
                        expression: ArrayLiteralNode {
                            elements: [
                                IdentifierNode {
                                    token: Ident(
                                        "a",
                                    ),
                                },
                                IdentifierNode {
                                    token: Ident(
                                        "b",
                                    ),
                                },
                                IdentifierNode {
                                    token: Ident(
                                        "c",
                                    ),
                                },
                            ],
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_array_literal_02() {
        let input = r#"
            [1 2 3]
        "#;
        let expected = "`,` expected but not found in array literal";
        test_error(input, expected);

        let input = r#"
            [,]
        "#;
        let expected = "unexpected start of expression: Comma";
        test_error(input, expected);

        let input = r#"
            [a,,b]
        "#;
        let expected = "unexpected start of expression: Comma";
        test_error(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_unary_expression_01() {
        let input = r#"
            !3
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: UnaryExpressionNode {
                            operator: Invert,
                            expression: IntegerLiteralNode {
                                token: Int(
                                    3,
                                ),
                            },
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_binary_expression_01() {
        let input = r#"
            1 + 2
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: BinaryExpressionNode {
                            operator: Plus,
                            left: IntegerLiteralNode {
                                token: Int(
                                    1,
                                ),
                            },
                            right: IntegerLiteralNode {
                                token: Int(
                                    2,
                                ),
                            },
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_index_expression_01() {
        let input = r#"
            a[0]; [1, 2, 3][1]
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: IndexExpressionNode {
                            array: IdentifierNode {
                                token: Ident(
                                    "a",
                                ),
                            },
                            index: IntegerLiteralNode {
                                token: Int(
                                    0,
                                ),
                            },
                        },
                    },
                    ExpressionStatementNode {
                        expression: IndexExpressionNode {
                            array: ArrayLiteralNode {
                                elements: [
                                    IntegerLiteralNode {
                                        token: Int(
                                            1,
                                        ),
                                    },
                                    IntegerLiteralNode {
                                        token: Int(
                                            2,
                                        ),
                                    },
                                    IntegerLiteralNode {
                                        token: Int(
                                            3,
                                        ),
                                    },
                                ],
                            },
                            index: IntegerLiteralNode {
                                token: Int(
                                    1,
                                ),
                            },
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_index_expression_02() {
        let input = r#"
            a[]
        "#;
        let expected = "empty index in array index expression";
        test_error(input, expected);

        let input = r#"
            a[3 + 2
        "#;
        let expected = "`]` missing in array index expression";
        test_error(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_call_expression_01() {
        let input = r#"
            f(); f(a); f(a,); f(a, b); f(a, b, c)
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: CallExpressionNode {
                            function: IdentifierNode {
                                token: Ident(
                                    "f",
                                ),
                            },
                            arguments: [],
                        },
                    },
                    ExpressionStatementNode {
                        expression: CallExpressionNode {
                            function: IdentifierNode {
                                token: Ident(
                                    "f",
                                ),
                            },
                            arguments: [
                                IdentifierNode {
                                    token: Ident(
                                        "a",
                                    ),
                                },
                            ],
                        },
                    },
                    ExpressionStatementNode {
                        expression: CallExpressionNode {
                            function: IdentifierNode {
                                token: Ident(
                                    "f",
                                ),
                            },
                            arguments: [
                                IdentifierNode {
                                    token: Ident(
                                        "a",
                                    ),
                                },
                            ],
                        },
                    },
                    ExpressionStatementNode {
                        expression: CallExpressionNode {
                            function: IdentifierNode {
                                token: Ident(
                                    "f",
                                ),
                            },
                            arguments: [
                                IdentifierNode {
                                    token: Ident(
                                        "a",
                                    ),
                                },
                                IdentifierNode {
                                    token: Ident(
                                        "b",
                                    ),
                                },
                            ],
                        },
                    },
                    ExpressionStatementNode {
                        expression: CallExpressionNode {
                            function: IdentifierNode {
                                token: Ident(
                                    "f",
                                ),
                            },
                            arguments: [
                                IdentifierNode {
                                    token: Ident(
                                        "a",
                                    ),
                                },
                                IdentifierNode {
                                    token: Ident(
                                        "b",
                                    ),
                                },
                                IdentifierNode {
                                    token: Ident(
                                        "c",
                                    ),
                                },
                            ],
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_call_expression_02() {
        let input = r#"
            f(1 2 3)
        "#;
        let expected = "`,` expected but not found in argument list";
        test_error(input, expected);

        let input = r#"
            f(,)
        "#;
        let expected = "unexpected start of expression: Comma";
        test_error(input, expected);

        let input = r#"
            f(a,,b)
        "#;
        let expected = "unexpected start of expression: Comma";
        test_error(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_if_expression_01() {
        let input = r#"
            if (x) { y };
            if (x) { y; z; } else { w }
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: IfExpressionNode {
                            condition: IdentifierNode {
                                token: Ident(
                                    "x",
                                ),
                            },
                            if_value: BlockExpressionNode {
                                statements: [
                                    ExpressionStatementNode {
                                        expression: IdentifierNode {
                                            token: Ident(
                                                "y",
                                            ),
                                        },
                                    },
                                ],
                            },
                            else_value: None,
                        },
                    },
                    ExpressionStatementNode {
                        expression: IfExpressionNode {
                            condition: IdentifierNode {
                                token: Ident(
                                    "x",
                                ),
                            },
                            if_value: BlockExpressionNode {
                                statements: [
                                    ExpressionStatementNode {
                                        expression: IdentifierNode {
                                            token: Ident(
                                                "y",
                                            ),
                                        },
                                    },
                                    ExpressionStatementNode {
                                        expression: IdentifierNode {
                                            token: Ident(
                                                "z",
                                            ),
                                        },
                                    },
                                ],
                            },
                            else_value: Some(
                                BlockExpressionNode {
                                    statements: [
                                        ExpressionStatementNode {
                                            expression: IdentifierNode {
                                                token: Ident(
                                                    "w",
                                                ),
                                            },
                                        },
                                    ],
                                },
                            ),
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_if_expression_02() {
        let input = r#"
            if true { 3 }
        "#;
        let expected = "`(` missing in `if` condition";
        test_error(input, expected);

        let input = r#"
            if (true { 3 }
        "#;
        let expected = "`)` missing in `if` condition";
        test_error(input, expected);

        let input = r#"
            if (true) 3 }
        "#;
        let expected = "`{` missing in `if` block";
        test_error(input, expected);

        let input = r#"
            if (true) { 3
        "#;
        let expected = "unexpected eof in the middle of a statement";
        test_error(input, expected);

        let input = r#"
            if (true) { 3 } else 4
        "#;
        let expected = "`{` missing in `else` block";
        test_error(input, expected);

        let input = r#"
            if (true) { 3 } else { 4
        "#;
        let expected = "unexpected eof in the middle of a statement";
        test_error(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_function_literal_01() {
        let input = r#"
            fn() { }; fn(a) { 1 }; fn(a,) { 1; 2 }; fn(a, b) { 1; 2; }; fn(a, b, c) { }
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: FunctionLiteralNode {
                            parameters: [],
                            body: BlockExpressionNode {
                                statements: [],
                            },
                        },
                    },
                    ExpressionStatementNode {
                        expression: FunctionLiteralNode {
                            parameters: [
                                IdentifierNode {
                                    token: Ident(
                                        "a",
                                    ),
                                },
                            ],
                            body: BlockExpressionNode {
                                statements: [
                                    ExpressionStatementNode {
                                        expression: IntegerLiteralNode {
                                            token: Int(
                                                1,
                                            ),
                                        },
                                    },
                                ],
                            },
                        },
                    },
                    ExpressionStatementNode {
                        expression: FunctionLiteralNode {
                            parameters: [
                                IdentifierNode {
                                    token: Ident(
                                        "a",
                                    ),
                                },
                            ],
                            body: BlockExpressionNode {
                                statements: [
                                    ExpressionStatementNode {
                                        expression: IntegerLiteralNode {
                                            token: Int(
                                                1,
                                            ),
                                        },
                                    },
                                    ExpressionStatementNode {
                                        expression: IntegerLiteralNode {
                                            token: Int(
                                                2,
                                            ),
                                        },
                                    },
                                ],
                            },
                        },
                    },
                    ExpressionStatementNode {
                        expression: FunctionLiteralNode {
                            parameters: [
                                IdentifierNode {
                                    token: Ident(
                                        "a",
                                    ),
                                },
                                IdentifierNode {
                                    token: Ident(
                                        "b",
                                    ),
                                },
                            ],
                            body: BlockExpressionNode {
                                statements: [
                                    ExpressionStatementNode {
                                        expression: IntegerLiteralNode {
                                            token: Int(
                                                1,
                                            ),
                                        },
                                    },
                                    ExpressionStatementNode {
                                        expression: IntegerLiteralNode {
                                            token: Int(
                                                2,
                                            ),
                                        },
                                    },
                                ],
                            },
                        },
                    },
                    ExpressionStatementNode {
                        expression: FunctionLiteralNode {
                            parameters: [
                                IdentifierNode {
                                    token: Ident(
                                        "a",
                                    ),
                                },
                                IdentifierNode {
                                    token: Ident(
                                        "b",
                                    ),
                                },
                                IdentifierNode {
                                    token: Ident(
                                        "c",
                                    ),
                                },
                            ],
                            body: BlockExpressionNode {
                                statements: [],
                            },
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_function_literal_02() {
        let input = r#"
            fn (a b c) { 1 }
        "#;
        let expected = "`,` expected but not found in parameter list";
        test_error(input, expected);

        let input = r#"
            fn (,) { 1 }
        "#;
        let expected = "expected identifier but found `Comma` in function parameter list";
        test_error(input, expected);

        let input = r#"
            fn (a,,b) { 1 }
        "#;
        let expected = "expected identifier but found `Comma` in function parameter list";
        test_error(input, expected);

        let input = r#"
            fn (1, 2, 3) { 1 }
        "#;
        let expected = "expected identifier but found `Int(1)` in function parameter list";
        test_error(input, expected);

        let input = r#"
            fn a, b, c) { 1 }
        "#;
        let expected = "`(` missing in function parameter list";
        test_error(input, expected);

        let input = r#"
            fn (a, b, c { 1 }
        "#;
        let expected = "`,` expected but not found in parameter list";
        test_error(input, expected);

        let input = r#"
            fn (a, b, c) 1
        "#;
        let expected = "function body missing";
        test_error(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_precedence_01() {
        let input = r#"
            0 || 1 && 2 + -f() * 4 == 5;
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: BinaryExpressionNode {
                            operator: Or,
                            left: IntegerLiteralNode {
                                token: Int(
                                    0,
                                ),
                            },
                            right: BinaryExpressionNode {
                                operator: And,
                                left: IntegerLiteralNode {
                                    token: Int(
                                        1,
                                    ),
                                },
                                right: BinaryExpressionNode {
                                    operator: Eq,
                                    left: BinaryExpressionNode {
                                        operator: Plus,
                                        left: IntegerLiteralNode {
                                            token: Int(
                                                2,
                                            ),
                                        },
                                        right: BinaryExpressionNode {
                                            operator: Asterisk,
                                            left: UnaryExpressionNode {
                                                operator: Minus,
                                                expression: CallExpressionNode {
                                                    function: IdentifierNode {
                                                        token: Ident(
                                                            "f",
                                                        ),
                                                    },
                                                    arguments: [],
                                                },
                                            },
                                            right: IntegerLiteralNode {
                                                token: Int(
                                                    4,
                                                ),
                                            },
                                        },
                                    },
                                    right: IntegerLiteralNode {
                                        token: Int(
                                            5,
                                        ),
                                    },
                                },
                            },
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }

    #[test]
    // #[ignore]
    fn test_precedence_02() {
        let input = r#"
            0 || 1 && 2 - !a[3] % 4 < 5;
        "#;
        let expected = r#"
            RootNode {
                statements: [
                    ExpressionStatementNode {
                        expression: BinaryExpressionNode {
                            operator: Or,
                            left: IntegerLiteralNode {
                                token: Int(
                                    0,
                                ),
                            },
                            right: BinaryExpressionNode {
                                operator: And,
                                left: IntegerLiteralNode {
                                    token: Int(
                                        1,
                                    ),
                                },
                                right: BinaryExpressionNode {
                                    operator: Lt,
                                    left: BinaryExpressionNode {
                                        operator: Minus,
                                        left: IntegerLiteralNode {
                                            token: Int(
                                                2,
                                            ),
                                        },
                                        right: BinaryExpressionNode {
                                            operator: Percent,
                                            left: UnaryExpressionNode {
                                                operator: Invert,
                                                expression: IndexExpressionNode {
                                                    array: IdentifierNode {
                                                        token: Ident(
                                                            "a",
                                                        ),
                                                    },
                                                    index: IntegerLiteralNode {
                                                        token: Int(
                                                            3,
                                                        ),
                                                    },
                                                },
                                            },
                                            right: IntegerLiteralNode {
                                                token: Int(
                                                    4,
                                                ),
                                            },
                                        },
                                    },
                                    right: IntegerLiteralNode {
                                        token: Int(
                                            5,
                                        ),
                                    },
                                },
                            },
                        },
                    },
                ],
            }
        "#;
        test(input, expected);
    }
}
