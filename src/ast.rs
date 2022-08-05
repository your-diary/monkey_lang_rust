use std::any::Any;
use std::fmt::Debug;

use super::token::Token;

/*-------------------------------------*/

pub trait Node: Debug {
    fn as_any(&self) -> &dyn Any;
}

pub trait StatementNode: Node {}

pub trait ExpressionNode: Node {}

/*-------------------------------------*/

#[derive(Debug)]
pub struct RootNode {
    statements: Vec<Box<dyn StatementNode>>,
}

impl Node for RootNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl RootNode {
    pub fn new() -> Self {
        RootNode {
            statements: Vec::new(),
        }
    }
    pub fn statements(&self) -> &Vec<Box<dyn StatementNode>> {
        &self.statements
    }
    pub fn statements_mut(&mut self) -> &mut Vec<Box<dyn StatementNode>> {
        &mut self.statements
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct IdentifierNode {
    token: Token,
}

impl Node for IdentifierNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for IdentifierNode {}

impl IdentifierNode {
    pub fn new(token: Token) -> Self {
        IdentifierNode { token }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct UnaryExpressionNode {
    operator: Token,
    expression: Box<dyn ExpressionNode>,
}

impl Node for UnaryExpressionNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for UnaryExpressionNode {}

impl UnaryExpressionNode {
    pub fn new(operator: Token, expression: Box<dyn ExpressionNode>) -> Self {
        UnaryExpressionNode {
            operator,
            expression,
        }
    }
    pub fn operator(&self) -> &Token {
        &self.operator
    }
    pub fn expression(&self) -> &dyn ExpressionNode {
        self.expression.as_ref()
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct BinaryExpressionNode {
    operator: Token,
    left: Box<dyn ExpressionNode>,
    right: Box<dyn ExpressionNode>,
}

impl Node for BinaryExpressionNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for BinaryExpressionNode {}

impl BinaryExpressionNode {
    pub fn new(
        operator: Token,
        left: Box<dyn ExpressionNode>,
        right: Box<dyn ExpressionNode>,
    ) -> Self {
        BinaryExpressionNode {
            operator,
            left,
            right,
        }
    }
    pub fn operator(&self) -> &Token {
        &self.operator
    }
    pub fn left(&self) -> &dyn ExpressionNode {
        self.left.as_ref()
    }
    pub fn right(&self) -> &dyn ExpressionNode {
        self.right.as_ref()
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct IntegerLiteralNode {
    token: Token,
}

impl Node for IntegerLiteralNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for IntegerLiteralNode {}

impl IntegerLiteralNode {
    pub fn new(token: Token) -> Self {
        IntegerLiteralNode { token }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct LetStatementNode {
    token: Token,
    identifier: IdentifierNode,
    expression: Box<dyn ExpressionNode>,
}

impl Node for LetStatementNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl StatementNode for LetStatementNode {}

impl LetStatementNode {
    pub fn new(identifier: IdentifierNode, expression: Box<dyn ExpressionNode>) -> Self {
        LetStatementNode {
            token: Token::Let,
            identifier,
            expression,
        }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
    pub fn identifier(&self) -> &IdentifierNode {
        &self.identifier
    }
    pub fn expression(&self) -> &dyn ExpressionNode {
        self.expression.as_ref()
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct ReturnStatementNode {
    token: Token,
    expression: Box<dyn ExpressionNode>,
}

impl Node for ReturnStatementNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl StatementNode for ReturnStatementNode {}

impl ReturnStatementNode {
    pub fn new(expression: Box<dyn ExpressionNode>) -> Self {
        ReturnStatementNode {
            token: Token::Return,
            expression,
        }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
    pub fn expression(&self) -> &dyn ExpressionNode {
        self.expression.as_ref()
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct ExpressionStatementNode {
    token: Token, //first token of an expression
    expression: Box<dyn ExpressionNode>,
}

impl Node for ExpressionStatementNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl StatementNode for ExpressionStatementNode {}

impl ExpressionStatementNode {
    pub fn new(token: Token, expression: Box<dyn ExpressionNode>) -> Self {
        ExpressionStatementNode { token, expression }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
    pub fn expression(&self) -> &dyn ExpressionNode {
        self.expression.as_ref()
    }
}

/*-------------------------------------*/
