use super::util;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Eof,
    Ident(String),
    Int(i32),
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
    Function,
    Let,
    Return,
    True,
    False,
    If,
    Else,
}

pub fn lookup_token(sequence: &str) -> Option<Token> {
    let first_char = sequence.chars().next().unwrap();
    match sequence {
        "\0" => Some(Token::Eof),
        "=" => Some(Token::Assign),
        "+" => Some(Token::Plus),
        "-" => Some(Token::Minus),
        "*" => Some(Token::Asterisk),
        "/" => Some(Token::Slash),
        "%" => Some(Token::Percent),
        "**" => Some(Token::Power),
        "!" => Some(Token::Invert),
        "==" => Some(Token::Eq),
        "!=" => Some(Token::NotEq),
        "<" => Some(Token::Lt),
        ">" => Some(Token::Gt),
        "<=" => Some(Token::LtEq),
        ">=" => Some(Token::GtEq),
        "&&" => Some(Token::And),
        "||" => Some(Token::Or),
        "," => Some(Token::Comma),
        ";" => Some(Token::Semicolon),
        "(" => Some(Token::Lparen),
        ")" => Some(Token::Rparen),
        "{" => Some(Token::Lbrace),
        "}" => Some(Token::Rbrace),
        "fn" => Some(Token::Function),
        "let" => Some(Token::Let),
        "return" => Some(Token::Return),
        "true" => Some(Token::True),
        "false" => Some(Token::False),
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        _ if (first_char == '\'') => Some(Token::Char('\0')),
        _ if (first_char == '"') => Some(Token::String(String::new())),
        _ if util::is_number(first_char) => Some(Token::Float(0.0)),
        _ if util::is_identifier(first_char) => Some(Token::Ident(String::new())),
        _ => None,
    }
}
