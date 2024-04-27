use std::fmt::Pointer;

use crate::token::token::{Token, TokenType};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current_token_index: usize,
    consumed_token: Option<&'a Token>,
}

impl<'a> Parser<'a> {
    /// Creates a new [`Parser`].
    pub fn new(tokens: Vec<Token>) -> Parser<'a> {
        let mut parser = Parser {
            tokens,
            current_token_index: 0,
            consumed_token: None,
        };

        parser
    }

    pub fn start() {

    }

    fn current_token(&mut self) -> Option<&Token> {
        self.tokens.get(self.current_token_index)
    }

    fn get_token_type(&self) -> TokenType {
        let token_type;
        if let Some(token) = self.current_token() {
            let token_type = token.r#type;
        } else {
            let token_type = TokenType::ILLEGAL;
        }

        token_type
    }

    fn consume(&mut self) {
        if let Some(token) = self.current_token() {
            self.consumed_token = Some(token.clone());
        }
        self.current_token_index += 1;
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.current_token_index + 1)
    }

    fn type_base(&mut self) -> bool {
        // if let Some(token) = self.current_token() {
        //     let start_token = self.current_token_index;
        //     match token.r#type {
        //         TokenType::INT => {
        //             self.consume();
        //             true
        //         }
        //
        //         TokenType::DOUBLE => {
        //             self.consume();
        //             true
        //         }
        //
        //         TokenType::CHAR => {
        //             self.consume();
        //             true
        //         }
        //
        //         TokenType::STRUCT => {
        //             self.consume();
        //
        //             if let Some(token) = self.current_token() {
        //                 if token.r#type == TokenType::ID {
        //                     self.consume();
        //                     true
        //                 } else {
        //                     self.current_token_index = start_token;
        //                     println!("Missing identifier!");
        //                     false
        //                 }
        //             } else {
        //                 false
        //             }
        //         }
        //
        //         _ => {
        //             println!("Should be INT, DOUBLE, CHAR or STRUCT");
        //             false
        //         }
        //     }
        // } else {
        //     false
        // }

        let start_token = self.current_token_index;

        if self.get_token_type() == TokenType::INT {
            self.consume();
            return true;
        }

        if self.get_token_type() == TokenType::DOUBLE {
            self.consume();
            return true;
        }

        if self.get_token_type() == TokenType::CHAR {
            self.consume();
            return true;
        }

        if self.get_token_type() == TokenType::STRUCT {
            self.consume();

            if self.get_token_type() == TokenType::ID {
                self.consume();
                return true;
            } else {
                self.current_token_index = start_token;
                println!("Missing identifier!");
                return false;
            }
        }
        
        println!("Should be INT, DOUBLE, CHAR or STRUCT");
        false
    }

    pub fn decl_struct(&mut self) -> bool {
        // if let Some(token) = self.current_token() {
        //     let start_token = self.current_token_index;
        //
        //     if token.r#type == TokenType::STRUCT {
        //         self.consume();
        //
        //         if let Some(token) = self.current_token() {
        //             if token.r#type == TokenType::ID {
        //                 self.consume();
        //
        //                 if let Some(token) = self.current_token() {
        //                     if token.r#type == TokenType::LACC {
        //                         self.consume();
        //
        //                         loop {
        //                             if !self.decl_var() {
        //                                 break;
        //                             } else {
        //                                 continue;
        //                             }
        //                         }
        //
        //                         if let Some(token) = self.current_token() {
        //                             if token.r#type == TokenType::RACC {
        //                                 self.consume();
        //
        //                                 if let Some(token) = self.current_token() && token.r#type == TokenType::SEMICOLON {
        //                                     self.consume();
        //                                     return true;
        //                                 } else {
        //                                     self.current_token_index = start_token;
        //                                     println!("Missing ';'!");
        //                                     return false;
        //                                 }
        //                             } else {
        //                                 self.current_token_index = start_token;
        //                                 println!("Missing ')'!");
        //                                 return false;
        //                             }
        //                         }
        //                     } else {
        //                         self.current_token_index = start_token;
        //                         println!("Missing '('!");
        //                         return false;
        //
        //                     }
        //                 }
        //             } else {
        //                 self.current_token_index = start_token;
        //                 println!("Missing identifier!");
        //                 return false;
        //             }
        //         }
        //     }
        // } else {
        //     println!("Couldn't get token!");
        //     false
        // }

        let start_token = self.current_token_index;

        if self.get_token_type() == TokenType::STRUCT {
            self.consume();

            if self.get_token_type() == TokenType::ID {
                self.consume();

                if self.get_token_type() == TokenType::LACC {
                    self.consume();

                    loop {
                        if !self.decl_var() {
                            break;
                        } else {
                            continue;
                        }
                    }

                    if self.get_token_type() == TokenType::RACC {
                        self.consume();

                        if self.get_token_type() == TokenType::SEMICOLON {
                            self.consume();
                            return true;
                        } else {
                            self.current_token_index = start_token;
                            println!("Missing ';'!");
                            return false;
                        }
                    } else {
                        self.current_token_index = start_token;
                        println!("Missing ')'!");
                        return false;
                    }
                } else {
                    self.current_token_index = start_token;
                    println!("Missing '('!");
                    return false;
                }
            } else{
                self.current_token_index = start_token;
                println!("Missing identifier!");
                return false;
            }
        } else {
            self.current_token_index = start_token;
            println!("Missing 'struct' keyword!");
            return false;
        }
    }

    fn decl_var(&mut self) -> bool {
        // if self.type_base() {
        //     if let Some(TokenType::ID) = self.current_token() {
        //         let start_token = self.current_token_index;
        //
        //         self.consume();
        //         self.array_decl();
        //
        //         loop {
        //             if let Some(TokenType::COMMA) = self.current_token() {
        //                 self.consume();
        //
        //                 if let Some(TokenType::ID) = self.current_token() {
        //                     self.consume();
        //                     self.array_decl();
        //                 } else {
        //                     self.current_token_index = start_token;
        //                     println!("{:?}", self.current_token());
        //                     break;
        //                 }
        //             } else {
        //                 break;
        //             }
        //         }
        //
        //         if let Some(TokenType::SEMICOLON) = self.current_token() {
        //             self.consume();
        //             true
        //         } else {
        //             self.current_token_index = start_token;
        //             println!("{:?}", self.current_token());
        //             false
        //         }
        //     } else {
        //         println!("Missing identifier!");
        //         false
        //     }
        // } else {
        //     println!("Cannot define variable without a type!");
        //     false
        // }
        //
        
        if self.type_base() {
            if self.get_token_type() == TokenType::ID {
                let start_token = self.current_token_index;

                self.consume();
                self.array_decl();

                loop {
                    if self.get_token_type() == TokenType::COMMA {
                        self.consume();

                        if self.get_token_type() == TokenType::ID {
                            self.consume();
                            self.array_decl();
                        } else {
                            self.current_token_index = start_token;
                            break;
                        }
                    } else {
                        break;
                    }
                }

                if self.get_token_type() == TokenType::SEMICOLON {
                    self.consume();
                    return true;
                } else {
                    self.current_token_index = start_token;
                    return false;
                }
            } else {
                println!("Missing identifier!");
                return false;
            }
        } else {
            println!("Cannot define variable without a type!");
            return false;
        }
    }

    fn array_decl(&mut self) -> bool {
        // if let Some(TokenType::LBRACKET) = self.current_token() {
        //     let start_token = self.current_token_index;
        //
        //     self.consume();
        //     self.expr();
        //
        //     if let Some(TokenType::RBRACKET) = self.current_token() {
        //         self.consume();
        //         return true;
        //     } else {
        //         self.current_token_index = start_token;
        //         println!("Missing ']'!");
        //         return false;
        //     }
        // } else {
        //     return false;
        // }
        
        let start_token = self.current_token_index;

        if self.get_token_type() == TokenType::LBRACKET {
            self.consume();
            self.expr();

            if self.get_token_type() == TokenType::RBRACKET {
                self.consume();
                return true;
            } else {
                self.current_token_index = start_token;
                println!("Missing ']'!");
                return false;
            }
        } else {
            return false;
        }
    }

    fn type_name(&mut self) -> bool {
        if self.type_base() {
            self.array_decl(); 
            return true;
        } else {
            println!("Missing type base!");
            return false;
        }
    }

    fn decl_func(&mut self) -> bool {
        if self.type_base() {
            if self.get_token_type() == TokenType::MUL {
                self.consume();
            }
        } else if self.get_token_type() == TokenType::VOID {
            self.consume();
        } else {
            println!("Cannot declare a funcion without a type!");
            return false;
        }

        if self.get_token_type() == TokenType::ID {
            let start_token = self.current_token_index;
            self.consume();

            if self.get_token_type() == TokenType::LPAR {
                self.consume();
            }
        } else {
            println!("Function needs an identifier!");
            return false;
        }
    }

    fn func_arg(&mut self) -> bool {
        if self.type_base() {

        } else {
            println!("Missing typebase!");
        }
    }

    fn expr(&self) -> bool {
        false
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decl_struct() {
        let mut tokens: Vec<TokenType> = Vec::new();

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


        let t_int = Token {
            r#type: TokenType::INT,
            literal: String::from("int"),
            line: 1,
            column: 1,
        };

        let t_id = Token {
            r#type: TokenType::ID,
            literal: String::from("x"),
            line: 1,
            column: 1,
        };

        let t_id1 = Token {
            r#type: TokenType::ID,
            literal: String::from("y"),
            line: 1,
            column: 1,
        };
        let t_semicolon = Token {
            r#type: TokenType::SEMICOLON,
            literal: String::from(""),
            line: 1,
            column: 1,
        };

        let t_comma = Token {
            r#type: TokenType::COMMA,
            literal: String::from(","),
            line: 1,
            column: 1,
        };

        let t_lbrack = Token {
            r#type: TokenType::LBRACKET,
            literal: String::from("["),
            line: 1,
            column: 1,
        };

        let t_rbrack= Token {
            r#type: TokenType::RBRACKET,
            literal: String::from("]"),
            line: 1,
            column: 1,
        };

        tokens.push(t_int.r#type);
        // tokens.push(t_id.r#type);
        // tokens.push(t_comma.r#type);
        // tokens.push(t_id1.r#type);
        // tokens.push(t_lbrack.r#type);
        // tokens.push(t_rbrack.r#type);
        // tokens.push(t_semicolon.r#type);

        let mut parser = Parser::new(tokens);

        assert_eq!(parser.type_base(), true);
        println!("{:?}", parser.current_token());

    }
}
