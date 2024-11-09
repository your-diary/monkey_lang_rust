use std::rc::Rc;

use super::evaluator::EvalResult;
use super::object::*;

pub fn unary_minus(o: &dyn Object) -> EvalResult {
    if let Some(o) = o.as_any().downcast_ref::<Int>() {
        return Ok(Rc::new(Int::new(-o.value())));
    }
    if let Some(o) = o.as_any().downcast_ref::<Float>() {
        return Ok(Rc::new(Float::new(-o.value())));
    }
    Err("operand of unary `-` is not a number".to_string())
}

pub fn unary_invert(o: &dyn Object) -> EvalResult {
    if let Some(o) = o.as_any().downcast_ref::<Bool>() {
        return Ok(Rc::new(Bool::new(!o.value())));
    }
    Err("operand of unary `!` is not a boolean".to_string())
}

fn try_cast<'a, T1: Object + 'static, T2: Object + 'static>(
    left: &'a dyn Object,
    right: &'a dyn Object,
) -> Option<(&'a T1, &'a T2)> {
    if let Some(left) = left.as_any().downcast_ref::<T1>() {
        if let Some(right) = right.as_any().downcast_ref::<T2>() {
            return Some((left, right));
        }
    }
    None
}

pub fn binary_plus(left: &dyn Object, right: &dyn Object) -> EvalResult {
    if let Some(t) = try_cast::<Int, Int>(left, right) {
        return Ok(Rc::new(Int::new(t.0.value() + t.1.value())));
    }
    if let Some(t) = try_cast::<Float, Float>(left, right) {
        return Ok(Rc::new(Float::new(t.0.value() + t.1.value())));
    }
    if let Some(t) = try_cast::<Str, Str>(left, right) {
        return Ok(Rc::new(Str::new(Rc::new(format!(
            "{}{}",
            t.0.value(),
            t.1.value()
        )))));
    }
    if let Some(t) = try_cast::<Array, Array>(left, right) {
        let mut elements = t.0.elements().clone();
        for i in 0..t.1.elements().len() {
            elements.push(t.1.elements()[i].clone());
        }
        return Ok(Rc::new(Array::new(elements)));
    }
    Err("operand of binary `+` is not a number, a string nor an array".to_string())
}

pub fn binary_minus(left: &dyn Object, right: &dyn Object) -> EvalResult {
    if let Some(t) = try_cast::<Int, Int>(left, right) {
        return Ok(Rc::new(Int::new(t.0.value() - t.1.value())));
    }
    if let Some(t) = try_cast::<Float, Float>(left, right) {
        return Ok(Rc::new(Float::new(t.0.value() - t.1.value())));
    }
    Err("operand of binary `-` is not a number".to_string())
}

pub fn binary_asterisk(left: &dyn Object, right: &dyn Object) -> EvalResult {
    if let Some(t) = try_cast::<Int, Int>(left, right) {
        return Ok(Rc::new(Int::new(t.0.value() * t.1.value())));
    }
    if let Some(t) = try_cast::<Float, Float>(left, right) {
        return Ok(Rc::new(Float::new(t.0.value() * t.1.value())));
    }
    Err("operand of binary `*` is not a number".to_string())
}

pub fn binary_slash(left: &dyn Object, right: &dyn Object) -> EvalResult {
    if let Some(t) = try_cast::<Int, Int>(left, right) {
        if t.0.value() == 0 {
            return Err("zero division".to_string());
        }
        return Ok(Rc::new(Int::new(t.0.value() / t.1.value())));
    }
    if let Some(t) = try_cast::<Float, Float>(left, right) {
        if t.1.value() == 0.0 {
            return Err("zero division".to_string());
        }
        return Ok(Rc::new(Float::new(t.0.value() / t.1.value())));
    }
    Err("operand of binary `/` is not a number".to_string())
}

pub fn binary_percent(left: &dyn Object, right: &dyn Object) -> EvalResult {
    if let Some(t) = try_cast::<Int, Int>(left, right) {
        if t.1.value() == 0 {
            return Err("zero division in `%`".to_string());
        }
        return Ok(Rc::new(Int::new(t.0.value() % t.1.value())));
    }
    if let Some(t) = try_cast::<Float, Float>(left, right) {
        if t.1.value() == 0.0 {
            return Err("zero division in `%`".to_string());
        }
        return Ok(Rc::new(Float::new(t.0.value() % t.1.value())));
    }
    Err("operand of binary `%` is not a number".to_string())
}

