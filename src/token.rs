use std::fmt;

#[derive(Debug, PartialEq, Default)]
pub enum TokenType {
    #[default]
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tt = match self {
            TokenType::Illegal => "ILLEGAL",
            TokenType::Eof => "EOF",
            TokenType::Ident => "IDENT",
            TokenType::Int => "INT",
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::LParen => "(",
            TokenType::RParen => ")",
            TokenType::LBrace => "{",
            TokenType::RBrace => "}",
            TokenType::Function => "FUNCTION",
            TokenType::Let => "LET"
        };
        write!(f, "{tt}")
    }

}

#[derive(Debug, Default)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String
}


fn keywords(key: &str) -> TokenType {
    match key {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        _  => TokenType::Illegal
    }
}


pub fn lookup_ident(ident: &str) -> TokenType {
    let res = keywords(ident);
    match res {
        TokenType::Illegal => TokenType::Ident,
        _ => res
    }

}
