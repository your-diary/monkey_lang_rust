use std::collections::HashMap;
use std::rc::Rc;

use super::object::Object;

//This struct is used as a function table, a variable table, etc.
#[derive(Clone)]
pub struct Environment {
    m: HashMap<String, Rc<dyn Object>>, //current scope (inner-most scope)
    outer: Option<Rc<Environment>>,     //enclosing scope (parent or outer scope)
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
            None => match &self.outer {
                None => None,
                Some(e) => e.get(key),
            },
        }
    }

    pub fn set(&mut self, key: String, value: Rc<dyn Object>) {
        self.m.insert(key, value);
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

    //We perform recursive calls to guarantee `outer` is added as the outer-most environment.
    //The performance is not optimized well as we have to call `Rc.as_ref().clone()` multiple times to extract value from `Rc`.
    pub fn set_outer(&mut self, outer: Option<Rc<Environment>>) {
        self.outer = match &self.outer {
            None => outer,
            Some(e) => {
                let mut e: Environment = e.as_ref().clone();
                e.set_outer(outer);
                Some(Rc::new(e))
            }
        }
    }

    fn to_debug_string(&self) -> String {
        format!(
            "Environment {{\n    m: {:?},\n    outer: {}\n}}",
            self.m.keys(),
            match self.outer {
                None => "None".to_string(),
                Some(ref e) => e.to_debug_string(),
            }
        )
    }
}
