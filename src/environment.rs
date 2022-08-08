use std::collections::HashMap;
use std::rc::Rc;

use super::object::Object;

//This struct is used as a function table, a variable table, etc.
#[derive(Clone)]
pub struct Environment {
    m: HashMap<String, Rc<dyn Object>>,
    outer: Option<Rc<Environment>>, //enclosing scope (or parent scope)
}

impl Environment {
    pub fn new(outer: Option<Rc<Environment>>) -> Self {
        Self {
            m: HashMap::new(),
            outer,
        }
    }
    pub fn get(&self, key: &str) -> Option<&Rc<dyn Object>> {
        match self.m.get(key) {
            Some(e) => Some(e),
            None => match self.outer {
                None => None,
                Some(ref e) => e.get(key),
            },
        }
    }
    pub fn try_set(&mut self, key: String, value: Rc<dyn Object>) -> Result<(), String> {
        match self.m.get(&key) {
            None => {
                self.m.insert(key, value);
                Ok(())
            }
            Some(_) => Err(format!("`{}` is already defined", &key)),
        }
    }
    pub fn set(&mut self, key: String, value: Rc<dyn Object>) {
        self.m.insert(key, value);
    }
    pub fn set_outer(&mut self, outer: Option<Rc<Environment>>) {
        self.outer = outer;
    }
}
