use std::{fmt::Pointer, num::ParseIntError};

use crate::token::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<TokenType>,
    current_token_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Parser {
        let mut parser = Parser {
            tokens,
            current_token_index: 0,
        };

        parser
    }

    pub fn start() {

    }

    fn current_token(&self) -> Option<&TokenType> {
        self.tokens.get(self.current_token_index)
    }

    fn consume(&mut self) {
        self.current_token_index += 1;
    }

    fn peek_token(&self) -> Option<&TokenType> {
        self.tokens.get(self.current_token_index + 1)
    }

    fn type_base(&mut self) -> bool {
        if let Some(token) = self.current_token() {
            let start_token = self.current_token_index;
            match *token {
                TokenType::INT => {
                    self.consume();
                    true
                }

                TokenType::DOUBLE => {
                    self.consume();
                    true
                }

                TokenType::CHAR => {
                    self.consume();
                    true
                }

                TokenType::STRUCT => {
                    self.consume();

                    if let Some(TokenType::ID) = self.current_token() {
                        self.consume();
                        true
                    } else {
                        self.current_token_index = start_token;
                        println!("Missing identifier!");
                        false
                    }
                }

                _ => {
                    println!("Should be INT, DOUBLE, CHAR or STRUCT");
                    false
                }
            }
        } else {
            false
        }
    }

    pub fn decl_struct(&mut self) -> bool {
        if let Some(token) = self.current_token() {
            let start_token = self.current_token_index;

            match *token {
                TokenType::STRUCT => {
                    self.consume();

                    if let Some(TokenType::ID) = self.current_token() {
                        self.consume();
                        
                        if let Some(TokenType::LACC) = self.current_token() {
                            self.consume();

                            loop {
                                if !self.decl_var() {
                                    break;
                                } else {
                                    continue;
                                }
                            }

                            if let Some(TokenType::RACC) = self.current_token() {
                                self.consume();

                                if let Some(TokenType::SEMICOLON) = self.current_token() {
                                    self.consume();
                                    true
                                } else {
                                    self.current_token_index = start_token;
                                    println!("Missing ';'");
                                    false
                                }
                            } else {
                                self.current_token_index = start_token;
                                println!("Missing ')'");
                                false
                            }
                        } else {
                            self.current_token_index = start_token;
                            println!("Missing '('!");
                            false
                        }
                    } else {
                        self.current_token_index = start_token;
                        println!("Missing identifier!");
                        false
                    }
                }

                _ => {
                    println!("Missing struct keyword");
                    false
                }
            }
        } else {
            println!("Couldn't get token!");
            false
        }
    }

    fn decl_var(&mut self) -> bool {
        if self.type_base() {
            if let Some(TokenType::ID) = self.current_token() {
                let start_token = self.current_token_index;

                self.consume();
                self.array_decl();

                loop {
                    if let Some(TokenType::COMMA) = self.current_token() {
                        self.consume();

                        if let Some(TokenType::ID) = self.current_token() {
                            self.consume();
                            self.array_decl();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        } else {
            false
        }
    }

    fn array_decl(&self) -> _ {
        todo!()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decl_struct() {
        let mut tokens: Vec<TokenType> = Vec::new();

        let t_struct = Token {
            r#type: TokenType::STRUCT,
            literal: String::from(""),
            line: 1,
            column: 1,

        };

        let t_id = Token {
            r#type: TokenType::ID,
            literal: String::from(""),
            line: 1,
            column: 1,

        };

        let t_lacc = Token {
            r#type: TokenType::LACC,
            literal: String::from(""),
            line: 1,
            column: 1,

        };

        let t_racc = Token {
            r#type: TokenType::RACC,
            literal: String::from(""),
            line: 1,
            column: 1,

        };

        let t_semicolon = Token {
            r#type: TokenType::SEMICOLON,
            literal: String::from(""),
            line: 1,
            column: 1,

        };

        tokens.push(t_struct.r#type);
        tokens.push(t_id.r#type);
        tokens.push(t_lacc.r#type);
        tokens.push(t_racc.r#type);
        tokens.push(t_semicolon.r#type);

        let mut parser = Parser::new(tokens);

        assert_eq!(parser.decl_struct(), true);
        println!("{:?}", parser.current_token());

    }
}