use std::collections::HashMap;

use super::object::Object;

//This struct is used as a function table, a variable table, etc.
pub struct Environment {
    m: HashMap<String, Box<dyn Object>>,
}

impl Environment {
    pub fn new() -> Self {
        Self { m: HashMap::new() }
    }
    pub fn get(&self, key: &str) -> Option<&dyn Object> {
        self.m.get(key).map(|e| e.as_ref())
    }
    pub fn set(&mut self, key: String, value: Box<dyn Object>) {
        self.m.insert(key, value);
    }
}
