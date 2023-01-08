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

impl Node for BlockExpressionNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for BlockExpressionNode {}

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
pub struct IndexExpressionNode {
    array: Box<dyn ExpressionNode>,
    index: Box<dyn ExpressionNode>,
}

impl Node for IndexExpressionNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for IndexExpressionNode {}

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
    if_value: BlockExpressionNode,
    else_value: Option<BlockExpressionNode>,
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

impl Node for FloatLiteralNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for FloatLiteralNode {}

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
pub struct CharacterLiteralNode {
    token: Token,
}

impl Node for CharacterLiteralNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for CharacterLiteralNode {}

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

impl Node for StringLiteralNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for StringLiteralNode {}

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

impl Node for ArrayLiteralNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for ArrayLiteralNode {}

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

impl Node for FunctionLiteralNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExpressionNode for FunctionLiteralNode {}

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
    expression: Option<Box<dyn ExpressionNode>>,
}

impl Node for ReturnStatementNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl StatementNode for ReturnStatementNode {}

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
