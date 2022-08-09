use super::token::{self, Token};
use super::util;

pub struct Lexer {
    input: Vec<char>,
    position: usize,  //last read position
    ch: Option<char>, //last read character
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = String::from(input).chars().collect();
        let first_ch = chars.get(0).copied();
        Lexer {
            input: chars,
            position: 0,
            ch: first_ch,
        }
    }

    fn read_next_char(&mut self) {
        self.position += 1;
        self.ch = self.input.get(self.position).copied();
    }

    //like `read_next_char()` but immutable and instead returns the next character
    fn peek_next_char(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn eat_whitespace(&mut self) {
        while (self.ch.is_some() && self.ch.unwrap().is_ascii_whitespace()) {
            self.read_next_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while (self.ch.is_some() && util::is_identifier(self.ch.unwrap())) {
            self.read_next_char();
        }
        self.input[position..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while (self.ch.is_some() && self.ch.unwrap().is_ascii_digit()) {
            self.read_next_char();
        }
        self.input[position..self.position].iter().collect()
    }

    pub fn get_next_token(&mut self) -> Token {
        self.eat_whitespace();
        if (self.ch.is_none()) {
            return Token::Eof;
        }
        let sequence: String = match self.ch.unwrap() {
            c if c.is_ascii_digit() => self.read_number(),
            c if util::is_identifier(c) => self.read_identifier(), //this includes a keyword such as `if`
            //operators
            c => {
                let ret = match c {
                    '=' => {
                        let next_ch = self.peek_next_char();
                        if (next_ch.is_some() && (next_ch.unwrap() == '=')) {
                            self.read_next_char();
                            "==".to_string()
                        } else {
                            "=".to_string()
                        }
                    }
                    '!' => {
                        let next_ch = self.peek_next_char();
                        if (next_ch.is_some() && (next_ch.unwrap() == '=')) {
                            self.read_next_char();
                            "!=".to_string()
                        } else {
                            "!".to_string()
                        }
                    }
                    c => c.to_string(),
                };
                self.read_next_char(); //moves to the next position as `read_identifier()` does
                ret
            }
        };
        match token::lookup_token(&sequence) {
            Token::Ident(_) => Token::Ident(sequence),
            Token::Int(_) => Token::Int(sequence.parse().unwrap()),
            t => t,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::super::token::Token;
    use super::Lexer;

    #[test]
    fn test01() {
        let input = "=+(){},;";

        let expected = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for expected_token in expected {
            let token = lexer.get_next_token();
            assert_eq!(expected_token, token);
        }
    }

    #[test]
    fn test02() {
        let input = r#"
            let five = 5;
            let ten2 = 10;
            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);

            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 ==10;
            10 != 9;
        "#;

        let expected = vec![
            //1
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            //2
            Token::Let,
            Token::Ident("ten2".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            //3
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            //4
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::Lparen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::Rparen,
            Token::Semicolon,
            //5
            Token::Invert,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            //6
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::Semicolon,
            //7
            Token::If,
            Token::Lparen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
            //8
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
            //9
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for expected_token in expected {
            let token = lexer.get_next_token();
            assert_eq!(expected_token, token);
        }
    }
}
