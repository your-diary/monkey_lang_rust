use std::collections::HashMap;
use std::process;
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

    let print = BuiltinFunction::new(
        vec![IdentifierNode::new(Token::Ident("o".to_string()))],
        Rc::new(|env: &Environment| -> EvalResult {
            println!("{}", env.get("o").unwrap());
            Ok(Rc::new(Null::new()))
        }),
    );

    let eprint = BuiltinFunction::new(
        vec![IdentifierNode::new(Token::Ident("o".to_string()))],
        Rc::new(|env: &Environment| -> EvalResult {
            eprintln!("{}", env.get("o").unwrap());
            Ok(Rc::new(Null::new()))
        }),
    );

    /*-------------------------------------*/

    let exit = BuiltinFunction::new(
        vec![IdentifierNode::new(Token::Ident("i".to_string()))],
        Rc::new(|env: &Environment| -> EvalResult {
            let i = env.get("i").unwrap();
            if let Some(i) = i.as_any().downcast_ref::<Int>() {
                process::exit(i.value() as i32);
            }
            Err("argument type mismatch".to_string())
        }),
    );

    /*-------------------------------------*/

    let len = BuiltinFunction::new(
        vec![IdentifierNode::new(Token::Ident("l".to_string()))],
        Rc::new(|env: &Environment| -> EvalResult {
            let l = env.get("l").unwrap();
            if let Some(s) = l.as_any().downcast_ref::<Str>() {
                return Ok(Rc::new(Int::new(s.value().chars().count() as i64)));
            }
            if let Some(s) = l.as_any().downcast_ref::<Array>() {
                return Ok(Rc::new(Int::new(s.elements().len() as i64)));
            }
            Err("argument type mismatch".to_string())
        }),
    );

    /*-------------------------------------*/

    let append = BuiltinFunction::new(
        vec![
            IdentifierNode::new(Token::Ident("l".to_string())),
            IdentifierNode::new(Token::Ident("v".to_string())),
        ],
        Rc::new(|env: &Environment| -> EvalResult {
            let l = env.get("l").unwrap();
            if let Some(a) = l.as_any().downcast_ref::<Array>() {
                let mut elements = a.elements().clone();
                elements.push(env.get("v").cloned().unwrap());
                return Ok(Rc::new(Array::new(elements)));
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
            if let Some(v) = v.as_any().downcast_ref::<Array>() {
                return Ok(Rc::new(Bool::new(!v.elements().is_empty())));
            }
            Err("argument type mismatch".to_string())
        }),
    );

    let str_ = BuiltinFunction::new(
        vec![IdentifierNode::new(Token::Ident("v".to_string()))],
        Rc::new(|env: &Environment| -> EvalResult {
            let v = env.get("v").unwrap();
            if let Some(c) = v.as_any().downcast_ref::<Char>() {
                return Ok(Rc::new(Str::new(Rc::new(c.to_string()))));
            }
            Err("argument type mismatch".to_string())
        }),
    );

    let int_ = BuiltinFunction::new(
        vec![IdentifierNode::new(Token::Ident("v".to_string()))],
        Rc::new(|env: &Environment| -> EvalResult {
            let v = env.get("v").unwrap();
            if let Some(v) = v.as_any().downcast_ref::<Float>() {
                return Ok(Rc::new(Int::new(v.value() as i64)));
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

    m.insert("print".to_string(), Rc::new(print) as _);
    m.insert("eprint".to_string(), Rc::new(eprint) as _);
    m.insert("exit".to_string(), Rc::new(exit) as _);
    m.insert("len".to_string(), Rc::new(len) as _);
    m.insert("append".to_string(), Rc::new(append) as _);
    m.insert("bool".to_string(), Rc::new(bool_) as _);
    m.insert("str".to_string(), Rc::new(str_) as _);
    m.insert("int".to_string(), Rc::new(int_) as _);
    m.insert("float".to_string(), Rc::new(float_) as _);
    m.insert("pi".to_string(), Rc::new(pi) as _);

    Builtin { m }
}
