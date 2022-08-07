use std::rc::Rc;

use super::ast::*;
use super::environment::Environment;
use super::object::*;
use super::token::Token;

type EvalResult = Result<Rc<dyn Object>, String>;

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

    if let Some(n) = node.as_any().downcast_ref::<BooleanLiteralNode>() {
        return eval_boolean_literal_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<FunctionLiteralNode>() {
        return eval_function_literal_node(n, env);
    }

    if let Some(n) = node.as_any().downcast_ref::<IdentifierNode>() {
        match env.get(n.get_name()) {
            None => return Err(format!("`{}` is not defined", n.get_name())),
            Some(e) => return Ok(e.clone()),
        }
    }

    Err("not yet implemented".to_string()) //TODO replace it with `unreachable!()`
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
fn eval_block_statement_node(n: &BlockStatementNode, env: &mut Environment) -> EvalResult {
    let mut ret = Rc::new(Null::new()) as _;
    for statement in n.statements() {
        ret = eval(statement.as_node(), env)?;
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
    Ok(Rc::new(ReturnValue::new(eval(
        n.expression().as_node(),
        env,
    )?)))
}

fn eval_expression_statement_node(
    n: &ExpressionStatementNode,
    env: &mut Environment,
) -> EvalResult {
    eval(n.expression().as_node(), env)
}

fn eval_unary_expression_node(n: &UnaryExpressionNode, env: &mut Environment) -> EvalResult {
    match n.operator() {
        Token::Minus => {
            let o = eval(n.expression().as_node(), env)?;
            if let Some(o) = o.as_any().downcast_ref::<Integer>() {
                return Ok(Rc::new(Integer::new(-o.value())));
            }
            Err("operand of unary `-` is not a number".to_string())
        }
        Token::Invert => {
            let o = eval(n.expression().as_node(), env)?;
            if let Some(o) = o.as_any().downcast_ref::<Boolean>() {
                return Ok(Rc::new(Boolean::new(!o.value())));
            }
            if let Some(o) = o.as_any().downcast_ref::<Integer>() {
                return Ok(Rc::new(Boolean::new(o.value() == 0)));
            }
            Err("operand of unary `!` is not a number nor a boolean".to_string())
        }
        _ => unreachable!(),
    }
}

fn eval_binary_expression_node(n: &BinaryExpressionNode, env: &mut Environment) -> EvalResult {
    let left = eval(n.left().as_node(), env)?;
    let right = eval(n.right().as_node(), env)?;

    match n.operator() {
        Token::Plus => {
            if let Some(left) = left.as_any().downcast_ref::<Integer>() {
                if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                    return Ok(Rc::new(Integer::new(left.value() + right.value())));
                }
            }
            Err("operand of binary `+` is not a number nor a string".to_string())
        }
        Token::Minus => {
            if let Some(left) = left.as_any().downcast_ref::<Integer>() {
                if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                    return Ok(Rc::new(Integer::new(left.value() - right.value())));
                }
            }
            Err("operand of binary `-` is not a number".to_string())
        }
        Token::Asterisk => {
            if let Some(left) = left.as_any().downcast_ref::<Integer>() {
                if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                    return Ok(Rc::new(Integer::new(left.value() * right.value())));
                }
            }
            Err("operand of binary `*` is not a number".to_string())
        }
        Token::Slash => {
            if let Some(left) = left.as_any().downcast_ref::<Integer>() {
                if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                    if (right.value() == 0) {
                        return Err("zero division".to_string());
                    }
                    return Ok(Rc::new(Integer::new(left.value() / right.value())));
                }
            }
            Err("operand of binary `/` is not a number".to_string())
        }
        Token::Eq => {
            if let Some(left) = left.as_any().downcast_ref::<Integer>() {
                if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                    return Ok(Rc::new(Boolean::new(left.value() == right.value())));
                }
            }
            if let Some(left) = left.as_any().downcast_ref::<Boolean>() {
                if let Some(right) = right.as_any().downcast_ref::<Boolean>() {
                    return Ok(Rc::new(Boolean::new(left.value() == right.value())));
                }
            }
            Err("operand of binary `==` is not a number, a boolean nor a string".to_string())
        }
        Token::NotEq => {
            if let Some(left) = left.as_any().downcast_ref::<Integer>() {
                if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                    return Ok(Rc::new(Boolean::new(left.value() != right.value())));
                }
            }
            if let Some(left) = left.as_any().downcast_ref::<Boolean>() {
                if let Some(right) = right.as_any().downcast_ref::<Boolean>() {
                    return Ok(Rc::new(Boolean::new(left.value() != right.value())));
                }
            }
            Err("operand of binary `!=` is not a number, a boolean nor a string".to_string())
        }
        Token::Lt => {
            if let Some(left) = left.as_any().downcast_ref::<Integer>() {
                if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                    return Ok(Rc::new(Boolean::new(left.value() < right.value())));
                }
            }
            Err("operand of binary `<` is not a number nor a string".to_string())
        }
        Token::Gt => {
            if let Some(left) = left.as_any().downcast_ref::<Integer>() {
                if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                    return Ok(Rc::new(Boolean::new(left.value() > right.value())));
                }
            }
            Err("operand of binary `<` is not a number nor a string".to_string())
        }
        _ => unimplemented!(),
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

    let arguments = n.arguments();
    let parameters = function.parameters().clone();
    if (arguments.len() != parameters.len()) {
        return Err("argument number mismatch".to_string());
    }

    let body = function.body().clone();

    let mut function_env = function.env().clone();
    for i in 0..parameters.len() {
        function_env.set(
            parameters[i].get_name().to_string(),
            eval(arguments[i].as_node(), env)?,
        )
    }

    eval(&body, &mut function_env)
}

