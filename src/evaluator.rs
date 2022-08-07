use super::ast::*;
use super::object::*;
use super::token::Token;

fn eval(node: &dyn Node) -> Box<dyn Object> {
    if let Some(n) = node.as_any().downcast_ref::<RootNode>() {
        return eval(n.statements()[0].as_node());
    }

    if let Some(n) = node.as_any().downcast_ref::<ExpressionStatementNode>() {
        return eval(n.expression().as_node());
    }

    if let Some(n) = node.as_any().downcast_ref::<UnaryExpressionNode>() {
        match n.operator() {
            Token::Minus => {
                let o = eval(n.expression().as_node());
                if let Some(o) = o.as_any().downcast_ref::<Integer>() {
                    return Box::new(Integer::new(-o.value()));
                }
                unimplemented!();
            }
            Token::Invert => {
                let o = eval(n.expression().as_node());
                if let Some(o) = o.as_any().downcast_ref::<Boolean>() {
                    return Box::new(Boolean::new(!o.value()));
                }
                if let Some(o) = o.as_any().downcast_ref::<Integer>() {
                    return Box::new(Boolean::new(o.value() == 0));
                }
                unimplemented!();
            }
            _ => unreachable!(),
        }
    }

    if let Some(n) = node.as_any().downcast_ref::<IntegerLiteralNode>() {
        if let Token::Int(v) = n.token() {
            return Box::new(Integer::new(*v));
        }
        unreachable!();
    }

    if let Some(n) = node.as_any().downcast_ref::<BooleanLiteralNode>() {
        match n.token() {
            Token::True => return Box::new(Boolean::new(true)),
            Token::False => return Box::new(Boolean::new(false)),
            _ => unreachable!(),
        }
    }

    Box::new(Null::new())
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
        eval(&root.unwrap())
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
        assert_integer(r#" 5 "#, 5);
        assert_boolean(r#" true "#, true);
        assert_boolean(r#" false "#, false);
        assert_boolean(r#" !true "#, false);
        assert_boolean(r#" !false "#, true);
        assert_boolean(r#" !!true "#, true);
        assert_boolean(r#" !!false "#, false);
        assert_boolean(r#" !0 "#, true);
        assert_boolean(r#" !!0 "#, false);
        assert_boolean(r#" !1 "#, false);
        assert_boolean(r#" !!1 "#, true);
        assert_integer(r#" -5 "#, -5);
        assert_integer(r#" --5 "#, 5);
    }
}
