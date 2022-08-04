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

pub struct PrefixExpressionNode {
    operator: Token,
    expression: Box<dyn ExpressionNode>,
}

impl Node for PrefixExpressionNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for PrefixExpressionNode {}

impl PrefixExpressionNode {
    pub fn new(operator: Token, expression: Box<dyn ExpressionNode>) -> Self {
        PrefixExpressionNode {
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