fn eval_if_expression_node(n: &IfExpressionNode, env: &mut Environment) -> EvalResult {
    let condition = eval(n.condition().as_node(), env)?;
    if let Some(condition) = condition.as_any().downcast_ref::<Boolean>() {
        if (condition.value()) {
            return eval(n.if_value().as_node(), env);
        } else if (n.else_value().is_some()) {
            return eval(n.else_value().as_ref().unwrap().as_node(), env);
        } else {
            return Ok(Rc::new(Null::new()));
        }
    }
    Err("if condition is not a boolean".to_string())
}

fn eval_integer_literal_node(n: &IntegerLiteralNode, _env: &mut Environment) -> EvalResult {
    match n.token() {
        Token::Int(v) => Ok(Rc::new(Integer::new(*v))),
        _ => unreachable!(),
    }
}

fn eval_boolean_literal_node(n: &BooleanLiteralNode, _env: &mut Environment) -> EvalResult {
    match n.token() {
        Token::True => Ok(Rc::new(Boolean::new(true))),
        Token::False => Ok(Rc::new(Boolean::new(false))),
        _ => unreachable!(),
    }
}

fn eval_function_literal_node(n: &FunctionLiteralNode, env: &mut Environment) -> EvalResult {
    Ok(Rc::new(Function::new(
        n.parameters().clone(),
        n.body().clone(),
        env.clone(), //TODO should we clone?
    )))
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
            let token = lexer.get_next_token();
            if (token == Token::Eof) {
                break;
            }
            v.push(token);
        }
        v.push(Token::Eof);
        let root = Parser::new(v).parse();
        assert!(root.is_some());
        let mut env = Environment::new();
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
        let o = o.as_any().downcast_ref::<Integer>();
        assert!(o.is_some());
        assert_eq!(v, o.unwrap().value());
    }

    fn assert_boolean(s: &str, v: bool) {
        let o = read_and_eval(s);
        let o = o.as_any().downcast_ref::<Boolean>();
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

        //unary minus
        assert_integer(r#" -5 "#, -5);
        assert_integer(r#" --5 "#, 5);

        //binary + - * /
        assert_integer(r#" 2 + 3 "#, 5);
        assert_integer(r#" 2 - 3 "#, -1);
        assert_integer(r#" 2 * 3 "#, 6);
        assert_integer(r#" 2 / 3 "#, 0);
        assert_integer(r#" 4 / 3 "#, 1);
        assert_integer(r#" 2 + 3 * 4"#, 14);
        assert_integer(r#" (2 + 3) * 4"#, 20);

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
    }

    #[test]
    fn test02() {
        assert_integer(r#" if (true) { 10 } "#, 10);
        assert_null(r#" if (false) { 10 } "#);
        assert_boolean(r#" if (true) { false } "#, false);
        assert_integer(r#" if (false) { 10 } else { 20 }"#, 20);
    }

    #[test]
    fn test03() {
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
    fn test04() {
        assert_integer(r#" let a = 5; a; "#, 5);
        assert_integer(r#" let a = 5 * 5; a; "#, 25);
        assert_integer(r#" let a = 1; let b = a * 2; a + b "#, 3);
        assert_error(r#" let a = 1; b "#, "not defined");
        assert_error(r#" let a = 1; let a = 2; "#, "already");
        //TODO: uncomment
        //         assert_integer(
        //             r#" {
        //                 let a = 1;
        //                 {
        //                     let a = 2;
        //                     a
        //                 }
        //             }
        //              "#,
        //             2,
        //         );
    }

    #[test]
    fn test05() {
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
        match left.unwrap().token() {
            Token::Ident(n) if (n == "x") => (),
            _ => panic!(),
        }

        let right = s.right().as_any().downcast_ref::<IntegerLiteralNode>();
        assert!(right.is_some());
        match right.unwrap().token() {
            Token::Int(2) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn test06() {
        assert_integer(r#" let f = fn(x) { x; }; f(5) "#, 5);
        assert_integer(r#" let f = fn(x, y) { x + y }; f(1, 2) "#, 3);
        assert_integer(r#" fn() { return 3; }() "#, 3);
        assert_integer(r#" let a = 3; let f = fn() { a }; f() "#, 3);
        // assert_integer(r#" let a = 3; let f = fn() { a }; a = 10; f() "#, 10); //TODO uncomment after implementing assignment
        assert_error(r#" let f = 3; f(3) "#, "not a function");
        assert_error(r#" g(3) "#, "not defined");
        assert_error(r#" let f = fn(x) { x; }; f(5, 10) "#, "number mismatch");
        assert_error(r#" 1(3) "#, "can be called");
    }
}
