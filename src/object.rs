use std::any::Any;
use std::fmt::{self, Display};
use std::rc::Rc;

use super::ast::*;
use super::environment::Environment;

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

pub struct Str {
    value: String,
}
impl Object for Str {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Str {
    pub fn new(value: String) -> Self {
        Self { value }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}
impl Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
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
    pub fn parameters(&self) -> &Vec<IdentifierNode> {
        &self.parameters
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
