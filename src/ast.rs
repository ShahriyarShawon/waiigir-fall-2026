use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug, Default)]
pub enum Expression {
    #[default]
    NullExpression,
    Identifier(Identifier)
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(ie) => ie.token.literal.clone(),
            _ => String::new()
        }
    }
}


#[derive(Debug, Default)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}


pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement)
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(ls) => ls.token.literal.clone(),
            Statement::Return(rs) => rs.token.literal.clone()
        }
    }
}


#[derive(Debug, Default)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug, Default)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}


pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }
}

