use crate::token::token;
use crate::token::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
    line: i32,
    column: i32,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 1,
            column: 1,
        };

        lexer.read_char();
        lexer
    }
}
