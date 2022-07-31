use super::token::{self, Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,      //last read position
    read_position: usize, //next character position (always `position + 1`)
    ch: char,             //last read character
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut ret = Lexer {
            input: String::from(input).chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        ret.read_next_char();
        ret
    }

    pub fn read_next_char(&mut self) {
        if (self.read_position >= self.input.len()) {
            self.ch = '\0';
            return;
        }
        self.ch = self.input[self.read_position];
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn eat_whitespace(&mut self) {
        while (self.ch.is_ascii_whitespace()) {
            self.read_next_char();
        }
    }

    pub fn get_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_next_char();
        }
        self.input[position..self.position].iter().collect()
    }

    pub fn get_number(&mut self) -> String {
        let position = self.position;
        while (self.ch.is_ascii_digit()) {
            self.read_next_char();
        }
        self.input[position..self.position].iter().collect()
    }

    pub fn get_next_token(&mut self) -> Token {
        self.eat_whitespace();
        let ret = match self.ch {
            '\0' => Token::new(TokenType::Eof, None),
            '=' => Token::new(TokenType::Assign, None),
            '+' => Token::new(TokenType::Plus, None),
            ',' => Token::new(TokenType::Comma, None),
            ';' => Token::new(TokenType::Semicolon, None),
            '(' => Token::new(TokenType::Lparen, None),
            ')' => Token::new(TokenType::Rparen, None),
            '{' => Token::new(TokenType::Lbrace, None),
            '}' => Token::new(TokenType::Rbrace, None),
            c if is_letter(c) => {
                let identifier = self.get_identifier();
                //early return as `get_identifier()` calls `read_next_char()` internally
                return match token::lookup_token_type(&identifier) {
                    TokenType::Ident => Token::new(TokenType::Ident, Some(&identifier)),
                    t => Token::new(t, None),
                };
            }
            c if c.is_ascii_digit() => {
                return Token::new(TokenType::Int, Some(&self.get_number()));
            }
            _ => Token::new(TokenType::Illegal, None),
        };
        self.read_next_char();
        ret
    }
}

fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || (c == '_')
}

#[cfg(test)]
mod tests {

    use super::super::token::{Token, TokenType};
    use super::Lexer;

    #[test]
    fn test01() {
        let input = "=+(){},;";

        let expected = vec![
            Token::new(TokenType::Assign, None),
            Token::new(TokenType::Plus, None),
            Token::new(TokenType::Lparen, None),
            Token::new(TokenType::Rparen, None),
            Token::new(TokenType::Lbrace, None),
            Token::new(TokenType::Rbrace, None),
            Token::new(TokenType::Comma, None),
            Token::new(TokenType::Semicolon, None),
            Token::new(TokenType::Eof, None),
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
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
        "#;

        let expected = vec![
            //1
            Token::new(TokenType::Let, None),
            Token::new(TokenType::Ident, Some("five")),
            Token::new(TokenType::Assign, None),
            Token::new(TokenType::Int, Some("5")),
            Token::new(TokenType::Semicolon, None),
            //2
            Token::new(TokenType::Let, None),
            Token::new(TokenType::Ident, Some("ten")),
            Token::new(TokenType::Assign, None),
            Token::new(TokenType::Int, Some("10")),
            Token::new(TokenType::Semicolon, None),
            //3
            Token::new(TokenType::Let, None),
            Token::new(TokenType::Ident, Some("add")),
            Token::new(TokenType::Assign, None),
            Token::new(TokenType::Function, None),
            Token::new(TokenType::Lparen, None),
            Token::new(TokenType::Ident, Some("x")),
            Token::new(TokenType::Comma, None),
            Token::new(TokenType::Ident, Some("y")),
            Token::new(TokenType::Rparen, None),
            Token::new(TokenType::Lbrace, None),
            Token::new(TokenType::Ident, Some("x")),
            Token::new(TokenType::Plus, None),
            Token::new(TokenType::Ident, Some("y")),
            Token::new(TokenType::Semicolon, None),
            Token::new(TokenType::Rbrace, None),
            Token::new(TokenType::Semicolon, None),
            //4
            Token::new(TokenType::Let, None),
            Token::new(TokenType::Ident, Some("result")),
            Token::new(TokenType::Assign, None),
            Token::new(TokenType::Ident, Some("add")),
            Token::new(TokenType::Lparen, None),
            Token::new(TokenType::Ident, Some("five")),
            Token::new(TokenType::Comma, None),
            Token::new(TokenType::Ident, Some("ten")),
            Token::new(TokenType::Rparen, None),
            Token::new(TokenType::Semicolon, None),
            //5
            Token::new(TokenType::Eof, None),
        ];

        let mut lexer = Lexer::new(input);

        for expected_token in expected {
            let token = lexer.get_next_token();
            assert_eq!(expected_token, token);
        }
    }
}
