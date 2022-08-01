use std::rc::Rc;

use super::token::Token;

trait Node {
    fn get_literal(&self) -> Option<String>;
}

trait Statement: Node {}

trait Expression: Node {}

/*-------------------------------------*/

struct Root {
    statements: Vec<Box<dyn Statement>>,
}

impl Node for Root {
    fn get_literal(&self) -> Option<String> {
        if (self.statements.is_empty()) {
            self.statements[0].get_literal()
        } else {
            None
        }
    }
}

/*-------------------------------------*/

struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn get_literal(&self) -> Option<String> {
        self.token.literal().clone()
    }
}

impl Expression for Identifier {}

/*-------------------------------------*/

struct LetStatement {
    token: Token,
    left: Identifier,
    right: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn get_literal(&self) -> Option<String> {
        self.token.literal().clone()
    }
}

impl Statement for LetStatement {}

/*-------------------------------------*/
