use super::util;

#[derive(Debug, PartialEq)]
pub struct Token {
    tp: TokenType, //`type` is a reverved word
    literal: Option<String>,
}

impl Token {
    pub fn new(tp: TokenType, literal: Option<&str>) -> Self {
        Token {
            tp,
            literal: literal.map(String::from),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Invert,
    Lt,
    Gt,
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

pub fn lookup_token_type(sequence: &str) -> TokenType {
    let first_char = sequence.chars().next().unwrap();
    match sequence {
        "\0" => TokenType::Eof,
        "=" => TokenType::Assign,
        "+" => TokenType::Plus,
        "-" => TokenType::Minus,
        "*" => TokenType::Asterisk,
        "/" => TokenType::Slash,
        "!" => TokenType::Invert,
        "<" => TokenType::Lt,
        ">" => TokenType::Gt,
        "," => TokenType::Comma,
        ";" => TokenType::Semicolon,
        "(" => TokenType::Lparen,
        ")" => TokenType::Rparen,
        "{" => TokenType::Lbrace,
        "}" => TokenType::Rbrace,
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "return" => TokenType::Return,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        _ if util::is_letter(first_char) => TokenType::Ident,
        _ if first_char.is_ascii_digit() => TokenType::Int,
        _ => TokenType::Illegal,
    }
}
