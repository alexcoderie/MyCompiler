use std::num::ParseIntError;

use crate::token::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current_token_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let mut parser = Parser {
            tokens,
            current_token_index: 0,
        };

        parser
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current_token_index)
    }

    fn consume(&mut self) {
        self.current_token_index += 1;
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.current_token_index + 1)
    }

}
