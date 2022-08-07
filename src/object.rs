use std::any::Any;
use std::fmt::{self, Display};

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
