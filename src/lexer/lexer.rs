use crate::token::token;
use crate::token::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
    line: i32,
    column: i32,
    block_comment: bool,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 0,
            column: 0,
            block_comment: false,
        };

        lexer
    }

    pub fn get_line(&mut self) -> i32 {
        return self.line;
    }

    pub fn set_input(&mut self, input: String) {
        self.input = input;
        self.position = 0;
        self.read_position = 0;
        self.ch = '\0';
        self.line += 1;
        self.column = 0;

        self.read_char();
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.block_comment {
            self.skip_comment();
        }

        let mut tok = Token {
            r#type: TokenType::ILLEGAL,
            literal: String::from(""),
            line: self.line,
            column: self.column,
        };

        match self.ch {
            ',' => {
                tok.literal = self.ch.to_string();
                tok.r#type = TokenType::COMMA;
            }

            ';' => {
                tok.literal = self.ch.to_string();
                tok.r#type = TokenType::SEMICOLON;
            }

            '(' => {
                tok.literal = self.ch.to_string();
                tok.r#type = TokenType::LPAR;
            }

            ')' => {
                tok.literal = self.ch.to_string();
                tok.r#type = TokenType::RPAR;
            }

            '[' => {
                tok.literal = self.ch.to_string();
                tok.r#type = TokenType::LBRACKET;
            }

            ']' => {
                tok.literal = self.ch.to_string();
                tok.r#type = TokenType::RBRACKET;
            }

            '{' => {
                tok.literal = self.ch.to_string();
                tok.r#type = TokenType::LACC;
            }

            '}' => {
                tok.literal = self.ch.to_string();
                tok.r#type = TokenType::RACC;
            }

            '+' => {
                tok.literal = self.ch.to_string();
                tok.r#type = TokenType::ADD;
            }

            '-' => {
                tok.literal = self.ch.to_string();
                tok.r#type = TokenType::SUB;
            }

            '*' => {
                tok.literal = self.ch.to_string();
                tok.r#type = TokenType::MUL;
            }

            '/' => {
                if self.peek_char() == '/' {
                    self.read_char();
                    self.skip_line_comment();
                    return self.next_token();
                } else if self.peek_char() == '*' {
                    self.read_char();
                    self.block_comment = true;
                    return self.next_token();
                } else {
                    tok.literal = self.ch.to_string();
                    tok.r#type = TokenType::DIV;
                }
            }

            '.' => {
                tok.literal = self.ch.to_string();
                tok.r#type = TokenType::DOT;
            }

            '|' => {
                if self.peek_char() == '|' {
                    let ch = self.ch;
                    self.read_char();
                    tok.literal = format!("{}{}", ch, self.ch);
                    tok.r#type = TokenType::OR;
                }
            }

            '&' => {
                if self.peek_char() == '&' {
                    let ch = self.ch;
                    self.read_char();
                    tok.literal = format!("{}{}", ch, self.ch);
                    tok.r#type = TokenType::AND;
                }
            }

            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok.literal = format!("{}{}", ch, self.ch);
                    tok.r#type = TokenType::NOTEQ;
                } else {
                    tok.literal = self.ch.to_string();
                    tok.r#type = TokenType::NOT;
                }
            }

            '<' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok.literal = format!("{}{}", ch, self.ch);
                    tok.r#type = TokenType::LESSEQ;
                } else {
                    tok.literal = self.ch.to_string();
                    tok.r#type = TokenType::LESS;
                }
            }

            '>' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok.literal = format!("{}{}", ch, self.ch);
                    tok.r#type = TokenType::GREATEREQ;
                } else {
                    tok.literal = self.ch.to_string();
                    tok.r#type = TokenType::GREATER;
                }
            }

            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok.literal = format!("{}{}", ch, self.ch);
                    tok.r#type = TokenType::EQUAL;
                } else {
                    tok.literal = self.ch.to_string();
                    tok.r#type = TokenType::ASSIGN;
                }
            }

            '0' => {
               if self.peek_char() == 'x' {
                   tok.literal = self.read_hex();
                   tok.r#type = TokenType::CT_INT;
               } else if self.is_octal(self.peek_char()) {
                   tok.literal = self.read_octal();
                   tok.r#type = TokenType::CT_INT;
               } else if self.peek_char() == '.' {
                    let int_part = self.read_int();
                    self.read_position += 1;
                    let real_part = self.read_real();

                    match real_part {
                        Some(real_number) => {
                            tok.literal = format!("{int_part}{real_number}");
                            tok.r#type = TokenType::CT_REAL;
                        }    
                        None => {
                            tok.r#type = TokenType::ILLEGAL;
                        }
                    }
               } else {
                    tok.literal = self.ch.to_string();
                    tok.r#type = TokenType::CT_INT;
               }
            }

            '1'..='9' => {
                let int_part = self.read_int().to_string();

                if self.ch == '.' {
                    self.read_position += 1;
                    let real_part = self.read_real();

                    match real_part {
                        Some(real_number) => {
                            tok.literal = format!("{int_part}{real_number}");
                            tok.r#type = TokenType::CT_REAL;
                        }    
                        None => {
                            tok.r#type = TokenType::ILLEGAL;
                        }
                    }
                } else if self.ch == 'e' || self.ch == 'E' {
                    self.read_position += 1;
                    let real_part = self.read_real();

                    match real_part {
                        Some(real_number) => {
                            tok.literal = format!("{int_part}{real_number}");
                            tok.r#type = TokenType::CT_REAL;
                        }    
                        None => {
                            tok.r#type = TokenType::ILLEGAL;
                        }
                    }
                } else {
                    tok.literal = int_part;
                    tok.r#type = TokenType::CT_INT;
                }
            }

            '\'' => {
                let char_literal = self.read_char_literal();

                match char_literal {
                    Some(valid_char_literal) => {
                       tok.literal = format!("{valid_char_literal}");
                       tok.r#type = TokenType::CT_CHAR;
                    }

                    None => {
                        tok.r#type = TokenType::ILLEGAL;
                    }
                }
            }

            '\"' => {
                let string_literal = self.read_string();

                match string_literal {
                    Some(valid_string_literal) => {
                        tok.literal = valid_string_literal;
                        tok.r#type = TokenType::CT_STRING;
                    }

                    None => {
                        tok.r#type = TokenType::ILLEGAL;
                    }
                }
            } 

            'a'..='z' | 'A'..='Z' | '_' => {
               tok.literal = self.read_identifier(); 
               tok.r#type = token::lookup_identifier(&tok.literal.clone());
            }

            '\0' => {
                tok.r#type = TokenType::EOF;
            }

            _ => unreachable!("Not a valid token (for now at least)")
        }

        self.read_char();

        tok
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
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

    fn read_char_literal(&mut self) -> Option<String> {
        let position = self.position;

        // consume ' char
        self.read_char();

        if self.ch == '\\' {
            self.read_char();
            if self.is_escape(self.ch) {
                self.read_char();
            } else {
                return None;
            }
        } else if self.ch == '\'' {
            return None;
        } else if self.ch.is_ascii() {
            self.read_char();
        }

        if self.ch == '\'' {
            self.read_char();
        } else {
            return None;
        }

        self.read_position -= 1;
        Some(self.input[position + 1..self.position - 1].to_string())
    }

    fn read_string(&mut self) -> Option<String> {
        let position = self.position;

        //consume " char
        self.read_char();

        while let Some(ch) = self.input.chars().nth(self.position) {
            if ch == '\\' {
                self.read_char();
                if self.is_escape(ch) {
                    self.read_char();
                } else {
                    return None;
                }
            } else if ch == '"' {
                if self.peek_char() == '"' {
                    return None;
                }
                self.read_char();

                self.read_position -= 1;
                return Some(self.input[position + 1..self.position - 1].to_string());
            } else if ch.is_ascii() {
                self.read_char();
            } else {
                return None;
            }
        }

        None
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        } 

        self.read_position -= 1;
        self.input[position..self.position].to_string()
    }
    fn read_int(&mut self) -> String {
        let position = self.position;
        while self.is_decimal(self.ch) {
            self.read_char();
        }

        self.read_position -= 1;

        self.input[position..self.position].to_string()
    }

    fn read_octal(&mut self) -> String {
        let position = self.position;
        while self.is_octal(self.ch) {
            self.read_char();
        }

        self.read_position -= 1;
        self.input[position..self.position].to_string()
    }

    fn read_hex(&mut self) -> String {
        let position = self.position;

        self.read_char(); 
        self.read_char();
        
        while self.is_hex(self.ch) {
            self.read_char();
        }

        self.read_position -= 1;
        self.input[position..self.position].to_string()
    }

    fn read_real(&mut self) -> Option<String> {
        let position = self.position;

        if self.ch == '.' {
            self.read_char();

            let mut encountered_digits = false;

            while self.is_decimal(self.ch) {
                encountered_digits = true;
                self.read_char();
            }

            if !encountered_digits {
                return None;
            }
        }


        if self.ch == 'e' || self.ch == 'E' {
            self.read_char();

            let mut encountered_exp_digits = false;

            if self.ch == '+' || self.ch == '-' {
                self.read_char();
            }

            while self.is_decimal(self.ch) {
                encountered_exp_digits = true;
                self.read_char();
            }

            if !encountered_exp_digits {
                return None;
            }
        }

        self.read_position -= 1;
        Some(self.input[position..self.position].to_string())
    }

    fn is_escape(&self, ch: char) -> bool {
        matches!(ch, 'a' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' | '\'' | '?' | '\"' | '\\' | '0')    
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

    fn skip_line_comment(&mut self) {
        while self.ch != '\n' && self.ch != '\r' && self.ch != '\0' {
            self.read_char();
        }
    }

    fn skip_comment(&mut self) {
        while !(self.ch == '*' && self.peek_char() == '/') {
            if self.ch == '\0' {
                return;
            }
            self.read_char();
        }

        if self.ch == '*' && self.peek_char() == '/' {
            self.read_char(); 
            self.read_char(); 

            self.block_comment = false;
        }
    }
}
