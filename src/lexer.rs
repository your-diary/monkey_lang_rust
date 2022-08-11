use std::collections::HashMap;

use super::token::{self, Token};
use super::util;

pub type LexerResult<T> = Result<T, String>;

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

    fn read_number(&mut self) -> LexerResult<String> {
        let position = self.position;
        while (self.ch.is_some() && util::is_number(self.ch.unwrap())) {
            self.read_next_char();
        }
        let l = &self.input[position..self.position];
        if (l.iter().filter(|c| (**c == '.')).count() >= 2) {
            return Err("two or more dot found in a number literal".to_string());
        }
        if ((l.len() == 1) && (l[0] == '.')) {
            return Err("isolated `.` found".to_string());
        }
        Ok(l.iter().collect())
    }

    fn read_string(&mut self) -> LexerResult<String> {
        let mut l = vec!['"'];
        loop {
            self.read_next_char();
            if (self.ch.is_none()) {
                return Err("unexpected end of a string literal".to_string());
            }
            if (self.ch.unwrap() == '"') {
                l.push('"');
                break;
            }
            l.push(match self.ch.unwrap() {
                '\\' => {
                    self.read_next_char();
                    if (self.ch.is_none()) {
                        return Err("unexpected end of a string literal".to_string());
                    }
                    util::parse_escaped_character(self.ch.unwrap())
                }
                c => c,
            });
        }
        self.read_next_char();
        Ok(l.iter().collect())
    }

    fn read_character(&mut self) -> LexerResult<String> {
        self.read_next_char();
        if (self.ch.is_none() || (self.ch.unwrap() == '\'')) {
            return Err("unexpected end of a character literal".to_string());
        }
        let ret = match self.ch.unwrap() {
            '\\' => {
                self.read_next_char();
                if (self.ch.is_none()) {
                    return Err("unexpected end of a character literal".to_string());
                }
                vec!['\'', util::parse_escaped_character(self.ch.unwrap()), '\'']
                    .iter()
                    .collect()
            }
            c => format!("'{}'", c),
        };
        self.read_next_char();
        if (self.ch.is_none()) {
            return Err("unexpected end of a character literal".to_string());
        }
        if (self.ch.unwrap() != '\'') {
            return Err("character literal can contain only one character".to_string());
        }
        self.read_next_char();
        Ok(ret)
    }

    pub fn get_next_token(&mut self) -> LexerResult<Token> {
        self.eat_whitespace();
        if (self.ch.is_none()) {
            return Ok(Token::Eof);
        }
        let sequence: String = match self.ch.unwrap() {
            c if util::is_number(c) => self.read_number()?,
            c if util::is_identifier(c) => self.read_identifier(), //this includes a keyword such as `if`
            '"' => self.read_string()?,
            '\'' => self.read_character()?,
            //operators
            c => {
                let m = HashMap::from([
                    ('=', "=="),
                    ('!', "!="),
                    ('*', "**"),
                    ('>', ">="),
                    ('<', "<="),
                    ('&', "&&"),
                    ('|', "||"),
                ]);
                let ret = match c {
                    '=' | '!' | '*' | '>' | '<' => {
                        if let Some(next_ch) = self.peek_next_char() {
                            let s = m.get(&self.ch.unwrap()).unwrap();
                            if (next_ch == s.chars().nth(1).unwrap()) {
                                self.read_next_char();
                                s.to_string()
                            } else {
                                c.to_string()
                            }
                        } else {
                            c.to_string()
                        }
                    }
                    '&' | '|' => {
                        let next_ch = self.peek_next_char();
                        if (next_ch.is_none()) {
                            return Err("unexpected end of input".to_string());
                        }
                        let s = m.get(&self.ch.unwrap()).unwrap();
                        if (next_ch.unwrap() != s.chars().nth(1).unwrap()) {
                            return Err(format!("`{}` expected but not found", s));
                        }
                        self.read_next_char();
                        s.to_string()
                    }
                    c => c.to_string(),
                };
                self.read_next_char(); //moves to the next position as `read_identifier()` does
                ret
            }
        };
        match token::lookup_token(&sequence) {
            None => Err(format!(
                "not yet implemented or a lexer bug: `{}`",
                sequence
            )),
            Some(t) => match t {
                Token::Ident(_) => Ok(Token::Ident(sequence)),
                Token::Float(_) => {
                    if (sequence.contains('.')) {
                        match sequence.parse::<f64>() {
                            Err(e) => Err(e.to_string()),
                            Ok(i) => Ok(Token::Float(i)),
                        }
                    } else {
                        match sequence.parse::<i32>() {
                            Err(e) => Err(e.to_string()),
                            Ok(i) => Ok(Token::Int(i)),
                        }
                    }
                }
                Token::String(_) => {
                    let l: Vec<char> = sequence.chars().collect();
                    Ok(Token::String(l[1..l.len() - 1].iter().collect()))
                }
                Token::Char(_) => Ok(Token::Char(sequence.chars().nth(1).unwrap())),
                t => Ok(t),
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use super::super::token::Token;
    use super::Lexer;

    #[test]
    fn test01() {
        let input = r#"
            = + ( ) { } , ;
            * **
            %
            > >=
            < <=
            &&
            ||
        "#;

        let expected = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
            Token::Asterisk,
            Token::Power,
            Token::Percent,
            Token::Gt,
            Token::GtEq,
            Token::Lt,
            Token::LtEq,
            Token::And,
            Token::Or,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);

        for expected_token in expected {
            let token = lexer.get_next_token();
            assert!(token.is_ok());
            assert_eq!(expected_token, token.unwrap());
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
            assert!(token.is_ok());
            assert_eq!(expected_token, token.unwrap());
        }
    }

    #[test]
    fn test03() {
        let input = vec![
            //1
            r#"1"#,
            r#"1."#,
            r#"."#,
            r#".0"#,
            r#"3.14"#,
            r#"1.2.3"#,
            r#"1.2.3.4"#,
            //2
            r#"""#,
            r#""abc"#,
            r#""""#,
            r#""a""#,
            r#""ab""#,
            r#""a b""#,
            r#""あ""#,
            r#""あい""#,
            r#""\"#,
            r#""\""#,
            r#""\n""#,
            r#""\n\t""#,
            r#""\n\"""#,
            //3
            r#"'"#,
            r#"'a"#,
            r#"''"#,
            r#"'ab'"#,
            r#"'a'"#,
            r#"'あ'"#,
            r#"'\"#,
            r#"'\'"#,
            r#"'\\'"#,
            r#"'\''"#,
            r#"'\"'"#,
            r#"'\0'"#,
            r#"'\n'"#,
            r#"'\t'"#,
            r#"'\r'"#,
            r#"'\p'"#,
        ];

        let expected = vec![
            //1
            Ok(Token::Int(1)),
            Ok(Token::Float(1.0)),
            Err("isolated `.` found".to_string()),
            Ok(Token::Float(0.0)),
            Ok(Token::Float(3.14)),
            Err("two or more".to_string()),
            Err("two or more".to_string()),
            //2
            Err("unexpected end".to_string()),
            Err("unexpected end".to_string()),
            Ok(Token::String("".to_string())),
            Ok(Token::String("a".to_string())),
            Ok(Token::String("ab".to_string())),
            Ok(Token::String("a b".to_string())),
            Ok(Token::String("あ".to_string())),
            Ok(Token::String("あい".to_string())),
            Err("unexpected end".to_string()),
            Err("unexpected end".to_string()),
            Ok(Token::String("\n".to_string())),
            Ok(Token::String("\n\t".to_string())),
            Ok(Token::String("\n\"".to_string())),
            //3
            Err("unexpected end".to_string()),
            Err("unexpected end".to_string()),
            Err("unexpected end".to_string()),
            Err("only one".to_string()),
            Ok(Token::Char('a')),
            Ok(Token::Char('あ')),
            Err("unexpected end".to_string()),
            Err("unexpected end".to_string()),
            Ok(Token::Char('\\')),
            Ok(Token::Char('\'')),
            Ok(Token::Char('"')),
            Ok(Token::Char('\0')),
            Ok(Token::Char('\n')),
            Ok(Token::Char('\t')),
            Ok(Token::Char('\r')),
            Ok(Token::Char('p')),
        ];

        assert_eq!(input.len(), expected.len());

        for i in 0..input.len() {
            println!("[{}]", input[i]);
            let mut lexer = Lexer::new(input[i]);
            match lexer.get_next_token() {
                Ok(t) => assert_eq!(expected[i], Ok(t)),
                Err(t) => match &expected[i] {
                    Ok(_) => assert!(expected[i].is_err()),
                    Err(e) => assert!(t.contains(e)),
                },
            }
        }
    }
}
