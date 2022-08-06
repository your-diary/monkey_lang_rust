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

    if let Some(n) = node.as_any().downcast_ref::<IntegerLiteralNode>() {
        if let Token::Int(v) = n.token() {
            return Box::new(Integer::new(*v));
        }
    }

    Box::new(Null::new())
}

#[cfg(test)]
mod tests {

    use super::super::ast::*;
    use super::super::lexer::Lexer;
    use super::super::object::*;
    use super::super::parser::Parser;
    use super::super::token::Token;
    use super::eval;

    fn parse(s: &str) -> RootNode {
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
        root.unwrap()
    }

    fn assert_integer(o: &dyn Object, v: i32) {
        let o = o.as_any().downcast_ref::<Integer>();
        assert!(o.is_some());
        assert_eq!(v, o.unwrap().value());
    }

    #[test]
    fn test01() {
        let input = r#"
            5;
        "#;

        let root = parse(input);
        let o = eval(&root);
        assert_integer(o.as_ref(), 5);
    }
}