pub fn binary_power(left: &dyn Object, right: &dyn Object) -> EvalResult {
    if let Some(t) = try_cast::<Int, Int>(left, right) {
        if t.1.value() < 0 {
            return Err("negative exponent in <int>**<int> operation".to_string());
        }
        return Ok(Rc::new(Int::new(t.0.value().pow(t.1.value() as u32))));
    }
    if let Some(t) = try_cast::<Float, Float>(left, right) {
        return Ok(Rc::new(Float::new(t.0.value().powf(t.1.value()))));
    }
    Err("operand of binary `**` is not a number".to_string())
}

pub fn binary_eq(left: &dyn Object, right: &dyn Object) -> EvalResult {
    if let Some(t) = try_cast::<Int, Int>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() == t.1.value())));
    }
    if let Some(t) = try_cast::<Float, Float>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() == t.1.value())));
    }
    if let Some(t) = try_cast::<Bool, Bool>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() == t.1.value())));
    }
    if let Some(t) = try_cast::<Char, Char>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() == t.1.value())));
    }
    if let Some(t) = try_cast::<Str, Str>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() == t.1.value())));
    }
    Err("unsupported operand type for binary `==`".to_string())
}

pub fn binary_noteq(left: &dyn Object, right: &dyn Object) -> EvalResult {
    if let Some(t) = try_cast::<Int, Int>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() != t.1.value())));
    }
    if let Some(t) = try_cast::<Float, Float>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() != t.1.value())));
    }
    if let Some(t) = try_cast::<Bool, Bool>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() != t.1.value())));
    }
    if let Some(t) = try_cast::<Char, Char>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() != t.1.value())));
    }
    if let Some(t) = try_cast::<Str, Str>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() != t.1.value())));
    }
    Err("unsupported operand type for binary `!=`".to_string())
}

pub fn binary_lt(left: &dyn Object, right: &dyn Object) -> EvalResult {
    if let Some(t) = try_cast::<Int, Int>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() < t.1.value())));
    }
    if let Some(t) = try_cast::<Float, Float>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() < t.1.value())));
    }
    if let Some(t) = try_cast::<Char, Char>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() < t.1.value())));
    }
    if let Some(t) = try_cast::<Str, Str>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() < t.1.value())));
    }
    Err("unsupported operand type for binary `<`".to_string())
}

pub fn binary_gt(left: &dyn Object, right: &dyn Object) -> EvalResult {
    if let Some(t) = try_cast::<Int, Int>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() > t.1.value())));
    }
    if let Some(t) = try_cast::<Float, Float>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() > t.1.value())));
    }
    if let Some(t) = try_cast::<Char, Char>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() > t.1.value())));
    }
    if let Some(t) = try_cast::<Str, Str>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() > t.1.value())));
    }
    Err("unsupported operand type for binary `>`".to_string())
}

pub fn binary_lteq(left: &dyn Object, right: &dyn Object) -> EvalResult {
    if let Some(t) = try_cast::<Int, Int>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() <= t.1.value())));
    }
    if let Some(t) = try_cast::<Float, Float>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() <= t.1.value())));
    }
    if let Some(t) = try_cast::<Char, Char>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() <= t.1.value())));
    }
    if let Some(t) = try_cast::<Str, Str>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() <= t.1.value())));
    }
    Err("unsupported operand type for binary `<=`".to_string())
}

pub fn binary_gteq(left: &dyn Object, right: &dyn Object) -> EvalResult {
    if let Some(t) = try_cast::<Int, Int>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() >= t.1.value())));
    }
    if let Some(t) = try_cast::<Float, Float>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() >= t.1.value())));
    }
    if let Some(t) = try_cast::<Char, Char>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() >= t.1.value())));
    }
    if let Some(t) = try_cast::<Str, Str>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() >= t.1.value())));
    }
    Err("unsupported operand type for binary `>=`".to_string())
}

pub fn binary_and(left: &dyn Object, right: &dyn Object) -> EvalResult {
    if let Some(t) = try_cast::<Bool, Bool>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() && t.1.value())));
    }
    Err("operand of binary `&&` is not a boolean".to_string())
}

pub fn binary_or(left: &dyn Object, right: &dyn Object) -> EvalResult {
    if let Some(t) = try_cast::<Bool, Bool>(left, right) {
        return Ok(Rc::new(Bool::new(t.0.value() || t.1.value())));
    }
    Err("operand of binary `|| is not a boolean".to_string())
}
