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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let mut tok = Token {
            r#type: TokenType::ILLEGAL,
            literal: String::from(""),
            line: self.line,
            column: self.column,
        };

        match self.ch {
            '0' => {
               if self.peek_char() == 'x' {
                   tok.literal = self.read_hex();
                   tok.r#type = TokenType::CT_INT;
               } else if self.is_octal(self.peek_char()) {
                   tok.literal = self.read_octal();
                   tok.r#type = TokenType::CT_INT;
               } else {
                    tok.literal = self.ch.to_string();
                    tok.r#type = TokenType::CT_INT;
               }
            }

            '1'..='9' => {
                tok.literal = self.read_int();
                if tok.literal.contains('.') {
                    tok.r#type = TokenType::CT_REAL;
                } else {
                    tok.r#type = TokenType::CT_INT;
                }

            }
            
            _ => unreachable!("Not a valid token (for now at least)")
        }

        self.read_char();

        tok
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
        if self.ch == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    fn is_letter(&self, ch: char) -> bool {
        (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_'
    }
    
    fn is_decimal(&self, ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn is_octal(&self, ch: char) -> bool {
        ch >= '0' && ch <= '7'
    }
    
    fn is_hex(&self, ch: char) -> bool {
        (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f') || (ch >= 'A' && ch <= 'F')
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_int(&mut self) -> String {
        let position = self.position;
        while self.is_decimal(self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn read_octal(&mut self) -> String {
        let position = self.position;
        while self.is_octal(self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn read_hex(&mut self) -> String {
        let position = self.position;

        self.read_char(); 
        self.read_char();
        
        while self.is_hex(self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }
}
