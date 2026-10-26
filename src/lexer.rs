use crate::token::{Token, TokenType, lookup_ident};

#[derive(Default, Debug)]
struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut s = Self {
            input: input.to_string(),
            ..Default::default()
        };
        s.read_char();
        s
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self
                .input
                .chars()
                .nth(self.read_position)
                .expect("UH OH: Expected a char at this position");
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input
                .chars()
                .nth(self.read_position)
                .expect("Looked ahead")
        }
    }

    fn next_token(&mut self) -> Token {
        let mut tok = Token::default();

        self.skip_whitespace();

        match self.ch {
            '=' => match self.peek_char() {
                '=' => {
                    let ch = self.ch;
                    self.read_char();
                    tok = new_token(TokenType::Eq, &format!("{}{}", ch, self.ch))
                }
                _ => tok = new_token(TokenType::Assign, &self.ch.to_string()),
            },
            ';' => tok = new_token(TokenType::Semicolon, &self.ch.to_string()),
            '(' => tok = new_token(TokenType::LParen, &self.ch.to_string()),
            ')' => tok = new_token(TokenType::RParen, &self.ch.to_string()),
            ',' => tok = new_token(TokenType::Comma, &self.ch.to_string()),
            '+' => tok = new_token(TokenType::Plus, &self.ch.to_string()),
            '-' => tok = new_token(TokenType::Minus, &self.ch.to_string()),
            '!' => match self.peek_char() {
                '=' => {
                    let ch = self.ch;
                    self.read_char();
                    tok = new_token(TokenType::NotEq, &format!("{}{}", ch, self.ch))
                }
                _ => tok = new_token(TokenType::Bang, &self.ch.to_string()),
            },
            '*' => tok = new_token(TokenType::Asterisk, &self.ch.to_string()),
            '/' => tok = new_token(TokenType::Slash, &self.ch.to_string()),
            '<' => tok = new_token(TokenType::Lt, &self.ch.to_string()),
            '>' => tok = new_token(TokenType::Gt, &self.ch.to_string()),
            '{' => tok = new_token(TokenType::LBrace, &self.ch.to_string()),
            '}' => tok = new_token(TokenType::RBrace, &self.ch.to_string()),
            '\0' => {
                tok.literal = "".to_string();
                tok.ttype = TokenType::Eof;
            }
            _ => {
                if self.ch.is_alphabetic() {
                    tok.literal = self.read_identifier();
                    tok.ttype = lookup_ident(&tok.literal);
                    return tok;
                } else if self.ch.is_digit(10) {
                    tok.ttype = TokenType::Int;
                    tok.literal = self.read_number();
                    return tok;
                } else {
                    tok.literal = "".to_string();
                    tok.ttype = TokenType::Illegal;
                }
            }
        }

        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        loop {
            if self.ch.is_alphabetic() || self.ch == '_' {
                self.read_char();
            } else {
                break;
            }
        }
        let temp_str = self.input.as_str();
        temp_str[pos..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;
        loop {
            if self.ch.is_numeric() {
                self.read_char();
            } else {
                break;
            }
        }
        let temp_str = self.input.as_str();
        temp_str[pos..self.position].to_string()
    }
}

fn new_token(token_type: TokenType, literal: &str) -> Token {
    return Token {
        ttype: token_type,
        literal: literal.to_string(),
    };
}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::lexer::Lexer;
    use crate::token::TokenType;

    #[test]
    fn next_token() {
        let input = "let five = 5;
let ten = 10;
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
10 == 10;
10 != 9;
";
        let expected = [
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Gt, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::If, "if"),
            (TokenType::LParen, "("),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Int, "10"),
            (TokenType::Eq, "=="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "10"),
            (TokenType::NotEq, "!="),
            (TokenType::Int, "9"),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(input);
        for (i, (expected_type, expected_literal)) in expected.into_iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(tok.ttype, expected_type, "tests[{i}] - tokentype wrong");
            assert_eq!(tok.literal, expected_literal, "tests[{i}] - literal wrong");
        }
    }
}
