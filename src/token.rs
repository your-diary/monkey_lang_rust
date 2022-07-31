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
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
}

impl TokenType {
    pub fn value(&self) -> &str {
        match self {
            TokenType::Illegal => "ILLEGAL",
            TokenType::Eof => "EOF",
            TokenType::Ident => "IDENT",
            TokenType::Int => "INT",
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::Lparen => "(",
            TokenType::Rparen => ")",
            TokenType::Lbrace => "{",
            TokenType::Rbrace => "}",
            TokenType::Function => "FUNCTION",
            TokenType::Let => "LET",
        }
    }
}

pub fn lookup_token_type(identifier: &str) -> TokenType {
    match identifier {
        "let" => TokenType::Let,
        "fn" => TokenType::Function,
        _ => TokenType::Ident,
    }
}
