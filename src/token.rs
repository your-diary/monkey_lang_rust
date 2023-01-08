use itertools::Itertools;

use super::util;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Eof,
    Ident(String),
    Int(i64),
    Float(f64),
    String(String),
    Char(char),
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Power,
    Invert,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,
    Function,
    Let,
    Return,
    True,
    False,
    If,
    Else,
}

pub fn lookup_token(sequence: &str) -> Result<Token, String> {
    let first_char = sequence.chars().next().unwrap();
    let ret = match sequence {
        "=" => Token::Assign,
        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Asterisk,
        "/" => Token::Slash,
        "%" => Token::Percent,
        "**" => Token::Power,
        "!" => Token::Invert,
        "==" => Token::Eq,
        "!=" => Token::NotEq,
        "<" => Token::Lt,
        ">" => Token::Gt,
        "<=" => Token::LtEq,
        ">=" => Token::GtEq,
        "&&" => Token::And,
        "||" => Token::Or,
        "," => Token::Comma,
        ";" => Token::Semicolon,
        "(" => Token::Lparen,
        ")" => Token::Rparen,
        "{" => Token::Lbrace,
        "}" => Token::Rbrace,
        "[" => Token::Lbracket,
        "]" => Token::Rbracket,
        "fn" => Token::Function,
        "let" => Token::Let,
        "return" => Token::Return,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        _ if (first_char == '\'') => Token::Char(sequence.chars().nth(1).unwrap()),
        _ if (first_char == '"') => {
            let l = sequence.chars().collect_vec();
            Token::String(l.into_iter().skip(1).dropping_back(1).collect())
        }
        _ if util::is_digit(first_char) => {
            if (sequence.contains('.')) {
                match sequence.parse::<f64>() {
                    Err(e) => return Err(e.to_string()),
                    Ok(i) => Token::Float(i),
                }
            } else {
                match sequence.parse::<i64>() {
                    Err(e) => return Err(e.to_string()),
                    Ok(i) => Token::Int(i),
                }
            }
        }
        _ if util::is_identifier(first_char) => Token::Ident(sequence.to_string()),
        _ => unreachable!(),
    };
    Ok(ret)
}
