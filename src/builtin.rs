use std::collections::HashMap;
use std::rc::Rc;

use super::ast::IdentifierNode;
use super::environment::Environment;
use super::evaluator::EvalResult;
use super::object::*;
use super::token::Token;

pub struct Builtin {
    m: HashMap<String, Rc<dyn Object>>,
}

impl Builtin {
    pub fn new() -> Self {
        initialize_builtin()
    }
    pub fn lookup_builtin_identifier(&self, s: &str) -> Option<Rc<dyn Object>> {
        self.m.get(s).cloned()
    }
}

impl Default for Builtin {
    fn default() -> Self {
        Self::new()
    }
}

//Never embed this function in `Builtin::new()`; it'll increase the indent level by one to decrease readability.
fn initialize_builtin() -> Builtin {
    let mut m = HashMap::new();

    /*-------------------------------------*/

    let len = BuiltinFunction::new(
        vec![IdentifierNode::new(Token::Ident("l".to_string()))],
        Rc::new(|env: &Environment| -> EvalResult {
            let l = env.get("l").unwrap();
            if let Some(s) = l.as_any().downcast_ref::<Str>() {
                return Ok(Rc::new(Int::new(s.value().chars().count() as i32)));
            }
            Err("argument type mismatch".to_string())
        }),
    );

    /*-------------------------------------*/
    //cast functions

    let bool_ = BuiltinFunction::new(
        vec![IdentifierNode::new(Token::Ident("v".to_string()))],
        Rc::new(|env: &Environment| -> EvalResult {
            let v = env.get("v").unwrap();
            if let Some(v) = v.as_any().downcast_ref::<Int>() {
                return Ok(Rc::new(Bool::new(v.value() != 0)));
            }
            if let Some(v) = v.as_any().downcast_ref::<Float>() {
                return Ok(Rc::new(Bool::new(v.value() != 0.0)));
            }
            if let Some(v) = v.as_any().downcast_ref::<Str>() {
                return Ok(Rc::new(Bool::new(!v.value().is_empty())));
            }
            Err("argument type mismatch".to_string())
        }),
    );

    let str_ = BuiltinFunction::new(
        vec![IdentifierNode::new(Token::Ident("v".to_string()))],
        Rc::new(|env: &Environment| -> EvalResult {
            let v = env.get("v").unwrap();
            if let Some(c) = v.as_any().downcast_ref::<Char>() {
                return Ok(Rc::new(Str::new(c.to_string())));
            }
            Err("argument type mismatch".to_string())
        }),
    );

    let int_ = BuiltinFunction::new(
        vec![IdentifierNode::new(Token::Ident("v".to_string()))],
        Rc::new(|env: &Environment| -> EvalResult {
            let v = env.get("v").unwrap();
            if let Some(v) = v.as_any().downcast_ref::<Float>() {
                return Ok(Rc::new(Int::new(v.value() as i32)));
            }
            Err("argument type mismatch".to_string())
        }),
    );

    let float_ = BuiltinFunction::new(
        vec![IdentifierNode::new(Token::Ident("v".to_string()))],
        Rc::new(|env: &Environment| -> EvalResult {
            let v = env.get("v").unwrap();
            if let Some(v) = v.as_any().downcast_ref::<Int>() {
                return Ok(Rc::new(Float::new(v.value() as f64)));
            }
            Err("argument type mismatch".to_string())
        }),
    );

    /*-------------------------------------*/

    let pi = Float::new(std::f64::consts::PI);

    /*-------------------------------------*/

    m.insert("len".to_string(), Rc::new(len) as _);
    m.insert("bool".to_string(), Rc::new(bool_) as _);
    m.insert("str".to_string(), Rc::new(str_) as _);
    m.insert("int".to_string(), Rc::new(int_) as _);
    m.insert("float".to_string(), Rc::new(float_) as _);
    m.insert("pi".to_string(), Rc::new(pi) as _);

    Builtin { m }
}
