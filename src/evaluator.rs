use std::rc::Rc;

use super::ast::*;
use super::environment::Environment;
use super::object::*;
use super::operator;
use super::token::Token;

pub type EvalResult = Result<Rc<dyn Object>, String>;

pub fn eval(node: &dyn Node, env: &mut Environment) -> EvalResult {
    if let Some(n) = node.as_any().downcast_ref::<RootNode>() {
        return eval_root_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<BlockStatementNode>() {
        return eval_block_statement_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<LetStatementNode>() {
        return eval_let_statement_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<ReturnStatementNode>() {
        return eval_return_statement_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<ExpressionStatementNode>() {
        return eval_expression_statement_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<UnaryExpressionNode>() {
        return eval_unary_expression_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<BinaryExpressionNode>() {
        return eval_binary_expression_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<CallExpressionNode>() {
        return eval_call_expression_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<IfExpressionNode>() {
        return eval_if_expression_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<IntegerLiteralNode>() {
        return eval_integer_literal_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<FloatLiteralNode>() {
        return eval_float_literal_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<BooleanLiteralNode>() {
        return eval_boolean_literal_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<CharacterLiteralNode>() {
        return eval_character_literal_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<StringLiteralNode>() {
        return eval_string_literal_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<FunctionLiteralNode>() {
        return eval_function_literal_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<IdentifierNode>() {
        return eval_identifier_node(n, env);
    }

    Err("not yet implemented or a bug of interpreter".to_string())
}

fn eval_root_node(n: &RootNode, env: &mut Environment) -> EvalResult {
    let mut ret = Rc::new(Null::new()) as _;
    for statement in n.statements() {
        ret = eval(statement.as_node(), env)?;
        //early return at the first `return` statement
        //Note the returned value is the content of `ReturnValue`; not the `ReturnValue` itself.
        if let Some(e) = ret.as_any().downcast_ref::<ReturnValue>() {
            return Ok(e.value().clone());
        }
    }
    Ok(ret)
}

//similar to `eval_root_node()` but returns `ReturnValue` itself when early return
//This difference is important to properly handle the following statement:
// if (true) {
//     if (true) {
//         return a;
//     }
//     return b;
// }
//If we shared the implementations of `eval_root_node()` and `eval_block_statement_node()`, then the result
// would be `b` rather than `a` as the statement above is evaluated as
// if (true) {
//     a;
//     return b;
// }
fn eval_block_statement_node(n: &BlockStatementNode, env: &Environment) -> EvalResult {
    let mut block_env = Environment::new(Some(Rc::new(env.clone())));
    let mut ret = Rc::new(Null::new()) as _;
    for statement in n.statements() {
        ret = eval(statement.as_node(), &mut block_env)?;
        if ret.as_any().downcast_ref::<ReturnValue>().is_some() {
            break;
        }
    }
    Ok(ret)
}

fn eval_let_statement_node(n: &LetStatementNode, env: &mut Environment) -> EvalResult {
    let o = eval(n.expression().as_node(), env)?;
    env.try_set(n.identifier().get_name().to_string(), o)?;
    Ok(Rc::new(Null::new()))
}

fn eval_return_statement_node(n: &ReturnStatementNode, env: &mut Environment) -> EvalResult {
    Ok(Rc::new(ReturnValue::new(match n.expression() {
        None => Rc::new(Null::new()),
        Some(e) => eval(e.as_node(), env)?,
    })))
}

fn eval_expression_statement_node(
    n: &ExpressionStatementNode,
    env: &mut Environment,
) -> EvalResult {
    eval(n.expression().as_node(), env)
}

fn eval_unary_expression_node(n: &UnaryExpressionNode, env: &mut Environment) -> EvalResult {
    let o = eval(n.expression().as_node(), env)?;
    match n.operator() {
        Token::Minus => operator::unary_minus(o.as_ref()),
        Token::Invert => operator::unary_invert(o.as_ref()),
        t => Err(format!("unknown unary operator: `{:?}`", t)),
    }
}

fn eval_binary_expression_node(n: &BinaryExpressionNode, env: &mut Environment) -> EvalResult {
    let left = eval(n.left().as_node(), env)?;
    let right = eval(n.right().as_node(), env)?;
    match n.operator() {
        Token::Plus => operator::binary_plus(left.as_ref(), right.as_ref()),
        Token::Minus => operator::binary_minus(left.as_ref(), right.as_ref()),
        Token::Asterisk => operator::binary_asterisk(left.as_ref(), right.as_ref()),
        Token::Slash => operator::binary_slash(left.as_ref(), right.as_ref()),
        Token::Percent => operator::binary_percent(left.as_ref(), right.as_ref()),
        Token::Power => operator::binary_power(left.as_ref(), right.as_ref()),
        Token::Eq => operator::binary_eq(left.as_ref(), right.as_ref()),
        Token::NotEq => operator::binary_noteq(left.as_ref(), right.as_ref()),
        Token::Lt => operator::binary_lt(left.as_ref(), right.as_ref()),
        Token::Gt => operator::binary_gt(left.as_ref(), right.as_ref()),
        Token::LtEq => operator::binary_lteq(left.as_ref(), right.as_ref()),
        Token::GtEq => operator::binary_gteq(left.as_ref(), right.as_ref()),
        Token::And => operator::binary_and(left.as_ref(), right.as_ref()),
        Token::Or => operator::binary_or(left.as_ref(), right.as_ref()),
        t => Err(format!("unknown binary operator: `{:?}`", t)),
    }
}

fn eval_call_expression_node(n: &CallExpressionNode, env: &mut Environment) -> EvalResult {
    //Note a function call is of the form `<identifier>(<arg(s)>)` or `<function literal>(<arg(s)>)`.
    let function: Function = {
        let f = n.function().as_any().downcast_ref::<FunctionLiteralNode>();
        if (f.is_some()) {
            let f = eval(f.unwrap(), env)?;
            let f = f.as_any().downcast_ref::<Function>();
            if (f.is_none()) {
                unreachable!();
            }
            f.unwrap().clone()
        } else {
            let identifier = n.function().as_any().downcast_ref::<IdentifierNode>();
            if (identifier.is_none()) {
                return Err("only identifier or function literal can be called".to_string());
            }
            let identifier: &IdentifierNode = identifier.unwrap();

            let function = match env.get(identifier.get_name()) {
                None => {
                    return Err(format!(
                        "function `{}` is not defined",
                        identifier.get_name()
                    ))
                }
                Some(e) => e,
            };
            let function = function.as_any().downcast_ref::<Function>();
            if (function.is_none()) {
                return Err(format!("`{}` is not a function", identifier.get_name()));
            }
            function.unwrap().clone()
        }
    };

    if (n.arguments().len() != function.parameters().len()) {
        return Err("argument number mismatch".to_string());
    }

    //constructs the following nested environment
    // { //outer
    //     { //function capture
    //         { //arguments
    //         }
    //     }
    // }
    let mut function_env = {
        let mut e = function.env().clone();
        e.set_outer(Some(Rc::new(env.clone())));
        Environment::new(Some(Rc::new(e)))
    };

    let parameters = function.parameters();
    for (i, param) in parameters.iter().enumerate() {
        function_env.set(
            param.get_name().to_string(),
            eval(n.arguments()[i].as_node(), env)?,
        )
    }

    //Extracts the value of `ReturnValue` as in `eval_root_node()`.
    //Without this, `let f = fn() { return 3; 4 }; let a = f(); f(); return 100;` returns `3` (not `100`).
    //See the comments of `eval_root_node()` and `eval_block_statement_node()` for related information.
    let result = eval_block_statement_node(function.body(), &function_env)?;
    if let Some(e) = result.as_any().downcast_ref::<ReturnValue>() {
        return Ok(e.value().clone());
    }
    Ok(result)
}

fn eval_if_expression_node(n: &IfExpressionNode, env: &mut Environment) -> EvalResult {
    let condition = eval(n.condition().as_node(), env)?;
    match condition.as_any().downcast_ref::<Bool>() {
        None => Err("if condition is not a boolean".to_string()),
        Some(condition) => {
            if (condition.value()) {
                eval(n.if_value().as_node(), env)
            } else if (n.else_value().is_some()) {
                eval(n.else_value().as_ref().unwrap().as_node(), env)
            } else {
                Ok(Rc::new(Null::new()))
            }
        }
    }
}

fn eval_integer_literal_node(n: &IntegerLiteralNode, _env: &Environment) -> EvalResult {
    Ok(Rc::new(Int::new(n.get_value())))
}

fn eval_float_literal_node(n: &FloatLiteralNode, _env: &Environment) -> EvalResult {
    Ok(Rc::new(Float::new(n.get_value())))
}

fn eval_boolean_literal_node(n: &BooleanLiteralNode, _env: &Environment) -> EvalResult {
    Ok(Rc::new(Bool::new(n.get_value())))
}

fn eval_character_literal_node(n: &CharacterLiteralNode, _env: &Environment) -> EvalResult {
    Ok(Rc::new(Char::new(n.get_value())))
}

fn eval_string_literal_node(n: &StringLiteralNode, _env: &Environment) -> EvalResult {
    Ok(Rc::new(Str::new(n.get_value().to_string())))
}

fn eval_function_literal_node(n: &FunctionLiteralNode, env: &mut Environment) -> EvalResult {
    Ok(Rc::new(Function::new(
        n.parameters().clone(),
        n.body().clone(),
        env.clone(),
    )))
}

fn eval_identifier_node(n: &IdentifierNode, env: &Environment) -> EvalResult {
    match env.get(n.get_name()) {
        None => Err(format!("`{}` is not defined", n.get_name())),
        Some(e) => Ok(e.clone()),
    }
}

#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use super::super::ast::*;
    use super::super::environment::Environment;
    use super::super::lexer::Lexer;
    use super::super::object::*;
    use super::super::parser::Parser;
    use super::super::token::Token;
    use super::eval;
    use super::EvalResult;

    fn __eval(s: &str) -> EvalResult {
        let mut lexer = Lexer::new(s);
        let mut v = Vec::new();
        loop {
            let token = lexer.get_next_token().unwrap();
            if (token == Token::Eof) {
                break;
            }
            v.push(token);
        }
        v.push(Token::Eof);
        let root = Parser::new(v).parse();
        assert!(root.is_ok());
        let mut env = Environment::new(None);
        eval(&root.unwrap(), &mut env)
    }

    fn read_and_eval(s: &str) -> Rc<dyn Object> {
        let r = __eval(s);
        match r {
            Ok(a) => a,
            Err(ref b) => {
                assert!(r.is_ok(), "{}", b);
                unreachable!();
            }
        }
    }

    fn assert_error(s: &str, error_message: &str) {
        let r = __eval(s);
        if let Ok(ref e) = r {
            println!("{}", e);
            assert!(r.is_err());
        }
        if let Err(e) = r {
            assert!(e.contains(error_message));
        }
    }

    fn assert_integer(s: &str, v: i32) {
        let o = read_and_eval(s);
        let o = o.as_any().downcast_ref::<Int>();
        assert!(o.is_some());
        assert_eq!(v, o.unwrap().value());
    }

    fn assert_float(s: &str, v: f64) {
        let o = read_and_eval(s);
        let o = o.as_any().downcast_ref::<Float>();
        assert!(o.is_some());
        assert!((v - o.unwrap().value()).abs() < 1e-6);
    }

    fn assert_boolean(s: &str, v: bool) {
        let o = read_and_eval(s);
        let o = o.as_any().downcast_ref::<Bool>();
        assert!(o.is_some());
        assert_eq!(v, o.unwrap().value());
    }

    fn assert_character(s: &str, v: char) {
        let o = read_and_eval(s);
        let o = o.as_any().downcast_ref::<Char>();
        assert!(o.is_some());
        assert_eq!(v, o.unwrap().value());
    }

    fn assert_string(s: &str, v: &str) {
        let o = read_and_eval(s);
        let o = o.as_any().downcast_ref::<Str>();
        assert!(o.is_some());
        assert_eq!(v, o.unwrap().value());
    }

    fn assert_null(s: &str) {
        let o = read_and_eval(s);
        let o = o.as_any().downcast_ref::<Null>();
        assert!(o.is_some());
    }

    #[test]
    fn test01() {
        //literal
        assert_integer(r#" 5 "#, 5);
        assert_boolean(r#" true "#, true);
        assert_boolean(r#" false "#, false);

        //invert
        assert_boolean(r#" !true "#, false);
        assert_boolean(r#" !false "#, true);
        assert_boolean(r#" !!true "#, true);
        assert_boolean(r#" !!false "#, false);
        assert_boolean(r#" !0 "#, true);
        assert_boolean(r#" !!0 "#, false);
        assert_boolean(r#" !1 "#, false);
        assert_boolean(r#" !!1 "#, true);
        assert_boolean(r#" !0.0 "#, true);
        assert_boolean(r#" !3.14 "#, false);
        assert_boolean(r#" !"" "#, true);
        assert_boolean(r#" !"あ" "#, false);

        //unary minus
        assert_integer(r#" -5 "#, -5);
        assert_integer(r#" --5 "#, 5);
        assert_float(r#" -3.14 "#, -3.14);

        //binary + - * /
        assert_integer(r#" 2 + 3 "#, 5);
        assert_integer(r#" 2 - 3 "#, -1);
        assert_integer(r#" 2 * 3 "#, 6);
        assert_integer(r#" 2 / 3 "#, 0);
        assert_integer(r#" 4 / 3 "#, 1);
        assert_integer(r#" 2 + 3 * 4"#, 14);
        assert_integer(r#" (2 + 3) * 4"#, 20);
        assert_float(r#" 3.14 + 1.0 "#, 4.14);
        assert_float(r#" 3.14 - 1.0 "#, 2.14);
        assert_float(r#" 3.14 * 2.0 "#, 6.28);
        assert_float(r#" 3.14 / 2.0 "#, 1.57);
        assert_string(r#" "hello" + "world" "#, "helloworld");

        //binary == != < >
        assert_boolean(r#" true == false "#, false);
        assert_boolean(r#" true == true "#, true);
        assert_boolean(r#" true != false "#, true);
        assert_boolean(r#" false != false "#, false);
        assert_boolean(r#" 1 == 0 "#, false);
        assert_boolean(r#" 1 == 1 "#, true);
        assert_boolean(r#" 1 != 0 "#, true);
        assert_boolean(r#" 0 != 0 "#, false);
        assert_boolean(r#" 0 != 0 "#, false);
        assert_boolean(r#" 1 > 0 "#, true);
        assert_boolean(r#" 0 > 0 "#, false);
        assert_boolean(r#" -1 > 0 "#, false);
        assert_boolean(r#" 1 < 0 "#, false);
        assert_boolean(r#" 0 < 0 "#, false);
        assert_boolean(r#" -1 < 0 "#, true);
        assert_boolean(r#" 3.14 == 3.14 "#, true);
        assert_boolean(r#" 3.14 == 3.15 "#, false);
        assert_boolean(r#" 3.14 != 3.14 "#, false);
        assert_boolean(r#" 3.14 != 3.15 "#, true);
        assert_boolean(r#" 'a' == 'a' "#, true);
        assert_boolean(r#" 'a' != 'a' "#, false);
        assert_boolean(r#" 'a' == 'b' "#, false);
        assert_boolean(r#" 'a' != 'b' "#, true);
        assert_boolean(r#" "hello" == "hello" "#, true);
        assert_boolean(r#" "hello" != "hello" "#, false);
        assert_boolean(r#" "hello" == "world" "#, false);
        assert_boolean(r#" "hello" != "world" "#, true);
        assert_boolean(r#" 3.2 < 3.1 "#, false);
        assert_boolean(r#" 3.2 < 3.2 "#, false);
        assert_boolean(r#" 3.2 < 3.3 "#, true);
        assert_boolean(r#" 3.1 > 3.2 "#, false);
        assert_boolean(r#" 3.2 > 3.2 "#, false);
        assert_boolean(r#" 3.3 > 3.2 "#, true);
        assert_boolean(r#" 'b' < 'a' "#, false);
        assert_boolean(r#" 'b' < 'b' "#, false);
        assert_boolean(r#" 'b' < 'c' "#, true);
        assert_boolean(r#" 'a' > 'b' "#, false);
        assert_boolean(r#" 'b' > 'b' "#, false);
        assert_boolean(r#" 'c' > 'b' "#, true);
        assert_boolean(r#" "xb" < "xa" "#, false);
        assert_boolean(r#" "xb" < "xb" "#, false);
        assert_boolean(r#" "xb" < "xc" "#, true);
        assert_boolean(r#" "xa" > "xb" "#, false);
        assert_boolean(r#" "xb" > "xb" "#, false);
        assert_boolean(r#" "xc" > "xb" "#, true);
        assert_boolean(r#" 3.2 <= 3.1 "#, false);
        assert_boolean(r#" 3.2 <= 3.2 "#, true);
        assert_boolean(r#" 3.2 <= 3.3 "#, true);
        assert_boolean(r#" 3.1 >= 3.2 "#, false);
        assert_boolean(r#" 3.2 >= 3.2 "#, true);
        assert_boolean(r#" 3.3 >= 3.2 "#, true);
        assert_boolean(r#" 'b' <= 'a' "#, false);
        assert_boolean(r#" 'b' <= 'b' "#, true);
        assert_boolean(r#" 'b' <= 'c' "#, true);
        assert_boolean(r#" 'a' >= 'b' "#, false);
        assert_boolean(r#" 'b' >= 'b' "#, true);
        assert_boolean(r#" 'c' >= 'b' "#, true);
        assert_boolean(r#" "xb" <= "xa" "#, false);
        assert_boolean(r#" "xb" <= "xb" "#, true);
        assert_boolean(r#" "xb" <= "xc" "#, true);
        assert_boolean(r#" "xa" >= "xb" "#, false);
        assert_boolean(r#" "xb" >= "xb" "#, true);
        assert_boolean(r#" "xc" >= "xb" "#, true);
    }

    #[test]
    fn test02() {
        assert_integer(r#" 5 % 3 "#, 2);
        assert_float(r#" 5.0 % 3.0 "#, 2.0);
        assert_error(r#" 1 % 0 "#, "zero division");
        assert_error(r#" 1.0 % 0.0 "#, "zero division");

        assert_integer(r#" 2**3 "#, 8);
        assert_float(r#" 2.0**3.0 "#, 8.0);
        assert_error(r#" 2**-1 "#, "negative exponent");
        assert_float(r#" 2.0**-1.0 "#, 0.5);

        assert_boolean(r#" true || true "#, true);
        assert_boolean(r#" true || false "#, true);
        assert_boolean(r#" false || true "#, true);
        assert_boolean(r#" false || false "#, false);
        assert_error(r#" false || 0 "#, "not a boolean");

        assert_boolean(r#" true && true "#, true);
        assert_boolean(r#" true && false "#, false);
        assert_boolean(r#" false && true "#, false);
        assert_boolean(r#" false && false "#, false);
        assert_error(r#" false && 0 "#, "not a boolean");
    }

    #[test]
    fn test03() {
        assert_integer(r#" if (true) { 10 } "#, 10);
        assert_null(r#" if (false) { 10 } "#);
        assert_boolean(r#" if (true) { false } "#, false);
        assert_integer(r#" if (false) { 10 } else { 20 }"#, 20);
        assert_error(r#" if (true) { let a = 3; } a"#, "not defined");
    }

    #[test]
    fn test04() {
        assert_null(r#" return; 15"#);
        assert_integer(r#" return 10; 15"#, 10);
        assert_integer(r#" 5; return 2 * 5; 15"#, 10);
        assert_boolean(r#" return true; false"#, true);
        assert_integer(
            r#" if (10 > 1) {
                    if (10 > 1) {
                        return 10;
                    }
                    return 1;
                } "#,
            10,
        );
    }

    #[test]
    fn test05() {
        assert_integer(r#" let a = 5; a; "#, 5);
        assert_integer(r#" let a = 5 * 5; a; "#, 25);
        assert_integer(r#" let a = 1; let b = a * 2; a + b "#, 3);
        assert_error(r#" let a = 1; b "#, "not defined");
        assert_error(r#" let a = 1; let a = 2; "#, "already");
        assert_integer(
            r#" {
                    let a = 1;
                    {
                        let a = 2;
                        a
                    }
                }
             "#,
            2,
        );
        assert_integer(
            r#" {
                    let a = 1;
                    {
                        let a = 2;
                        a
                    }
                    a
                }
             "#,
            1,
        );
    }

    #[test]
    fn test06() {
        let s = r#"
            fn () { x + 2; }
        "#;
        let o = read_and_eval(s);

        let o = o.as_any().downcast_ref::<Function>();
        assert!(o.is_some());
        let f = o.unwrap();

        assert_eq!(0, f.parameters().len());

        assert_eq!(1, f.body().statements().len());
        let s = f.body().statements()[0]
            .as_any()
            .downcast_ref::<ExpressionStatementNode>();
        assert!(s.is_some());
        let s = s.unwrap();

        let s = s
            .expression()
            .as_any()
            .downcast_ref::<BinaryExpressionNode>();
        assert!(s.is_some());
        let s = s.unwrap();

        assert_eq!(s.operator(), &Token::Plus);

        let left = s.left().as_any().downcast_ref::<IdentifierNode>();
        assert!(left.is_some());
        assert_eq!(left.unwrap().get_name(), "x");

        let right = s.right().as_any().downcast_ref::<IntegerLiteralNode>();
        assert!(right.is_some());
        assert_eq!(right.unwrap().get_value(), 2);
    }

    #[test]
    fn test07() {
        assert_integer(r#" let f = fn(x) { x; }; f(5) "#, 5);
        assert_integer(r#" let f = fn(x, y) { x + y }; f(1, 2) "#, 3);
        assert_integer(r#" fn() { return 3; }() "#, 3);
        assert_integer(r#" let a = 3; let f = fn() { a }; f() "#, 3);
        assert_integer(
            r#" let f = fn() { return 3; 4 }; let a = f(); f(); return 100; "#,
            100,
        );
        assert_integer(
            r#" let f = fn(x) { fn(y) { x + y } }; let g = f(1); g(2) "#,
            3,
        );
        assert_integer(
            r#"
                let f = fn(x) { fn(y) { fn(z) { x + y + z } } }; let g = f(1); let h = g(2); h(3)
            "#,
            6,
        );
        //TODO uncomment after implementing assignment
        //         assert_integer(
        //             r#" let a = 1; let f = fn(x) { fn(y) { x + y } }; let g = f(a); a = 100; g(2) "#,
        //             3,
        //         );
        assert_integer(
            r#" let f = fn(g) { g(10) }; let g = fn(x) { x * 10 }; f(g) "#,
            100,
        );
        assert_integer(
            r#" let factorial = fn(x) { if (x == 0) { return 1; } return x * factorial(x - 1); }; factorial(4) "#,
            24,
        );
        // assert_integer(r#" let a = 3; let f = fn() { a }; a = 10; f() "#, 10); //TODO uncomment after implementing assignment
        assert_error(r#" let f = 3; f(3) "#, "not a function");
        assert_error(r#" g(3) "#, "not defined");
        assert_error(r#" let f = fn(x) { x; }; f(5, 10) "#, "number mismatch");
        assert_error(r#" 1(3) "#, "can be called");
    }
}
