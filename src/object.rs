use std::any::Any;

/*-------------------------------------*/

pub trait Object {
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

/*-------------------------------------*/
