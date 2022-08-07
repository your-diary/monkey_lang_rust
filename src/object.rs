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

pub struct Integer {
    value: i32,
}
impl Object for Integer {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Integer {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
impl Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/*-------------------------------------*/

pub struct Boolean {
    value: bool,
}
impl Object for Boolean {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Boolean {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}
impl Display for Boolean {
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
        write!(f, "return") //TODO
    }
}

/*-------------------------------------*/

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
    //TODO Should we receive `env` or intead call `Environment::new()`?
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
}
impl Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "function") //TODO
    }
}

/*-------------------------------------*/
