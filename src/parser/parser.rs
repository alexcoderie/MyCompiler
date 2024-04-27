use std::{borrow::Borrow, fmt::Pointer};

use crate::token::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current_token_index: usize,
    consumed_token: Option<Token>,
}

impl Parser {
    /// Creates a new [`Parser`].
    pub fn new(tokens: Vec<Token>) -> Parser {
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

    fn get_token_type(&mut self) -> TokenType {
        if let Some(token) = self.current_token() {
            token.r#type.clone()
        } else {
            TokenType::ILLEGAL
        }
    }

    fn consume(&mut self) {
        if let Some(token) = self.current_token() {
            self.consumed_token = Some(token).cloned();
        }
        self.current_token_index += 1;
    }

    fn type_base(&mut self) -> bool {
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
                    println!("Missing ';'");
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

                self.func_arg();

                loop {
                    if self.get_token_type() == TokenType::COMMA {
                        self.consume();

                        if !self.func_arg() {
                            println!("Comma should be followed by an argument!");
                            self.current_token_index = start_token;
                            break;
                        } else {
                            continue;
                        }
                    } else {
                        break;
                    }
                }

                if self.get_token_type() == TokenType::RPAR {
                    self.consume();
                    return true;

                } else {
                    println!("Expecting ')' after function arguments!");
                    self.current_token_index = start_token;
                    return false;
                }
            } else {
                println!("Funtion needs an opening paranthesis for arguments!");
                self.current_token_index = start_token;
                return false;
            }
        } else {
            println!("Function needs an identifier!");
            return false;
        }
    }

    fn func_arg(&mut self) -> bool {
        let start_token = self.current_token_index;

        if self.type_base() {
            if self.get_token_type() == TokenType::ID {
                self.consume();
                self.array_decl();

                return true;
            } else {
                println!("Missing identifier!");
                self.current_token_index = start_token;
                return false;
            }
        } else {
            return false;
        }
    }

    fn stm(&mut self) -> bool {
        let start_token = self.current_token_index;

        match self.get_token_type() {
            TokenType::LACC => {
                if self.stm_compound() {
                    return true;
                } else {
                    return false;
                }
            }

            TokenType::IF => {
                self.consume();
                
                if self.get_token_type() == TokenType::LPAR {
                    self.consume();

                    if self.expr() {
                        if self.get_token_type() == TokenType::RPAR {
                            self.consume();
                            
                            if self.stm() {
                                if self.get_token_type() == TokenType:: ELSE {
                                    self.consume();
                                    if !self.stm() {
                                        println!("Else branch left empty, should be followed by another statement!");
                                        self.current_token_index = start_token;
                                        return false;
                                    }
                                }

                                return true;
                            } else {
                                println!("Missing statement!");
                                self.current_token_index = start_token;
                                return false;
                            }
                        } else {
                            println!("Expected ')' to close the if statement!");
                            self.current_token_index = start_token;
                            return false;
                        }
                    } else {
                        println!("Expected expression!");
                        self.current_token_index = start_token;
                        return false;
                    }
                } else {
                    println!("Expected '(' to open the if statement!");
                    self.current_token_index = start_token;
                    return false;
                }
            }

            TokenType::WHILE => {
                self.consume();

                if self.get_token_type() == TokenType::LPAR {
                    self.consume();

                    if self.expr() {
                        if self.get_token_type() == TokenType::RPAR {
                            self.consume();

                            if self.stm() {
                                return true;
                            } else {
                                println!("Missing statement!");
                                return false;
                            }
                        } else {
                            println!("Expected ')' to close the while statement!");
                            self.current_token_index = start_token;
                            return false;
                        }
                    } else {
                        println!("Missing expression!");
                        self.current_token_index = start_token;
                        return false;
                    }
                } else {
                    println!("Expected '(' to open the while statement!");
                    return false;
                }
            }

            TokenType::FOR => {
                self.consume();

                if self.get_token_type() == TokenType::LPAR {
                    self.consume();
                    self.expr();

                    if self.get_token_type() == TokenType::SEMICOLON {
                        self.consume();
                        self.expr();

                        if self.get_token_type() == TokenType::SEMICOLON {
                            self.consume();
                            self.expr();

                            if self.get_token_type() == TokenType::RPAR {
                                self.consume();

                                if self.stm() {
                                    return true;
                                } else {
                                    println!("Missing statement!");
                                    return false;
                                }
                            } else {
                                println!("Expected ')' to close the for statement!");
                                self.current_token_index = start_token;
                                return false;
                            }
                        } else {
                            println!("Expected ';' in 'for' statement specifier!");
                            self.current_token_index = start_token;
                            return false;
                        }
                    } else {
                        println!("Expected ';' in 'for' statement specifier!");
                        self.current_token_index = start_token;
                        return false;
                    }

                } else {
                    println!("Expected '(' to open the for statement");
                    self.current_token_index = start_token;
                    return false;
                }
            }

            TokenType::BREAK => {
                self.consume();

                if self.get_token_type() == TokenType::SEMICOLON {
                    self.consume();
                    return true;
                } else {
                    println!("Expected ';' after expression!");
                    self.current_token_index = start_token;
                    return false;
                }
            }

            TokenType::RETURN => {
                self.consume();
                self.expr();

                if self.get_token_type() == TokenType::SEMICOLON {
                    self.consume();
                    return true;
                } else {
                    println!("Expected ';' after expression!");
                    return false;
                }
            }
            
            _ => {
                self.expr();
                if self.get_token_type() == TokenType::SEMICOLON {
                    self.consume();
                    return true;
                } else {
                    println!("Expected ';' after expression!");
                    return false;
                }
            }
        }
    }

    fn stm_compound(&mut self) -> bool {
        let start_token = self.current_token_index;

        if self.get_token_type() == TokenType::LACC {
            self.consume();

            loop {
                if self.decl_var() || self.stm(){
                    continue;
                } else {
                    break;
                }
            }

            if self.get_token_type() == TokenType::RACC {
                self.consume();
                return true;
            } else {
                println!("Expected '}}' to close the compound statement!");
                self.current_token_index = start_token;
                return false;
            }
        } else {
            println!("Expected '{{' to open the compound statement!");
            return false;
        }
    }

    fn expr(&mut self) -> bool {
        true
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rules() {
        let mut tokens: Vec<Token> = Vec::new();

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

        let t_struct = Token {
            r#type: TokenType::STRUCT,
            literal: String::from("struct"),
            line: 1,
            column: 1,
        };

        let t_char = Token {
            r#type: TokenType::CHAR,
            literal: String::from("char"),
            line: 1,
            column: 1,
        };

        let t_double = Token {
            r#type: TokenType::DOUBLE,
            literal: String::from("double"),
            line: 1,
            column: 1,
        };

        let t_lacc = Token {
            r#type: TokenType::LACC,
            literal: String::from("{"),
            line: 1,
            column: 1,
        };

        let t_racc = Token {
            r#type: TokenType::RACC,
            literal: String::from("}"),
            line: 1,
            column: 1,
        };

        let t_mul = Token {
            r#type: TokenType::MUL,
            literal: String::from("*"),
            line: 1,
            column: 1,
        };

        let t_lpar = Token {
            r#type: TokenType::LPAR,
            literal: String::from("("),
            line: 1,
            column: 1,
        };

        let t_rpar = Token {
            r#type: TokenType::RPAR,
            literal: String::from(")"),
            line: 1,
            column: 1,
        };

        let t_while = Token {
            r#type: TokenType::WHILE,
            literal: String::from("while"),
            line: 1,
            column: 1,
        };

        let t_if = Token {
            r#type: TokenType::IF,
            literal: String::from("if"),
            line: 1,
            column: 1,
        };

        let t_return = Token {
            r#type: TokenType::RETURN,
            literal: String::from("return"),
            line: 1,
            column: 1,
        };

        let t_else = Token {
            r#type: TokenType::ELSE,
            literal: String::from("else"),
            line: 1,
            column: 1,
        };

        let t_break = Token {
            r#type: TokenType::BREAK,
            literal: String::from("break"),
            line: 1,
            column: 1,
        };

        let t_for = Token {
            r#type: TokenType::FOR,
            literal: String::from("for"),
            line: 1,
            column: 1,
        };
        // tokens.push(t_id.r#type);
        // tokens.push(t_semicolon.r#type);
        // tokens.push(t_struct);
        // tokens.push(t_lacc.clone());
        // tokens.push(t_int);
        // tokens.push(t_id.clone());
        // tokens.push(t_semicolon.clone());
        tokens.push(t_for);
        tokens.push(t_lpar.clone());
        tokens.push(t_semicolon.clone());
        tokens.push(t_semicolon.clone());
        tokens.push(t_rpar.clone());
        tokens.push(t_lacc);
        tokens.push(t_if);
        tokens.push(t_lpar);
        tokens.push(t_rpar);
        tokens.push(t_return.clone());
        tokens.push(t_semicolon.clone());
        tokens.push(t_else);
        tokens.push(t_break);
        tokens.push(t_semicolon);
        tokens.push(t_racc.clone());
        // tokens.push(t_racc);
        // tokens.push(t_mul);
        // tokens.push(t_id1.clone());
        // tokens.push(t_lbrack);
        // tokens.push(t_rbrack);
        // tokens.push(t_comma);
        // tokens.push(t_id1);
        // tokens.push(t_semicolon.clone());
        // tokens.push(t_racc);
        // tokens.push(t_char);
        // tokens.push(t_double);

        let mut parser = Parser::new(tokens);

        assert_eq!(parser.stm(), true);
        println!("{:?}", parser.current_token());

    }
}
