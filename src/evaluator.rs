use super::ast::*;
use super::object::*;
use super::token::Token;

type EvalResult = Result<Box<dyn Object>, String>;

fn eval_root_node(n: &RootNode) -> EvalResult {
    eval(n.statements()[0].as_node()) //TODO
}

fn eval_expression_statement_node(n: &ExpressionStatementNode) -> EvalResult {
    eval(n.expression().as_node())
}

fn eval_unary_expression_node(n: &UnaryExpressionNode) -> EvalResult {
    match n.operator() {
        Token::Minus => {
            let o = eval(n.expression().as_node())?;
            if let Some(o) = o.as_any().downcast_ref::<Integer>() {
                return Ok(Box::new(Integer::new(-o.value())));
            }
            Err("operand of unary `-` is not a number".to_string())
        }
        Token::Invert => {
            let o = eval(n.expression().as_node())?;
            if let Some(o) = o.as_any().downcast_ref::<Boolean>() {
                return Ok(Box::new(Boolean::new(!o.value())));
            }
            if let Some(o) = o.as_any().downcast_ref::<Integer>() {
                return Ok(Box::new(Boolean::new(o.value() == 0)));
            }
            Err("operand of unary `!` is not a number nor a boolean".to_string())
        }
        _ => unreachable!(),
    }
}

fn eval_binary_expression_node(n: &BinaryExpressionNode) -> EvalResult {
    let left = eval(n.left().as_node())?;
    let right = eval(n.right().as_node())?;

    match n.operator() {
        Token::Plus => {
            if let Some(left) = left.as_any().downcast_ref::<Integer>() {
                if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                    return Ok(Box::new(Integer::new(left.value() + right.value())));
                }
            }
            Err("operand of binary `+` is not a number nor a string".to_string())
        }
        Token::Minus => {
            if let Some(left) = left.as_any().downcast_ref::<Integer>() {
                if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                    return Ok(Box::new(Integer::new(left.value() - right.value())));
                }
            }
            Err("operand of binary `-` is not a number".to_string())
        }
        Token::Asterisk => {
            if let Some(left) = left.as_any().downcast_ref::<Integer>() {
                if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                    return Ok(Box::new(Integer::new(left.value() * right.value())));
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
                    return Ok(Box::new(Integer::new(left.value() / right.value())));
                }
            }
            Err("operand of binary `/` is not a number".to_string())
        }
        Token::Eq => {
            if let Some(left) = left.as_any().downcast_ref::<Integer>() {
                if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                    return Ok(Box::new(Boolean::new(left.value() == right.value())));
                }
            }
            if let Some(left) = left.as_any().downcast_ref::<Boolean>() {
                if let Some(right) = right.as_any().downcast_ref::<Boolean>() {
                    return Ok(Box::new(Boolean::new(left.value() == right.value())));
                }
            }
            Err("operand of binary `==` is not a number, a boolean nor a string".to_string())
        }
        Token::NotEq => {
            if let Some(left) = left.as_any().downcast_ref::<Integer>() {
                if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                    return Ok(Box::new(Boolean::new(left.value() != right.value())));
                }
            }
            if let Some(left) = left.as_any().downcast_ref::<Boolean>() {
                if let Some(right) = right.as_any().downcast_ref::<Boolean>() {
                    return Ok(Box::new(Boolean::new(left.value() != right.value())));
                }
            }
            Err("operand of binary `!=` is not a number, a boolean nor a string".to_string())
        }
        Token::Lt => {
            if let Some(left) = left.as_any().downcast_ref::<Integer>() {
                if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                    return Ok(Box::new(Boolean::new(left.value() < right.value())));
                }
            }
            Err("operand of binary `<` is not a number nor a string".to_string())
        }
        Token::Gt => {
            if let Some(left) = left.as_any().downcast_ref::<Integer>() {
                if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                    return Ok(Box::new(Boolean::new(left.value() > right.value())));
                }
            }
            Err("operand of binary `<` is not a number nor a string".to_string())
        }
        _ => unimplemented!(),
    }
}

fn eval_integer_literal_node(n: &IntegerLiteralNode) -> EvalResult {
    match n.token() {
        Token::Int(v) => Ok(Box::new(Integer::new(*v))),
        _ => unreachable!(),
    }
}

fn eval_boolean_literal_node(n: &BooleanLiteralNode) -> EvalResult {
    match n.token() {
        Token::True => Ok(Box::new(Boolean::new(true))),
        Token::False => Ok(Box::new(Boolean::new(false))),
        _ => unreachable!(),
    }
}

pub fn eval(node: &dyn Node) -> EvalResult {
    if let Some(n) = node.as_any().downcast_ref::<RootNode>() {
        return eval_root_node(n);
    }

    if let Some(n) = node.as_any().downcast_ref::<ExpressionStatementNode>() {
        return eval_expression_statement_node(n);
    }

    if let Some(n) = node.as_any().downcast_ref::<UnaryExpressionNode>() {
        return eval_unary_expression_node(n);
    }

    if let Some(n) = node.as_any().downcast_ref::<BinaryExpressionNode>() {
        return eval_binary_expression_node(n);
    }

    if let Some(n) = node.as_any().downcast_ref::<IntegerLiteralNode>() {
        return eval_integer_literal_node(n);
    }

    if let Some(n) = node.as_any().downcast_ref::<BooleanLiteralNode>() {
        return eval_boolean_literal_node(n);
    }

    Err("not yet implemented".to_string()) //TODO replace it with `unreachable!()`
}

#[cfg(test)]
mod tests {

    use super::super::lexer::Lexer;
    use super::super::object::*;
    use super::super::parser::Parser;
    use super::super::token::Token;
    use super::eval;

    fn read_and_eval(s: &str) -> Box<dyn Object> {
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
        let r = eval(&root.unwrap());
        assert!(r.is_ok());
        r.unwrap()
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
}
