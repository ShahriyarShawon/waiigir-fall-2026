use std::io::{self, Write};

use crate::{lexer::Lexer, token::TokenType};

static PROMPT: &str= ">> ";

pub fn start() {
    print!("{PROMPT}");
    io::stdout().flush().expect("Cant flush :/");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Repl failed to read :o");
    let trimmed = input.trim();
    
    let mut l = Lexer::new(trimmed);

    loop {
        let tok = l.next_token();
        if tok.ttype == TokenType::Eof {
            break;
        }

        println!("{tok}");
    }
}
