use crate::{
    ast::{Expression, Identifier, LetStatement, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

#[derive(Debug, Default)]
pub struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut p = Parser {
            l: lexer,
            ..Default::default()
        };

        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        loop {
            if self.cur_token.ttype != TokenType::Eof {
                if let Some(s) = self.parse_statement() {
                    program.statements.push(s);
                };
                self.next_token();
            } else {
                break;
            }
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.ttype {
            TokenType::Let => {
                match self.parse_let_statement() {
                    Some(ls) => Some(Statement::Let(ls)),
                    None => None
                }
            },
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let mut stmt = LetStatement {
            token: self.cur_token.clone(),
            ..Default::default()
        };

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        stmt.name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        // skip expressions
        loop {
            if self.cur_token_is(TokenType::Semicolon) {
                self.next_token();
            } else {
                break;
            }
        }

        Some(stmt)
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.ttype == t
    }

    fn peek_token_is(&self, t: &TokenType) -> bool {
        &self.peek_token.ttype == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(&t) {
            self.next_token();
            return true
        } else {
            self.peek_error(&t);
            return false
        }
    }

    fn peek_error(&mut self, t: &TokenType) {
        let msg = format!("expected next token to be {}, got {} instead", t, self.peek_token.ttype);
        self.errors.push(msg);
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Node, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn let_statements() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements"
        );

        let expected = ["x", "y", "foobar"];
        for (stmt, name) in program.statements.iter().zip(expected) {
            test_let_statement(stmt, name);
        }
    }

    fn check_parser_errors(parser: &Parser) {
        assert!(
            parser.errors.is_empty(),
            "parser has {} errors: {:#?}",
            parser.errors.len(),
            parser.errors
        );
    }

    fn test_let_statement(stmt: &Statement, name: &str) {
        assert_eq!(stmt.token_literal(), "let", "stmt.token_literal not 'let'");

        let Statement::Let(let_stmt) = stmt;
        assert_eq!(
            let_stmt.name.value, name,
            "let_stmt.name.value not '{name}'"
        );
        assert_eq!(
            let_stmt.name.token.literal, name,
            "let_stmt.name not '{name}'"
        );
    }
}
