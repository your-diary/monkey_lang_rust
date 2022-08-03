use std::any::Any;

use super::token::Token;

/*-------------------------------------*/

pub trait Node {
    fn as_any(&self) -> &dyn Any;
}

pub trait StatementNode: Node {}

pub trait ExpressionNode: Node {}

/*-------------------------------------*/

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

pub struct LetStatementNode {
    token: Token,
    left: IdentifierNode,
    right: Box<dyn ExpressionNode>,
}

impl Node for LetStatementNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl StatementNode for LetStatementNode {}

impl LetStatementNode {
    pub fn new(left: IdentifierNode, right: Box<dyn ExpressionNode>) -> Self {
        LetStatementNode {
            token: Token::Let,
            left,
            right,
        }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
    pub fn left(&self) -> &IdentifierNode {
        &self.left
    }
    pub fn right(&self) -> &dyn ExpressionNode {
        self.right.as_ref()
    }
}

/*-------------------------------------*/

pub struct ReturnStatementNode {
    token: Token,
    value: Box<dyn ExpressionNode>,
}

impl Node for ReturnStatementNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl StatementNode for ReturnStatementNode {}

impl ReturnStatementNode {
    pub fn new(value: Box<dyn ExpressionNode>) -> Self {
        ReturnStatementNode {
            token: Token::Return,
            value,
        }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
    pub fn value(&self) -> &dyn ExpressionNode {
        self.value.as_ref()
    }
}

/*-------------------------------------*/

pub struct ExpressionStatementNode {
    token: Token, //first token of an expression
    value: Box<dyn ExpressionNode>,
}

impl Node for ExpressionStatementNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl StatementNode for ExpressionStatementNode {}

impl ExpressionStatementNode {
    pub fn new(token: Token, value: Box<dyn ExpressionNode>) -> Self {
        ExpressionStatementNode { token, value }
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
    pub fn value(&self) -> &dyn ExpressionNode {
        self.value.as_ref()
    }
}

/*-------------------------------------*/
