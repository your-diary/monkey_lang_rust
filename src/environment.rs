use std::collections::HashMap;
use std::rc::Rc;

use super::object::Object;

//This struct is used as a function table, a variable table, etc.
pub struct Environment {
    m: HashMap<String, Rc<dyn Object>>,
}

impl Environment {
    pub fn new() -> Self {
        Self { m: HashMap::new() }
    }
    pub fn get(&self, key: &str) -> Option<&Rc<dyn Object>> {
        self.m.get(key)
    }
    pub fn try_set(&mut self, key: String, value: Rc<dyn Object>) -> Result<(), String> {
        match self.get(&key) {
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
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
