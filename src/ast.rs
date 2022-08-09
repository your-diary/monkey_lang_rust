use std::any::Any;
use std::fmt::Debug;
use std::rc::Rc;

use super::token::Token;

/*-------------------------------------*/

pub trait Node: Base + Debug {
    fn as_any(&self) -> &dyn Any;
}

pub trait StatementNode: Node {}

pub trait ExpressionNode: Node {}

//for upcasting
//ref: |https://stackoverflow.com/questions/28632968/why-doesnt-rust-support-trait-object-upcasting|
pub trait Base {
    fn as_node(&self) -> &dyn Node;
}
impl<T: Node> Base for T {
    fn as_node(&self) -> &dyn Node {
        self
    }
}

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

impl Default for RootNode {
    fn default() -> Self {
        Self::new()
    }
}

/*-------------------------------------*/

#[derive(Debug, Clone)]
pub struct BlockStatementNode {
    statements: Vec<Rc<dyn StatementNode>>,
}

impl Node for BlockStatementNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl StatementNode for BlockStatementNode {}

impl BlockStatementNode {
    pub fn new() -> Self {
        BlockStatementNode {
            statements: Vec::new(),
        }
    }
    pub fn statements(&self) -> &Vec<Rc<dyn StatementNode>> {
        &self.statements
    }
    pub fn statements_mut(&mut self) -> &mut Vec<Rc<dyn StatementNode>> {
        &mut self.statements
    }
}

impl Default for BlockStatementNode {
    fn default() -> Self {
        Self::new()
    }
}

/*-------------------------------------*/

#[derive(Debug, Clone)]
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
    pub fn get_name(&self) -> &str {
        match &self.token {
            Token::Ident(s) => s,
            _ => unreachable!(),
        }
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
pub struct CallExpressionNode {
    function: Box<dyn ExpressionNode>,
    arguments: Vec<Box<dyn ExpressionNode>>,
}

impl Node for CallExpressionNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for CallExpressionNode {}

impl CallExpressionNode {
    pub fn new(function: Box<dyn ExpressionNode>, arguments: Vec<Box<dyn ExpressionNode>>) -> Self {
        CallExpressionNode {
            function,
            arguments,
        }
    }
    pub fn function(&self) -> &dyn ExpressionNode {
        self.function.as_ref()
    }
    pub fn arguments(&self) -> &Vec<Box<dyn ExpressionNode>> {
        &self.arguments
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct IfExpressionNode {
    condition: Box<dyn ExpressionNode>,
    if_value: BlockStatementNode,
    else_value: Option<BlockStatementNode>,
}

impl Node for IfExpressionNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for IfExpressionNode {}

impl IfExpressionNode {
    pub fn new(
        condition: Box<dyn ExpressionNode>,
        if_value: BlockStatementNode,
        else_value: Option<BlockStatementNode>,
    ) -> Self {
        IfExpressionNode {
            condition,
            if_value,
            else_value,
        }
    }
    pub fn condition(&self) -> &dyn ExpressionNode {
        self.condition.as_ref()
    }
    pub fn if_value(&self) -> &BlockStatementNode {
        &self.if_value
    }
    pub fn else_value(&self) -> &Option<BlockStatementNode> {
        &self.else_value
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
    pub fn get_value(&self) -> i32 {
        match self.token {
            Token::Int(i) => i,
            _ => unreachable!(),
        }
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct BooleanLiteralNode {
    token: Token,
}

impl Node for BooleanLiteralNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for BooleanLiteralNode {}

impl BooleanLiteralNode {
    pub fn new(token: Token) -> Self {
        BooleanLiteralNode { token }
    }
    pub fn get_value(&self) -> bool {
        match self.token {
            Token::True => true,
            Token::False => false,
            _ => unreachable!(),
        }
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct FunctionLiteralNode {
    parameters: Vec<IdentifierNode>,
    body: BlockStatementNode,
}

impl Node for FunctionLiteralNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for FunctionLiteralNode {}

impl FunctionLiteralNode {
    pub fn new(parameters: Vec<IdentifierNode>, body: BlockStatementNode) -> Self {
        FunctionLiteralNode { parameters, body }
    }
    pub fn parameters(&self) -> &Vec<IdentifierNode> {
        &self.parameters
    }
    pub fn body(&self) -> &BlockStatementNode {
        &self.body
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct LetStatementNode {
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
            identifier,
            expression,
        }
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
        ReturnStatementNode { expression }
    }
    pub fn expression(&self) -> &dyn ExpressionNode {
        self.expression.as_ref()
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct ExpressionStatementNode {
    expression: Box<dyn ExpressionNode>,
}

impl Node for ExpressionStatementNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl StatementNode for ExpressionStatementNode {}

impl ExpressionStatementNode {
    pub fn new(expression: Box<dyn ExpressionNode>) -> Self {
        ExpressionStatementNode { expression }
    }
    pub fn expression(&self) -> &dyn ExpressionNode {
        self.expression.as_ref()
    }
}

/*-------------------------------------*/
