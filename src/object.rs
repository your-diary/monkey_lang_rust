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

pub struct ReturnValue {
    value: Box<dyn Object>,
}
impl Object for ReturnValue {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl ReturnValue {
    pub fn new(value: Box<dyn Object>) -> Self {
        Self { value }
    }
    pub fn value(&self) -> &dyn Object {
        self.value.as_ref()
    }
    //HACK: manual extraction
    //We couldn't find a way to move `value` of `ReturnValue` out of the result of `downcast_ref::<ReturnValue>`
    // in Rust.
    pub fn extract(&self) -> Box<dyn Object> {
        if let Some(e) = self.value.as_any().downcast_ref::<Integer>() {
            return Box::new(Integer::new(e.value())) as _;
        }
        if let Some(e) = self.value.as_any().downcast_ref::<Boolean>() {
            return Box::new(Boolean::new(e.value())) as _;
        }
        unimplemented!();
    }
}
impl Display for ReturnValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return ({});", self.value) //TODO
    }
}

/*-------------------------------------*/
