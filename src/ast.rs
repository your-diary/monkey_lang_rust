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

macro_rules! impl_node {
    ($t:ty) => {
        impl Node for $t {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
}

macro_rules! impl_statement_node {
    ($t:ty) => {
        impl StatementNode for $t {}
    };
}

macro_rules! impl_expression_node {
    ($t:ty) => {
        impl ExpressionNode for $t {}
    };
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct RootNode {
    statements: Vec<Box<dyn StatementNode>>,
}

impl_node!(RootNode);

impl RootNode {
    pub fn new(statements: Vec<Box<dyn StatementNode>>) -> Self {
        RootNode { statements }
    }
    pub fn statements(&self) -> &Vec<Box<dyn StatementNode>> {
        &self.statements
    }
}

/*-------------------------------------*/

#[derive(Debug, Clone)]
pub struct BlockExpressionNode {
    statements: Vec<Rc<dyn StatementNode>>,
}

impl_node!(BlockExpressionNode);
impl_expression_node!(BlockExpressionNode);

impl BlockExpressionNode {
    pub fn new(statements: Vec<Rc<dyn StatementNode>>) -> Self {
        BlockExpressionNode { statements }
    }
    pub fn statements(&self) -> &Vec<Rc<dyn StatementNode>> {
        &self.statements
    }
}

/*-------------------------------------*/

#[derive(Debug, Clone)]
pub struct IdentifierNode {
    token: Token,
}

impl_node!(IdentifierNode);
impl_expression_node!(IdentifierNode);

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

impl_node!(UnaryExpressionNode);
impl_expression_node!(UnaryExpressionNode);

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

impl_node!(BinaryExpressionNode);
impl_expression_node!(BinaryExpressionNode);

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
pub struct IndexExpressionNode {
    array: Box<dyn ExpressionNode>,
    index: Box<dyn ExpressionNode>,
}

impl_node!(IndexExpressionNode);
impl_expression_node!(IndexExpressionNode);

impl IndexExpressionNode {
    pub fn new(array: Box<dyn ExpressionNode>, index: Box<dyn ExpressionNode>) -> Self {
        IndexExpressionNode { array, index }
    }
    pub fn array(&self) -> &dyn ExpressionNode {
        self.array.as_ref()
    }
    pub fn index(&self) -> &dyn ExpressionNode {
        self.index.as_ref()
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct CallExpressionNode {
    function: Box<dyn ExpressionNode>,
    arguments: Vec<Box<dyn ExpressionNode>>,
}

impl_node!(CallExpressionNode);
impl_expression_node!(CallExpressionNode);

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
    if_value: BlockExpressionNode,
    else_value: Option<BlockExpressionNode>,
}

impl_node!(IfExpressionNode);
impl_expression_node!(IfExpressionNode);

impl IfExpressionNode {
    pub fn new(
        condition: Box<dyn ExpressionNode>,
        if_value: BlockExpressionNode,
        else_value: Option<BlockExpressionNode>,
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
    pub fn if_value(&self) -> &BlockExpressionNode {
        &self.if_value
    }
    pub fn else_value(&self) -> &Option<BlockExpressionNode> {
        &self.else_value
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct IntegerLiteralNode {
    token: Token,
}

impl_node!(IntegerLiteralNode);
impl_expression_node!(IntegerLiteralNode);

impl IntegerLiteralNode {
    pub fn new(token: Token) -> Self {
        IntegerLiteralNode { token }
    }
    pub fn get_value(&self) -> i64 {
        match self.token {
            Token::Int(i) => i,
            _ => unreachable!(),
        }
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct FloatLiteralNode {
    token: Token,
}

impl_node!(FloatLiteralNode);
impl_expression_node!(FloatLiteralNode);

impl FloatLiteralNode {
    pub fn new(token: Token) -> Self {
        FloatLiteralNode { token }
    }
    pub fn get_value(&self) -> f64 {
        match self.token {
            Token::Float(i) => i,
            _ => unreachable!(),
        }
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct BooleanLiteralNode {
    token: Token,
}

impl_node!(BooleanLiteralNode);
impl_expression_node!(BooleanLiteralNode);

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
pub struct CharacterLiteralNode {
    token: Token,
}

impl_node!(CharacterLiteralNode);
impl_expression_node!(CharacterLiteralNode);

impl CharacterLiteralNode {
    pub fn new(token: Token) -> Self {
        CharacterLiteralNode { token }
    }
    pub fn get_value(&self) -> char {
        match self.token {
            Token::Char(c) => c,
            _ => unreachable!(),
        }
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct StringLiteralNode {
    token: Token,
}

impl_node!(StringLiteralNode);
impl_expression_node!(StringLiteralNode);

impl StringLiteralNode {
    pub fn new(token: Token) -> Self {
        StringLiteralNode { token }
    }
    pub fn get_value(&self) -> &str {
        match self.token {
            Token::String(ref s) => s,
            _ => unreachable!(),
        }
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct ArrayLiteralNode {
    elements: Vec<Box<dyn ExpressionNode>>,
}

impl_node!(ArrayLiteralNode);
impl_expression_node!(ArrayLiteralNode);

impl ArrayLiteralNode {
    pub fn new(elements: Vec<Box<dyn ExpressionNode>>) -> Self {
        ArrayLiteralNode { elements }
    }
    pub fn elements(&self) -> &Vec<Box<dyn ExpressionNode>> {
        &self.elements
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct FunctionLiteralNode {
    parameters: Vec<IdentifierNode>,
    body: BlockExpressionNode,
}

impl_node!(FunctionLiteralNode);
impl_expression_node!(FunctionLiteralNode);

impl FunctionLiteralNode {
    pub fn new(parameters: Vec<IdentifierNode>, body: BlockExpressionNode) -> Self {
        FunctionLiteralNode { parameters, body }
    }
    pub fn parameters(&self) -> &Vec<IdentifierNode> {
        &self.parameters
    }
    pub fn body(&self) -> &BlockExpressionNode {
        &self.body
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct LetStatementNode {
    identifier: IdentifierNode,
    expression: Box<dyn ExpressionNode>,
}

impl_node!(LetStatementNode);
impl_statement_node!(LetStatementNode);

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
    expression: Option<Box<dyn ExpressionNode>>,
}

impl_node!(ReturnStatementNode);
impl_statement_node!(ReturnStatementNode);

impl ReturnStatementNode {
    pub fn new(expression: Option<Box<dyn ExpressionNode>>) -> Self {
        ReturnStatementNode { expression }
    }
    pub fn expression(&self) -> &Option<Box<dyn ExpressionNode>> {
        &self.expression
    }
}

/*-------------------------------------*/

#[derive(Debug)]
pub struct ExpressionStatementNode {
    expression: Box<dyn ExpressionNode>,
}

impl_node!(ExpressionStatementNode);
impl_statement_node!(ExpressionStatementNode);

impl ExpressionStatementNode {
    pub fn new(expression: Box<dyn ExpressionNode>) -> Self {
        ExpressionStatementNode { expression }
    }
    pub fn expression(&self) -> &dyn ExpressionNode {
        self.expression.as_ref()
    }
}

/*-------------------------------------*/
