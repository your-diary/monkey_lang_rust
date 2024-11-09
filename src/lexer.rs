use std::collections::{HashMap, VecDeque};

use super::token::{self, Token};
use super::util;

pub type LexerResult<T> = Result<T, String>;

pub struct Lexer {
    queue: VecDeque<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            queue: input.to_string().chars().collect(),
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut l = vec![];
        while !self.queue.is_empty() && util::is_identifier(self.queue[0]) {
            l.push(self.queue.pop_front().unwrap());
        }
        l.into_iter().collect()
    }

    fn read_number(&mut self) -> LexerResult<String> {
        let mut l = vec![];
        while !self.queue.is_empty() && util::is_digit(self.queue[0]) {
            l.push(self.queue.pop_front().unwrap());
        }
        if l.iter().filter(|c| (**c == '.')).count() >= 2 {
            return Err("two or more dots found in a number literal".to_string());
        } else if (l.len() == 1) && (l[0] == '.') {
            return Err("isolated `.` found".to_string());
        }
        Ok(l.into_iter().collect())
    }

    fn read_string(&mut self) -> LexerResult<String> {
        let mut l = vec![self.queue.pop_front().unwrap()];
        assert_eq!('"', l[0]);
        loop {
            if self.queue.is_empty() {
                return Err("unexpected end of a string literal".to_string());
            }
            let next = self.queue.pop_front().unwrap();
            if next == '"' {
                l.push(next);
                break;
            }
            let c = match next {
                '\\' => {
                    if self.queue.is_empty() {
                        return Err("unexpected end of a string literal".to_string());
                    }
                    match util::parse_escaped_character(self.queue.pop_front().unwrap()) {
                        None => return Err("unknown escape sequence found".to_string()),
                        Some(c) => c,
                    }
                }
                c => c,
            };
            l.push(c);
        }
        Ok(l.into_iter().collect())
    }

    fn read_character(&mut self) -> LexerResult<String> {
        assert_eq!('\'', self.queue.pop_front().unwrap());
        if self.queue.is_empty() {
            return Err("unexpected end of a character literal".to_string());
        } else if self.queue[0] == '\'' {
            return Err("character literal is empty".to_string());
        }
        let ret = match self.queue.pop_front().unwrap() {
            '\\' => {
                if self.queue.is_empty() {
                    return Err("unexpected end of a character literal".to_string());
                }
                format!(
                    "'{}'",
                    match util::parse_escaped_character(self.queue.pop_front().unwrap()) {
                        None => return Err("unknown escape sequence found".to_string()),
                        Some(c) => c,
                    }
                )
            }
            c => format!("'{}'", c),
        };
        if self.queue.is_empty() {
            return Err("unexpected end of a character literal".to_string());
        } else if self.queue[0] != '\'' {
            return Err("character literal can contain only one character".to_string());
        }
        self.queue.pop_front().unwrap();
        Ok(ret)
    }

    pub fn get_next_token(&mut self) -> LexerResult<Token> {
        //eats whitespace
        while !self.queue.is_empty() && self.queue[0].is_ascii_whitespace() {
            self.queue.pop_front().unwrap();
        }
        if self.queue.is_empty() {
            return Ok(Token::Eof);
        }
        let sequence: String = match self.queue[0] {
            c if util::is_digit(c) => self.read_number()?,
            c if util::is_identifier(c) => self.read_identifier(), //this includes keywords such as `if`
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
                let cur = self.queue.pop_front().unwrap();
                let ret = match c {
                    '=' | '!' | '*' | '>' | '<' => {
                        if self.queue.is_empty() {
                            c.to_string()
                        } else {
                            let s = m[&cur];
                            if self.queue[0] == s.chars().nth(1).unwrap() {
                                self.queue.pop_front().unwrap();
                                s.to_string()
                            } else {
                                c.to_string()
                            }
                        }
                    }
                    '&' | '|' => {
                        let s = m[&cur];
                        if self.queue.is_empty() {
                            return Err(format!("`{}` expected but not found", s));
                        }
                        let next = self.queue.pop_front().unwrap();
                        if next != s.chars().nth(1).unwrap() {
                            return Err(format!("`{}` expected but not found", s));
                        }
                        s.to_string()
                    }
                    c => c.to_string(),
                };
                ret
            }
        };
        token::lookup_token(&sequence)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    // #[ignore]
    fn test_eat_whitespace_and_eof() {
        let input = r#"
            3
        "#;
        let mut lexer = Lexer::new(input);
        assert_eq!(Ok(Token::Int(3)), lexer.get_next_token());
        assert_eq!(Ok(Token::Eof), lexer.get_next_token());
        assert_eq!(Ok(Token::Eof), lexer.get_next_token());
    }

    fn test(input: &str, expected: &[LexerResult<Token>]) {
        let mut lexer = Lexer::new(input);
        for i in 0..expected.len() {
            println!("i = {}", i);
            assert_eq!(expected[i], lexer.get_next_token());
        }
    }

    #[test]
    // #[ignore]
    fn test_integer() {
        let input = r#"
            -1 0 1
        "#;
        let expected = vec![
            Ok(Token::Minus),
            Ok(Token::Int(1)),
            Ok(Token::Int(0)),
            Ok(Token::Int(1)),
            Ok(Token::Eof),
        ];
        test(input, &expected);
    }

    #[test]
    // #[ignore]
    fn test_float_01() {
        let input = r#"
            -3.14 .3 1.
        "#;
        let expected = vec![
            Ok(Token::Minus),
            Ok(Token::Float(3.14)),
            Ok(Token::Float(0.3)),
            Ok(Token::Float(1.0)),
            Ok(Token::Eof),
        ];
        test(input, &expected);
    }

    #[test]
    // #[ignore]
    fn test_float_02() {
        let input = r#"
            . 1.2.3 1.2.3.4
        "#;
        let expected = vec![
            Err("isolated `.` found".to_string()),
            Err("two or more dots found in a number literal".to_string()),
            Err("two or more dots found in a number literal".to_string()),
            Ok(Token::Eof),
        ];
        test(input, &expected);
    }

    #[test]
    // #[ignore]
    fn test_identifier() {
        let input = r#"
            apple bear2 cow3
        "#;
        let expected = vec![
            Ok(Token::Ident("apple".to_string())),
            Ok(Token::Ident("bear2".to_string())),
            Ok(Token::Ident("cow3".to_string())),
            Ok(Token::Eof),
        ];
        test(input, &expected);
    }

    #[test]
    // #[ignore]
    fn test_string_01() {
        let input = r#"
            "" "apple" "apple\\bear\ncow" "こんにちは"
        "#;
        let expected = vec![
            Ok(Token::String("".to_string())),
            Ok(Token::String("apple".to_string())),
            Ok(Token::String("apple\\bear\ncow".to_string())),
            Ok(Token::String("こんにちは".to_string())),
            Ok(Token::Eof),
        ];
        test(input, &expected);
    }

    #[test]
    // #[ignore]
    fn test_string_02() {
        let input = r#"
            "
        "#;
        let expected = vec![
            Err("unexpected end of a string literal".to_string()),
            Ok(Token::Eof),
        ];
        test(input, &expected);

        let input = r#"
            "apple
        "#;
        let expected = vec![
            Err("unexpected end of a string literal".to_string()),
            Ok(Token::Eof),
        ];
        test(input, &expected);

        let input = r#"
            "\p"
        "#;
        let expected = vec![Err("unknown escape sequence found".to_string())];
        test(input, &expected);

        let input = r#"
            "\"
        "#;
        let expected = vec![
            Err("unexpected end of a string literal".to_string()),
            Ok(Token::Eof),
        ];
        test(input, &expected);
    }

    #[test]
    // #[ignore]
    fn test_character_01() {
        let input = r#"
            'c' '\n' 'あ'
        "#;
        let expected = vec![
            Ok(Token::Char('c')),
            Ok(Token::Char('\n')),
            Ok(Token::Char('あ')),
            Ok(Token::Eof),
        ];
        test(input, &expected);
    }

    #[test]
    // #[ignore]
    fn test_character_02() {
        let input = r#"'"#;
        let expected = vec![
            Err("unexpected end of a character literal".to_string()),
            Ok(Token::Eof),
        ];
        test(input, &expected);

        let input = r#"
            ''
        "#;
        let expected = vec![Err("character literal is empty".to_string())];
        test(input, &expected);

        let input = r#"'\"#;
        let expected = vec![
            Err("unexpected end of a character literal".to_string()),
            Ok(Token::Eof),
        ];
        test(input, &expected);

        let input = r#"
            '\p'
        "#;
        let expected = vec![Err("unknown escape sequence found".to_string())];
        test(input, &expected);

        let input = r#"'a"#;
        let expected = vec![
            Err("unexpected end of a character literal".to_string()),
            Ok(Token::Eof),
        ];
        test(input, &expected);

        let input = r#"
            'ab'
        "#;
        let expected = vec![Err(
            "character literal can contain only one character".to_string()
        )];
        test(input, &expected);
    }

    #[test]
    // #[ignore]
    fn test_keywords() {
        let input = r#"
            true false fn let return if else
        "#;
        let expected = vec![
            Ok(Token::True),
            Ok(Token::False),
            Ok(Token::Function),
            Ok(Token::Let),
            Ok(Token::Return),
            Ok(Token::If),
            Ok(Token::Else),
            Ok(Token::Eof),
        ];
        test(input, &expected);
    }

    #[test]
    // #[ignore]
    fn test_operators_01() {
        let input = r#"
            = + - * / % ** ! == != < > <= >= && || , ; () { } [ ]
        "#;
        let expected = vec![
            Ok(Token::Assign),
            Ok(Token::Plus),
            Ok(Token::Minus),
            Ok(Token::Asterisk),
            Ok(Token::Slash),
            Ok(Token::Percent),
            Ok(Token::Power),
            Ok(Token::Invert),
            Ok(Token::Eq),
            Ok(Token::NotEq),
            Ok(Token::Lt),
            Ok(Token::Gt),
            Ok(Token::LtEq),
            Ok(Token::GtEq),
            Ok(Token::And),
            Ok(Token::Or),
            Ok(Token::Comma),
            Ok(Token::Semicolon),
            Ok(Token::Lparen),
            Ok(Token::Rparen),
            Ok(Token::Lbrace),
            Ok(Token::Rbrace),
            Ok(Token::Lbracket),
            Ok(Token::Rbracket),
            Ok(Token::Eof),
        ];
        test(input, &expected);

        let input = r#"("#;
        let expected = vec![Ok(Token::Lparen), Ok(Token::Eof)];
        test(input, &expected);
    }

    #[test]
    // #[ignore]
    fn test_operators_02() {
        let input = r#"
            &+
        "#;
        let expected = vec![
            Err("`&&` expected but not found".to_string()),
            Ok(Token::Eof),
        ];
        test(input, &expected);

        let input = r#"&"#;
        let expected = vec![
            Err("`&&` expected but not found".to_string()),
            Ok(Token::Eof),
        ];
        test(input, &expected);
    }

    #[test]
    fn test_misc_01() {
        let input = r#"
            3x 3.y 3.14z
        "#;
        let expected = vec![
            Ok(Token::Int(3)),
            Ok(Token::Ident("x".to_string())),
            Ok(Token::Float(3.0)),
            Ok(Token::Ident("y".to_string())),
            Ok(Token::Float(3.14)),
            Ok(Token::Ident("z".to_string())),
            Ok(Token::Eof),
        ];
        test(input, &expected);
    }

    #[test]
    fn test_misc_02() {
        let input = r#"
            let add=fn(x,y){x+y;};
        "#;
        let expected = vec![
            Ok(Token::Let),
            Ok(Token::Ident("add".to_string())),
            Ok(Token::Assign),
            Ok(Token::Function),
            Ok(Token::Lparen),
            Ok(Token::Ident("x".to_string())),
            Ok(Token::Comma),
            Ok(Token::Ident("y".to_string())),
            Ok(Token::Rparen),
            Ok(Token::Lbrace),
            Ok(Token::Ident("x".to_string())),
            Ok(Token::Plus),
            Ok(Token::Ident("y".to_string())),
            Ok(Token::Semicolon),
            Ok(Token::Rbrace),
            Ok(Token::Semicolon),
            Ok(Token::Eof),
        ];
        test(input, &expected);
    }
}
