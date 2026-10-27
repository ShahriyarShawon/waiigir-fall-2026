use std::fmt;
use crate::ast::{Expression};

#[derive(Debug, PartialEq, Default, Clone)]
pub enum TokenType {
    #[default]
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Eq,
    NotEq,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tt = match self {
            TokenType::Illegal => "ILLEGAL",
            TokenType::Eof => "EOF",
            TokenType::Ident => "IDENT",
            TokenType::Int => "INT",
            TokenType::Assign => "=",
            TokenType::Eq => "==",
            TokenType::NotEq => "!=",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Bang => "!",
            TokenType::Asterisk => "*",
            TokenType::Slash => "/",
            TokenType::Lt => "<",
            TokenType::Gt => ">",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::LParen => "(",
            TokenType::RParen => ")",
            TokenType::LBrace => "{",
            TokenType::RBrace => "}",
            TokenType::Function => "FUNCTION",
            TokenType::Let => "LET",
            TokenType::True => "TRUE",
            TokenType::False => "FALSE",
            TokenType::If => "IF",
            TokenType::Else => "ELSE",
            TokenType::Return => "RETURN",
        };
        write!(f, "{tt}")
    }

}

#[derive(Debug, Default, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token - Type: {} - Literal: {}", self.ttype, self.literal)
    }

}


fn keywords(key: &str) -> TokenType {
    match key {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
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
