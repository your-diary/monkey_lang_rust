use std::any::Any;
use std::fmt::{self, Display};
use std::rc::Rc;

use itertools::Itertools;

use super::ast::*;
use super::environment::Environment;
use super::evaluator::EvalResult;

/*-------------------------------------*/

pub trait Object: Display {
    fn as_any(&self) -> &dyn Any;
}

macro_rules! impl_object {
    ($t:ty) => {
        impl Object for $t {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
}

/*-------------------------------------*/

pub struct Null {}

impl_object!(Null);

impl Null {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for Null {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "null")
    }
}

/*-------------------------------------*/

pub struct Int {
    value: i64,
}

impl_object!(Int);

impl Int {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
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

impl_object!(Float);

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

impl_object!(Bool);

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

impl_object!(Char);

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
#[allow(clippy::len_without_is_empty)]
pub trait Indexable: Object {
    fn len(&self) -> usize;
}

/*-------------------------------------*/

#[derive(Clone)]
pub struct Str {
    value: Rc<String>,
    length: usize, //for performance of `Indexable`
}

impl_object!(Str);

impl Str {
    pub fn new(value: Rc<String>) -> Self {
        let length = value.chars().count();
        Self { value, length }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Indexable for Str {
    fn len(&self) -> usize {
        self.length
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

impl_object!(Array);

impl Array {
    pub fn new(elements: Vec<Rc<dyn Object>>) -> Self {
        Self { elements }
    }
    pub fn elements(&self) -> &Vec<Rc<dyn Object>> {
        &self.elements
    }
}

impl Indexable for Array {
    fn len(&self) -> usize {
        self.elements.len()
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.elements.iter().join(", "))
    }
}

/*-------------------------------------*/

pub struct ReturnValue {
    value: Rc<dyn Object>,
}

impl_object!(ReturnValue);

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

/*-------------------------------------*/

#[derive(Clone)]
pub struct Function {
    parameters: Rc<Vec<IdentifierNode>>,
    body: Rc<BlockExpressionNode>,
    env: Environment,
}

impl_object!(Function);

impl Function {
    pub fn new(
        parameters: Rc<Vec<IdentifierNode>>,
        body: Rc<BlockExpressionNode>,
        env: Environment,
    ) -> Self {
        Self {
            parameters,
            body,
            env,
        }
    }
    pub fn body(&self) -> &BlockExpressionNode {
        &self.body
    }
    pub fn env(&self) -> &Environment {
        &self.env
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

impl Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "function")
    }
}

/*-------------------------------------*/

#[derive(Clone)]
pub struct BuiltinFunction {
    parameters: Rc<Vec<IdentifierNode>>,
    f: Rc<dyn Fn(&Environment) -> EvalResult>,
}

impl_object!(BuiltinFunction);

impl BuiltinFunction {
    pub fn new(
        parameters: Rc<Vec<IdentifierNode>>,
        f: Rc<dyn Fn(&Environment) -> EvalResult>,
    ) -> Self {
        Self { parameters, f }
    }
    pub fn call(&self, env: &Environment) -> EvalResult {
        (self.f)(env)
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

impl Display for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "built-in function")
    }
}

/*-------------------------------------*/
