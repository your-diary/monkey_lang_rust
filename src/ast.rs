// use std::any::Any;
// 
// use super::token::Token;
// 
// /*-------------------------------------*/
// 
// pub trait Node {
//     fn get_literal(&self) -> Option<String>;
//     fn as_any(&self) -> &dyn Any;
// }
// 
// pub trait Statement: Node {}
// 
// pub trait Expression: Node {}
// 
// /*-------------------------------------*/
// 
// pub struct Root {
//     statements: Vec<Box<dyn Statement>>,
// }
// 
// impl Node for Root {
//     fn get_literal(&self) -> Option<String> {
//         if (self.statements.is_empty()) {
//             self.statements[0].get_literal()
//         } else {
//             None
//         }
//     }
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }
// 
// impl Root {
//     pub fn new() -> Self {
//         Root {
//             statements: Vec::new(),
//         }
//     }
//     pub fn statements(&self) -> &Vec<Box<dyn Statement>> {
//         &self.statements
//     }
//     pub fn statements_mut(&mut self) -> &mut Vec<Box<dyn Statement>> {
//         &mut self.statements
//     }
// }
// 
// /*-------------------------------------*/
// 
// pub struct Identifier {
//     token: Token,
//     value: String,
// }
// 
// impl Node for Identifier {
//     fn get_literal(&self) -> Option<String> {
//         self.token.literal().clone()
//     }
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }
// 
// impl Expression for Identifier {}
// 
// impl Identifier {
//     pub fn new(token: Token, value: String) -> Self {
//         Identifier { token, value }
//     }
//     pub fn token(&self) -> &Token {
//         &self.token
//     }
//     pub fn value(&self) -> &str {
//         &self.value
//     }
// }
// 
// /*-------------------------------------*/
// 
// pub struct LetStatement {
//     token: Token,
//     left: Identifier,
//     right: Box<dyn Expression>,
// }
// 
// impl Node for LetStatement {
//     fn get_literal(&self) -> Option<String> {
//         self.token.literal().clone()
//     }
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }
// 
// impl Statement for LetStatement {}
// 
// impl LetStatement {
//     pub fn new(left: Identifier, right: Box<dyn Expression>) -> Self {
//         LetStatement {
//             token: Token::new(TokenType::Let, None),
//             left,
//             right,
//         }
//     }
//     pub fn token(&self) -> &Token {
//         &self.token
//     }
//     pub fn left(&self) -> &Identifier {
//         &self.left
//     }
//     pub fn right(&self) -> &dyn Expression {
//         self.right.as_ref()
//     }
// }
// 
// /*-------------------------------------*/
// 
// pub struct ReturnStatement {
//     token: Token,
//     value: Box<dyn Expression>,
// }
// 
// impl Node for ReturnStatement {
//     fn get_literal(&self) -> Option<String> {
//         self.token.literal().clone()
//     }
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }
// 
// impl Statement for ReturnStatement {}
// 
// impl ReturnStatement {
//     pub fn new(value: Box<dyn Expression>) -> Self {
//         ReturnStatement {
//             token: Token::new(TokenType::Return, None),
//             value,
//         }
//     }
//     pub fn token(&self) -> &Token {
//         &self.token
//     }
//     pub fn value(&self) -> &dyn Expression {
//         self.value.as_ref()
//     }
// }
// 
// /*-------------------------------------*/
// 
// pub struct ExpressionStatement {
//     token: Token, //first token of an expression
//     value: Box<dyn Expression>,
// }
// 
// impl Node for ExpressionStatement {
//     fn get_literal(&self) -> Option<String> {
//         self.token.literal().clone()
//     }
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }
// 
// impl Statement for ExpressionStatement {}
// 
// impl ExpressionStatement {
//     pub fn new(token: Token, value: Box<dyn Expression>) -> Self {
//         ExpressionStatement { token, value }
//     }
//     pub fn token(&self) -> &Token {
//         &self.token
//     }
//     pub fn value(&self) -> &dyn Expression {
//         self.value.as_ref()
//     }
// }
// 
// /*-------------------------------------*/
