use std::any::Any;
use std::fmt::{self, Display};
use std::rc::Rc;

use super::ast::*;
use super::environment::Environment;
use super::evaluator::EvalResult;

/*-------------------------------------*/

pub trait Object: Display {
    fn as_any(&self) -> &dyn Any;
}

/*-------------------------------------*/

pub struct Null {}
impl Object for Null {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Null {
    pub fn new() -> Self {
        Self {}
    }
}
impl Display for Null {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "null")
    }
}
impl Default for Null {
    fn default() -> Self {
        Self::new()
    }
}

/*-------------------------------------*/

pub struct Int {
    value: i32,
}
impl Object for Int {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Int {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
impl Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/*-------------------------------------*/

pub struct Float {
    value: f64,
}
impl Object for Float {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Float {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
}
impl Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/*-------------------------------------*/

pub struct Bool {
    value: bool,
}
impl Object for Bool {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Bool {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}
impl Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/*-------------------------------------*/

pub struct Char {
    value: char,
}
impl Object for Char {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Char {
    pub fn new(value: char) -> Self {
        Self { value }
    }
    pub fn value(&self) -> char {
        self.value
    }
}
impl Display for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/*-------------------------------------*/

//implemented by `Str` and `Array`
pub trait Indexable: Object {
    fn num_element(&self) -> usize;
}

#[derive(Clone)]
pub struct Str {
    value: Rc<String>,
    length: usize, //for performance of `Indexable`
}
impl Object for Str {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Indexable for Str {
    fn num_element(&self) -> usize {
        self.length
    }
}
impl Str {
    pub fn new(value: Rc<String>) -> Self {
        let length = value.chars().count();
        Self { value, length }
    }
    pub fn value(&self) -> &Rc<String> {
        &self.value
    }
}
impl Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/*-------------------------------------*/

#[derive(Clone)]
pub struct Array {
    elements: Vec<Rc<dyn Object>>,
}
impl Object for Array {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Indexable for Array {
    fn num_element(&self) -> usize {
        self.elements.len()
    }
}
impl Array {
    pub fn new(elements: Vec<Rc<dyn Object>>) -> Self {
        Self { elements }
    }
    pub fn elements(&self) -> &Vec<Rc<dyn Object>> {
        &self.elements
    }
}
impl Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.elements
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

/*-------------------------------------*/

pub struct ReturnValue {
    value: Rc<dyn Object>,
}
impl Object for ReturnValue {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ReturnValue {
    pub fn new(value: Rc<dyn Object>) -> Self {
        Self { value }
    }
    pub fn value(&self) -> &Rc<dyn Object> {
        &self.value
    }
}
impl Display for ReturnValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return")
    }
}

/*-------------------------------------*/

//implemented by `Function` and `BuiltinFunction`
pub trait FunctionBase: Object {
    fn num_parameter(&self) -> usize;
    fn parameters(&self) -> &Vec<IdentifierNode>;
}

#[derive(Clone)]
pub struct Function {
    parameters: Vec<IdentifierNode>,
    body: BlockStatementNode,
    env: Environment,
}
impl Object for Function {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl FunctionBase for Function {
    fn num_parameter(&self) -> usize {
        self.parameters.len()
    }
    fn parameters(&self) -> &Vec<IdentifierNode> {
        &self.parameters
    }
}
impl Function {
    pub fn new(
        parameters: Vec<IdentifierNode>,
        body: BlockStatementNode,
        env: Environment,
    ) -> Self {
        Self {
            parameters,
            body,
            env,
        }
    }
    pub fn body(&self) -> &BlockStatementNode {
        &self.body
    }
    pub fn env(&self) -> &Environment {
        &self.env
    }
}
impl Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "function")
    }
}

/*-------------------------------------*/

#[derive(Clone)]
pub struct BuiltinFunction {
    parameters: Vec<IdentifierNode>,
    f: Rc<dyn Fn(&Environment) -> EvalResult>,
}
impl Object for BuiltinFunction {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl FunctionBase for BuiltinFunction {
    fn num_parameter(&self) -> usize {
        self.parameters.len()
    }
    fn parameters(&self) -> &Vec<IdentifierNode> {
        &self.parameters
    }
}
impl BuiltinFunction {
    pub fn new(parameters: Vec<IdentifierNode>, f: Rc<dyn Fn(&Environment) -> EvalResult>) -> Self {
        Self { parameters, f }
    }
    pub fn call(&self, env: &Environment) -> EvalResult {
        (self.f)(env)
    }
}
impl Display for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "built-in function")
    }
}

/*-------------------------------------*/
