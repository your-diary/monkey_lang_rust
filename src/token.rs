use super::util;

#[derive(Debug, PartialEq, Clone)]
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
    pub fn tp(&self) -> &TokenType {
        &self.tp
    }
    pub fn literal(&self) -> &Option<String> {
        &self.literal
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    Eq,
    NotEq,
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
        "==" => TokenType::Eq,
        "!=" => TokenType::NotEq,
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
        _ if first_char.is_ascii_digit() => TokenType::Int,
        _ if util::is_identifier(first_char) => TokenType::Ident,
        _ => TokenType::Illegal,
    }
}
