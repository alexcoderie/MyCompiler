use std::borrow::BorrowMut;
use std::char;
use std::ops::Deref;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::token::token::{Token, TokenType};
use crate::symbols::symbols::{self, Class, CtVal, Memory, RetVal, Symbol, SymbolTable, Type, TypeBase};

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    current_token_index: usize,
    consumed_token: Option<Token>,
    crt_depth: i32,
    crt_struct: Option<Symbol>,
    pub crt_func: Option<Symbol>,
    current_type: Type,
    pub symbols_table: SymbolTable,
    rv: RetVal,
}

impl Parser {
    /// Creates a new [`Parser`].
    pub fn new(tokens: Vec<Token>) -> Parser {
        let mut parser = Parser {
            tokens,
            current_token_index: 0,
            consumed_token: None,
            crt_depth: 0,
            crt_struct: None,
            crt_func: None,
            current_type: Type {
                type_base: TypeBase::Int,
                s: None,
                n_elements: -1,
            },
            symbols_table: symbols::SymbolTable { table: Vec::new() },
            rv: RetVal {
               r#type: None,
               is_lval: false,
               is_ctval:false,
               ct_val: None,
            }
        };

        parser
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

    fn add_var(&mut self, token: Token) {
        if let Some(crt_struct) = &mut self.crt_struct {
            let crt_struct_name = crt_struct.name.clone();
            if let Some(ref mut s_struct) = self.symbols_table.find_symbol_mut(&crt_struct_name) {
               let struct_members = s_struct.members.as_mut().unwrap();

               if struct_members.find_symbol(&token.literal).is_some() {
                   println!("ERROR1: Symbol redefinition: {}", token.literal);
                   return;
               }
               let s = struct_members.add_symbol(
                   Symbol::new(token.literal.clone(), Class::Var, None, Some(self.current_type.clone()), self.crt_depth, None, None)
                   );
            }
        }else if let Some(crt_func) = &mut self.crt_func {
            if let Some(existing_symbol) = self.symbols_table.find_symbol(&token.literal) {
                if existing_symbol.depth == self.crt_depth {
                    println!("ERROR2: Symbol redefinition: {}", token.literal);
                    return;
                }
            } 

            let s = self.symbols_table.add_symbol(
                Symbol::new(token.literal.clone(), Class::Var, Some(Memory::Local), Some(self.current_type.clone()), self.crt_depth, None, None)
                );
        } else {
            if self.symbols_table.find_symbol(&token.literal).is_some() {
                println!("ERROR3: Symbol redefinition: {}", token.literal);
                return;
            }

            let s = self.symbols_table.add_symbol(
                Symbol::new(token.literal.clone(), Class::Var, Some(Memory::Global), Some(self.current_type.clone()), self.crt_depth, None, None)
                );
        }
    }

    fn cast_var(dst: &Type, src: &Type) {
        if src.n_elements > -1 {
            if dst.n_elements > -1 {
                if src.type_base != dst.type_base {
                    println!("ERROR: An array cannot be converted to an array of another type");
                    return;
                }
            } else {
                println!("ERROR: An array cannot be converted to a non-array");
                return;
            }
        } else {
            if dst.n_elements > -1 {
                println!("ERROR: A non-array cannot be converted to an array");
                return;
            }
        }

        match src.type_base {
            TypeBase::Char | TypeBase::Int | TypeBase::Double => {
                match dst.type_base {
                    TypeBase::Char | TypeBase::Int | TypeBase::Double => return,
                    _ => {
                        println!("ERROR: Incompatible types");
                        return;
                    }
                }
            }
            TypeBase::Struct => {
                if let TypeBase::Struct = dst.type_base {
                    if src.s != dst.s {
                        println!("ERROR: A structure cannot be converted to another one");
                        return;
                    }
                    return;
                } else {
                    println!("ERROR: Incompatible types");
                    return;
                }
            }
            TypeBase::Void => (),
        }
    }

    fn add_ext_func(&mut self, name: &str, func_type: Type) -> &mut Symbol {
        self.symbols_table.add_symbol(
            Symbol::new(
                name.to_string(),
                Class::ExtFunc,
                None,
                Some(func_type),
                self.crt_depth,
                Some(symbols::SymbolTable {table: Vec::new()}),
                None,
                )
            )
    }

    fn add_func_arg(&mut self, symbol:&mut Symbol, name: &str, arg_type: Type) {
        if let Some(ref mut func) = self.symbols_table.find_symbol_mut(&symbol.name) {
            let func_args = func.args.as_mut().unwrap();
            func_args.add_symbol(
                Symbol::new(
                    name.to_string(),
                    Class::Var,
                    None,
                    Some(arg_type),
                    self.crt_depth,
                    None,
                    None,
                    )
                );
        }
    }

    fn add_ext_functions(&mut self) {
        let mut s = self.add_ext_func("put_s", Type::new(TypeBase::Void, -1)).to_owned();
        self.add_func_arg(&mut s, "s", Type::new(TypeBase::Char, 0));

        let mut s = self.add_ext_func("get_s", Type::new(TypeBase::Void, -1)).to_owned();
        self.add_func_arg(&mut s, "s", Type::new(TypeBase::Char, 0));

        let mut s = self.add_ext_func("put_i", Type::new(TypeBase::Void, -1)).to_owned();
        self.add_func_arg(&mut s, "i", Type::new(TypeBase::Int, 0));

        let mut s = self.add_ext_func("get_i", Type::new(TypeBase::Int, -1)).to_owned();

        let mut s = self.add_ext_func("put_d", Type::new(TypeBase::Void, -1)).to_owned();
        self.add_func_arg(&mut s, "d", Type::new(TypeBase::Double, 0));

        let mut s = self.add_ext_func("get_d", Type::new(TypeBase::Double, -1)).to_owned();

        let mut s = self.add_ext_func("put_c", Type::new(TypeBase::Void, -1)).to_owned();
        self.add_func_arg(&mut s, "c", Type::new(TypeBase::Char, 0));

        let mut s = self.add_ext_func("get_c", Type::new(TypeBase::Char, -1)).to_owned();


    }

    pub fn unit(&mut self) -> bool {
        self.add_ext_functions();
        loop {
            if self.decl_struct() {
                continue;
            } else if self.decl_func() {
                continue;
            } else if self.decl_var() {
                continue;
            } else {
                break;
            }
        }

        if self.get_token_type() == TokenType::EOF {
            self.consume();
            return true;
        } else {
            return false;
        }
    }

    fn type_base(&mut self) -> bool {
        let start_token = self.current_token_index;

        if self.get_token_type() == TokenType::INT {
            self.consume();
            self.current_type.type_base = TypeBase::Int;
            return true;
        }

        if self.get_token_type() == TokenType::DOUBLE {
            self.consume();
            self.current_type.type_base = TypeBase::Double;
            return true;
        }

        if self.get_token_type() == TokenType::CHAR {
            self.consume();
            self.current_type.type_base = TypeBase::Char;
            return true;
        }

        if self.get_token_type() == TokenType::STRUCT {
            self.consume();

            if self.get_token_type() == TokenType::ID {
                self.consume();
                let token_name = self.consumed_token.clone().unwrap().literal;

                if let Some(s) = self.symbols_table.find_symbol(&token_name) {
                    if s.class != Class::Struct {
                        println!("ERROR: {} is not a struct", token_name);
                        return false;
                    }
                    self.current_type.type_base = TypeBase::Struct;
                    self.current_type.s = Some(Box::new(s.clone()));
                    return true;
                } else {
                    println!("Undefined symbol: {}", token_name);
                }
            } else {
                self.current_token_index = start_token;
                println!("Missing identifier!");
            }
        }
        
        println!("Should be INT, DOUBLE, CHAR or STRUCT");
        return false;
    }

    pub fn decl_struct(&mut self) -> bool {
        println!("IN DECL_STRUCT");
        let start_token = self.current_token_index;

        if self.get_token_type() == TokenType::STRUCT {
            self.consume();

            if self.get_token_type() == TokenType::ID {
                self.consume();
                let token_name = self.consumed_token.clone().unwrap().literal;
                

                if self.get_token_type() == TokenType::LACC {
                    self.consume();

                    if self.symbols_table.find_symbol(&token_name).is_some() {
                        println!("ERROR: symbol redefinition: {}", token_name);
                        return false;
                    }

                    let crt_struct = self.symbols_table.add_symbol(
                            Symbol::new(
                                token_name, 
                                Class::Struct, 
                                None, 
                                None,
                                self.crt_depth,
                                None,
                                None
                                )
                            );
                    crt_struct.members = Some(symbols::SymbolTable { table: Vec::new()});
                    self.crt_struct = Some(crt_struct.clone());

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
                            self.crt_struct = None;
                            return true;
                        } else {
                            println!("Missing ';'!");
                        }
                    } else {
                        println!("Missing ')'!");
                    }
                } else {
                    println!("Missing '('!");
                }
            } else{
                println!("Missing identifier!");
            }
        } else {
            println!("Missing 'struct' keyword!");
        }

        self.current_token_index = start_token;
        return false;
    }

    fn decl_var(&mut self) -> bool {
        let start_token = self.current_token_index;
        println!("IN DECL_VAR");

        if self.type_base() {
            if self.get_token_type() == TokenType::ID {
                self.consume();
                let token_name = self.consumed_token.clone();
                println!("Consumed token : {:?}", token_name);
                self.array_decl();
                self.add_var(token_name.expect("Add variable into symbol table"));

                loop {
                    if self.get_token_type() == TokenType::COMMA {
                        self.consume();

                        if self.get_token_type() == TokenType::ID {
                            self.consume();
                            let token_name = self.consumed_token.clone();
                            println!("Consumed token : {:?}", token_name);
                            self.array_decl();
                            self.add_var(token_name.expect("Add variable into symbol table"));
                        } else {
                            println!("Comma should be followed by an argument!");
                            self.current_token_index = start_token;
                            return false;
                        }
                    } else {
                        break;
                    }
                }

                if self.get_token_type() == TokenType::SEMICOLON {
                    self.consume();
                    return true;
                } else {
                    println!("Missing ';'");
                }
            } else {
                println!("Missing identifier!");
            }
        } else {
            println!("Cannot define variable without a type!");
        }

        self.current_token_index = start_token;
        return false;
    }

    fn array_decl(&mut self) -> bool {
        let start_token = self.current_token_index;
        let mut rv_copy = self.rv.clone();

        if self.get_token_type() == TokenType::LBRACKET {
            self.consume();
            self.expr(&mut rv_copy);
            self.rv = rv_copy;
            self.current_type.n_elements = 0;

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
        let start_token = self.current_token_index;

        println!("IN DECL_FUNC");
        if self.type_base() {
            if self.get_token_type() == TokenType::MUL {
                self.consume();
                self.current_type.n_elements = 0;
            }
        } else if self.get_token_type() == TokenType::VOID {
            self.consume();
            println!("Void token: {:?}", self.consumed_token);
            self.current_type.type_base = TypeBase::Void;
        } else {
            println!("Cannot declare a funcion without a type!");
            return false;
        }

        if self.get_token_type() == TokenType::ID {
            self.consume();
            let token_name = self.consumed_token.clone().unwrap().literal;

            if self.get_token_type() == TokenType::LPAR {
                self.consume();

                if self.symbols_table.find_symbol(&token_name).is_some() {
                    println!("ERROR: symbol redefinition: {}", token_name);
                    return false;
                }

                self.crt_func = Some(Symbol::new(
                            token_name, 
                            Class::Func, 
                            None, 
                            Some(self.current_type.clone()),
                            self.crt_depth,
                            Some(symbols::SymbolTable { table: Vec::new()}),
                            None
                            )
                    );
                self.symbols_table.add_symbol(self.crt_func.clone().expect("Adding function symbol into the table"));
                self.crt_depth += 1;

                if self.func_arg() {
                    loop {
                        if self.get_token_type() == TokenType::COMMA {
                            self.consume();

                            if !self.func_arg() {
                                println!("Comma should be followed by an argument!");
                                self.current_token_index = start_token;
                                return false;
                            } else {
                                continue;
                            }
                        } else {
                            break;
                        }
                    }
                }

                if self.get_token_type() == TokenType::RPAR {
                    self.consume();
                    self.crt_depth -= 1;
                    if !self.stm_compound() {
                        return false;
                    } else {
                        if let Some(crt_func) = &self.crt_func {
                            self.symbols_table.delete_symbol_after(crt_func);
                        }
                        self.crt_func = None;
                        return true;
                    }
                    

                } else {
                    println!("Expecting ')' after function arguments!");
                }
            } else {
                println!("Function needs an opening paranthesis for arguments!");
            }
        } else {
            println!("Function needs an identifier!");
        }

        self.current_token_index = start_token;
        return false;
    }

    fn func_arg(&mut self) -> bool {
        let start_token = self.current_token_index;

        if self.type_base() {
            if self.get_token_type() == TokenType::ID {
                self.consume();
                let token_name = self.consumed_token.clone().unwrap().literal;
                let s = self.symbols_table.add_symbol(
                        Symbol::new(
                            token_name.clone(), 
                            Class::Var, 
                            Some(Memory::Arg), 
                            Some(self.current_type.clone()),
                            self.crt_depth,
                            None,
                            None
                            )
                        );

                if let Some(crt_func) = &self.crt_func {
                    let crt_func_name = crt_func.name.clone();
                    if let Some(ref mut func) = self.symbols_table.find_symbol_mut(&crt_func_name) {
                        let func_args = func.args.as_mut().unwrap();
                        func_args.add_symbol(Symbol::new(
                                token_name.clone(),
                                Class::Var, 
                                Some(Memory::Arg), 
                                Some(self.current_type.clone()), 
                                self.crt_depth, 
                                None, 
                                None
                                )
                            );
                    }
                }
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
        let mut rv_copy = self.rv.clone();

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

                    if self.expr(&mut rv_copy) {
                        if rv_copy.r#type.as_ref().map_or(false, |t| t.type_base == TypeBase::Struct) {
                            println!("A structure cannot be logically tested");
                            return false;
                        }
                        self.rv = rv_copy;

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
                            }
                        } else {
                            println!("Expected ')' to close the if statement!");
                        }
                    } else {
                        println!("Expected expression!");
                    }
                } else {
                    println!("Expected '(' to open the if statement!");
                }

                self.current_token_index = start_token;
                return false;
            }

            TokenType::WHILE => {
                self.consume();

                if self.get_token_type() == TokenType::LPAR {
                    self.consume();

                    if self.expr(&mut rv_copy) {
                        if rv_copy.r#type.as_ref().map_or(false, |t| t.type_base == TypeBase::Struct) {
                            println!("A structure cannot be logically tested");
                            return false;
                        }
                        self.rv = rv_copy;

                        if self.get_token_type() == TokenType::RPAR {
                            self.consume();

                            if self.stm() {
                                return true;
                            } else {
                                println!("Missing statement!");
                            }
                        } else {
                            println!("Expected ')' to close the while statement!");
                        }
                    } else {
                        println!("Missing expression!");
                    }
                } else {
                    println!("Expected '(' to open the while statement!");
                }

                self.current_token_index = start_token;
                return false;
            }

            TokenType::FOR => {
                self.consume();

                if self.get_token_type() == TokenType::LPAR {
                    self.consume();

                    let mut rv1 = RetVal {
                        r#type: None,
                        is_ctval: false,
                        is_lval: false,
                        ct_val: None,
                    };

                    self.expr(&mut rv1);

                    if self.get_token_type() == TokenType::SEMICOLON {
                        self.consume();

                        let mut rv2 = RetVal {
                            r#type: None,
                            is_ctval: false,
                            is_lval: false,
                            ct_val: None,
                        };
                        self.expr(&mut rv2);

                        if let Some(rv_type) = &rv2.r#type {
                            if rv_type.type_base == TypeBase::Struct {
                                println!("A structure cannot be logically tested");
                                return false;
                            }
                        }

                        if self.get_token_type() == TokenType::SEMICOLON {
                            self.consume();

                            let mut rv3 = RetVal {
                                r#type: None,
                                is_ctval: false,
                                is_lval: false,
                                ct_val: None,
                            };
                            self.expr(&mut rv3);

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
                            }
                        } else {
                            println!("Expected ';' in 'for' statement specifier!");
                        }
                    } else {
                        println!("Expected ';' in 'for' statement specifier!");
                    }
                } else {
                    println!("Expected '(' to open the for statement");
                }

                self.current_token_index = start_token;
                return false;
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
                if self.expr(&mut rv_copy) {
                    self.rv = rv_copy.clone();

                    if let Some(crt_func) = self.crt_func.clone() {
                        if crt_func.r#type.as_ref().map_or(false, |t| t.type_base == TypeBase::Void) && rv_copy.r#type.is_some() {
                            println!("A void function cannot return a value");
                            return false;
                        }
                        if let Some(ref rv_type) = rv_copy.r#type {
                            Parser::cast_var(&crt_func.r#type.unwrap(), rv_type);
                        }
                    }
                }

                if self.get_token_type() == TokenType::SEMICOLON {
                    self.consume();
                    return true;
                } else {
                    println!("Expected ';' after expression!");
                    self.current_token_index = start_token;
                    return false;
                }
            }
            
            _ => {
                self.expr(&mut rv_copy);
                self.rv = rv_copy;

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

            let start = self.symbols_table.table.last().cloned();
            self.crt_depth += 1;
            loop {
                if self.decl_var() {
                    continue;
                } else if self.stm() {
                    continue;
                } else {
                    break
                }
            }

            if self.get_token_type() == TokenType::RACC {
                self.consume();
                self.crt_depth -= 1;

                if let Some(start) = start {
                    self.symbols_table.delete_symbol_after(&start);
                }
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

    fn expr(&mut self, rv: &mut RetVal) -> bool {
        return self.expr_assign(rv);
    }

    fn expr_assign(&mut self, rv: &mut RetVal) -> bool {
        let start_token = self.current_token_index;

        if self.expr_unary(rv) {
            if self.get_token_type() == TokenType::ASSIGN {
                self.consume();

                let token_type = self.consumed_token.clone().unwrap().r#type;

                let mut rve = RetVal {
                    r#type: None,
                    is_ctval: false,
                    is_lval: false,
                    ct_val: None,
                };

                if self.expr_assign(&mut rve) {
                    if !rv.is_lval {
                        println!("Cannot assign to a non-lval");
                        return false;
                    }

                    if rv.r#type.as_ref().map_or(false, |t| t.n_elements > -1) 
                        || rve.r#type.as_ref().map_or(false, |t| t.n_elements > -1) 
                    {
                        println!("Arrays cannot be assigned");
                        return false;
                    }

                    Parser::cast_var(rv.r#type.as_mut().unwrap(), rve.r#type.as_ref().unwrap());
                    rv.is_ctval = false;
                    rv.is_lval = false;

                    return true;
                } else {
                    return false
                }
            } else {
                self.current_token_index = start_token;
                return self.expr_or(rv);
            }
        }

        return false;
    }

    fn expr_or(&mut self, rv: &mut RetVal) -> bool {
        if self.expr_and(rv) {
            return self.expr_or_tail(rv);
        } else {
            return false;
        }
    }

    fn expr_or_tail(&mut self, rv: &mut RetVal) -> bool {
        let start_token = self.current_token_index;

        if self.get_token_type() == TokenType::OR {
            self.consume();

            let token_type = self.consumed_token.clone().unwrap().r#type;

            let mut rve = RetVal {
                r#type: None,
                is_ctval: false,
                is_lval: false,
                ct_val: None,
            };

            if self.expr_and(&mut rve) {
                if let Some(rv_type) = &rv.r#type {
                    if rv_type.type_base == TypeBase::Struct || rve.r#type.as_ref().map_or(false, |t| t.type_base == TypeBase::Struct) {
                        println!("A structure cannot be logically tested");
                        return false;
                    }

                    rv.r#type = Some(Type::new(TypeBase::Int, -1));
                    rv.is_ctval = false;
                    rv.is_lval = false;
                }

                return self.expr_or_tail(rv); 
            } else {
                self.current_token_index = start_token;
                return false;
            }
        } else {
            return true;
        }
    }

    fn expr_and(&mut self, rv: &mut RetVal) -> bool {
        if self.expr_eq(rv) {
            return self.expr_and_tail(rv);
        } else {
            return false;
        }
    }

    fn expr_and_tail(&mut self, rv: &mut RetVal) -> bool {
        let start_token = self.current_token_index;

        if self.get_token_type() == TokenType::AND {
            self.consume();

            let token_type = self.consumed_token.clone().unwrap().r#type;

            let mut rve = RetVal {
                r#type: None,
                is_ctval: false,
                is_lval: false,
                ct_val: None,
            };

            if self.expr_eq(&mut rve) {
                if let Some(rv_type) = &rv.r#type {
                    if rv_type.type_base == TypeBase::Struct || rve.r#type.as_ref().map_or(false, |t| t.type_base == TypeBase::Struct) {
                        println!("A structure cannot be logically tested");
                        return false;
                    }

                    rv.r#type = Some(Type::new(TypeBase::Int, -1));
                    rv.is_ctval = false;
                    rv.is_lval = false;
                }

                return self.expr_and_tail(rv);
            } else {
                self.current_token_index = start_token;
                return false;
            }
        } else {
            return true;
        }
    }

    fn expr_eq(&mut self, rv: &mut RetVal) -> bool {
        if self.expr_rel(rv) {
            return self.expr_eq_tail(rv);
        } else {
            return false;
        }
    }

    fn expr_eq_tail(&mut self, rv: &mut RetVal) -> bool {
        let start_token = self.current_token_index;

        if self.get_token_type() == TokenType::EQUAL 
            || self.get_token_type() == TokenType::NOTEQ
        {
            self.consume();

            let token_type = self.consumed_token.clone().unwrap().r#type;

            let mut rve = RetVal {
                r#type: None,
                is_ctval: false,
                is_lval: false,
                ct_val: None,
            };

            if self.expr_rel(&mut rve) {
                if let Some(rv_type) = &rv.r#type {
                    if rv_type.type_base == TypeBase::Struct || rve.r#type.as_ref().map_or(false, |t| t.type_base == TypeBase::Struct) {
                        println!("A structure cannot be compared");
                        return false;
                    }

                    rv.r#type = Some(Type::new(TypeBase::Int, -1));
                    rv.is_ctval = false;
                    rv.is_lval = false;
                }

                return self.expr_eq_tail(rv);
            } else {
                self.current_token_index = start_token;
                return false;
            }
        } else {
            return true;
        }
    }

    fn expr_rel(&mut self, rv: &mut RetVal) -> bool {
        if self.expr_add(rv) {
            return self.expr_rel_tail(rv);
        } else {
            return false;
        }
    }

    fn expr_rel_tail(&mut self, rv: &mut RetVal) -> bool {
        let start_token = self.current_token_index;

        if self.get_token_type() == TokenType::LESS 
            || self.get_token_type() == TokenType::LESSEQ 
            || self.get_token_type() == TokenType::GREATER 
            || self.get_token_type() == TokenType::GREATEREQ
        {
            self.consume();

            let token_type = self.consumed_token.clone().unwrap().r#type;

            let mut rve = RetVal {
                r#type: None,
                is_ctval: false,
                is_lval: false,
                ct_val: None,
            };

            if self.expr_add(&mut rve) {
                if let Some(rv_type) = &rv.r#type {
                    if rv_type.n_elements > -1 || rve.r#type.as_ref().map_or(false, |t| t.n_elements > -1) {
                        println!("An array cannot be compared");
                        return false;
                    }
                    if rv_type.type_base == TypeBase::Struct || rve.r#type.as_ref().map_or(false, |t| t.type_base == TypeBase::Struct) {
                        println!("A structure cannot be compared");
                        return false;
                    }

                    rv.r#type = Some(Type::new(TypeBase::Int, -1));
                    rv.is_ctval = false;
                    rv.is_lval = false;
                }

                return self.expr_rel_tail(rv);
            } else {
                self.current_token_index = start_token;
                return false;
            }
        } else {
            return true;
        }
    }

    fn expr_add(&mut self, rv: &mut RetVal) -> bool {
        if self.expr_mul(rv) {
            return self.expr_add_tail(rv);
        } else {
            return false;
        }
    }

    fn expr_add_tail(&mut self, rv: &mut RetVal) -> bool {
        let start_token = self.current_token_index;

        if self.get_token_type() == TokenType::ADD 
            || self.get_token_type() == TokenType::SUB 
        {
            self.consume();

            let token_type = self.consumed_token.clone().unwrap().r#type;

            let mut rve = RetVal {
                r#type: None,
                is_ctval: false,
                is_lval: false,
                ct_val: None,
            };

            if self.expr_mul(&mut rve) {
                if let Some(rv_type) = &rv.r#type {
                    if rv_type.n_elements > -1 || rve.r#type.as_ref().map_or(false, |t| t.n_elements > -1) {
                        println!("An array cannot be added or substracted");
                        return false;
                    }
                    if rv_type.type_base == TypeBase::Struct || rve.r#type.as_ref().map_or(false, |t| t.type_base == TypeBase::Struct) {
                        println!("An structure cannot be added or substracted");
                        return false;
                    }

                    rv.is_ctval = false;
                    rv.is_lval = false;
                }
                return self.expr_add_tail(rv);
            } else {
                self.current_token_index = start_token;
                return false;
            }
        } else {
            return true;
        }
    }

    fn expr_mul(&mut self, rv: &mut RetVal) -> bool {
        if self.expr_cast(rv) {
            return self.expr_mul_tail(rv);
        } else {
            return false;
        }
    }

    fn expr_mul_tail(&mut self, rv: &mut RetVal) -> bool {
        let start_token = self.current_token_index;

        if self.get_token_type() == TokenType::MUL 
            || self.get_token_type() == TokenType::DIV 
        {
            self.consume();
            let token_type = self.consumed_token.clone().unwrap().r#type;

            let mut rve = RetVal {
                r#type: None,
                is_ctval: false,
                is_lval: false,
                ct_val: None,
            };

            if self.expr_cast(&mut rve) {
                if let Some(rv_type) = &rv.r#type {
                    if rv_type.n_elements > -1 || rve.r#type.as_ref().map_or(false, |t| t.n_elements > -1) {
                        println!("An array cannot be multiplied or divided");
                        return false;
                    }
                    if rv_type.type_base == TypeBase::Struct || rve.r#type.as_ref().map_or(false, |t| t.type_base == TypeBase::Struct) {
                        println!("A structure cannot be multiplied or divided");
                        return false;
                    }

                    rv.is_ctval = false;
                    rv.is_lval = false;
                }
                return self.expr_mul_tail(rv);
            } else {
                self.current_token_index = start_token;
                return false
            }
        } else {
            return true;
        }
    }

    fn expr_cast(&mut self, rv: &mut RetVal) -> bool {
        let start_token = self.current_token_index;

        if self.get_token_type() == TokenType::LPAR {
            self.consume();

            if self.type_name() {
                if self.get_token_type() == TokenType::RPAR {
                    self.consume();

                    let mut rve = RetVal {
                        r#type: None,
                        is_lval: false,
                        is_ctval: false,
                        ct_val: None,
                    };

                    if self.expr_cast(&mut rve) {
                        let mut current_type = self.current_type.clone();
                        Parser::cast_var(&mut current_type, rve.r#type.as_ref().unwrap());
                        rv.r#type = Some(current_type.clone());
                        rv.is_ctval = false;
                        rv.is_lval = false;

                        return true;
                    } else {
                        return false;
                    }
                } else {
                    println!("Expected ')' to close the expression!");
                    self.current_token_index = start_token;
                    return false;
                }
            } else {
                println!("Expected expression!");
                self.current_token_index = start_token;
                return false;
            }
        } else {
            return self.expr_unary(rv);
        }
    }

    fn expr_unary(&mut self, rv: &mut RetVal) -> bool {
        let start_token = self.current_token_index;

        if self.get_token_type() == TokenType::SUB || self.get_token_type() == TokenType::NOT {
            self.consume();
            let token_type = self.consumed_token.clone().unwrap().r#type;

            if self.expr_unary(rv) {
                if token_type == TokenType::SUB {
                    if let Some(t) = &rv.r#type {
                        if t.n_elements >= 0 {
                            println!("unary '-' cannot be applied to an array");
                            return false;
                        }
                        if t.type_base == TypeBase::Struct {
                            println!("unary '-' cannot be applied to a struct");
                            return false;
                        }
                    }
                } else if token_type == TokenType::NOT {
                    if let Some(t) = &rv.r#type {
                        if t.type_base == TypeBase::Struct {
                            println!("'!' cannot be applied to a struct");
                            return false;
                        }
                    }
                    rv.r#type = Some(Type::new(TypeBase::Int, -1));
                }
                rv.is_ctval = false;
                rv.is_lval = false;

                return true;
            } else {
                println!("Invalid expression after uanry operator");
                self.current_token_index = start_token;
                return false;
            }
        } else {
            return self.expr_postfix(rv);
        }
    }

    fn expr_postfix(&mut self, rv: &mut RetVal) -> bool {
        if self.expr_primary(rv) {
            return self.expr_postfix_tail(rv);
        } else {
            return false;
        }
    }

    fn expr_postfix_tail(&mut self, rv: &mut RetVal) -> bool {
        let start_token = self.current_token_index;

        match self.get_token_type() {
            TokenType::LBRACKET => {
                self.consume();

                let mut rve = RetVal {
                    r#type: None,
                    is_lval: false,
                    is_ctval: false,
                    ct_val: None,
                };

                if self.expr(&mut rve) {
                    if rv.r#type.as_ref().map_or(true, |t| t.n_elements < 0) {
                        println!("Only an array can be indexed");
                        return false;
                    }
                    
                    let mut type_int = Type::new(TypeBase::Int, -1);
                    Parser::cast_var(&mut type_int, rve.r#type.as_ref().unwrap());

                    rv.r#type.as_mut().unwrap().n_elements = -1;
                    rv.is_lval = true;
                    rv.is_ctval = false;

                    if self.get_token_type() == TokenType::RBRACKET {
                        self.consume();
                        return self.expr_postfix_tail(rv);
                    } else {
                        println!("Missing ']' after array index");
                        self.current_token_index = start_token;
                        return false;
                    }
                } else {
                    println!("Invalid expression inside square brackets");
                    return false;
                }
            }

            TokenType::DOT => {
                self.consume();

                if self.get_token_type() == TokenType::ID {
                    self.consume();
                    let token_name = self.consumed_token.clone().unwrap().literal;
                    let symbol_struct = match &rv.r#type {
                        Some(t) => t.s.clone(),
                        None => None,
                    };

                    if let Some(s_struct) = symbol_struct {
                        if let Some(s_members) = s_struct.members.expect("Couldn't find symbol!").find_symbol(&token_name) {
                            rv.r#type = Some(s_members.r#type.clone().expect("Symbol doesn't have a type!"));
                            rv.is_lval = true;
                            rv.is_ctval = false;
                        } else {
                            println!("Struct does not have a member {}", token_name);
                            return false;
                        }
                    }
                    return self.expr_postfix_tail(rv);
                } else {
                    println!("Missing identifier after '.' operator");
                    self.current_token_index = start_token;
                    return false;
                }
            }

            _ => true
        }
    }

    fn expr_primary(&mut self, rv: &mut RetVal) -> bool {
        let start_token = self.current_token_index;
        let mut arg = self.rv.clone();

        match self.get_token_type() {
            TokenType::ID => {
                self.consume();
                let token_name = self.consumed_token.clone().unwrap().literal;
                if let Some(mut symbol) = self.symbols_table.find_symbol(&token_name).cloned() {
                    // let symbol = symbol.unwrap();
                    rv.r#type = Some(symbol.r#type.clone().expect("symbol.r#type is None"));
                    rv.is_ctval = false;
                    rv.is_lval = true;
                    
                    if self.get_token_type() == TokenType::LPAR {
                        self.consume();
                        if symbol.class != Class::Func && symbol.class != Class::ExtFunc {
                            println!("Call to a non-function {}", token_name);
                            return false;
                        }


                        let mut crt_def_args = symbol.args.as_mut().unwrap().table.iter_mut();
                        if self.expr(&mut arg) {
                            if let Some(crt_arg) = crt_def_args.next() {
                                if let (Some(crt_arg_type), Some(arg_type)) = (&crt_arg.r#type, &arg.r#type) {
                                    Parser::cast_var(crt_arg_type, arg_type);
                                }
                            } else {
                                println!("Too many arguments in call");
                                return false;
                            }

                            loop {
                                if self.get_token_type() == TokenType::COMMA {
                                    self.consume();

                                    if !self.expr(&mut arg) {
                                        println!("Comma should be followed by another expression!");
                                        self.current_token_index = start_token;
                                        return false;
                                    } else if let Some(crt_arg) = crt_def_args.next() {
                                        if let (Some(crt_arg_type), Some(arg_type)) = (&crt_arg.r#type, &arg.r#type) {
                                            Parser::cast_var(crt_arg_type, arg_type);
                                        }
                                        continue;
                                    } else {
                                        println!("Too many arguments in call");
                                        return false;
                                    }
                                } else {
                                    break;
                                }
                            }
                        }

                        if self.get_token_type() == TokenType::RPAR {
                            self.consume();
                            if crt_def_args.next().is_some() {
                                println!("Too few arguments in call!");
                                return false;
                            }

                            rv.r#type = Some(symbol.r#type.clone().expect("symbol.r#type is None"));
                            rv.is_ctval = false;
                            rv.is_lval = false;
                        } else {
                            println!("Expected ')' to close the expression!");
                            self.current_token_index = start_token;
                            return false;
                        }
                    } else if symbol.class == Class::Func || symbol.class == Class::ExtFunc {
                        println!("Missing call for function {}", token_name);
                        return false;
                    }
                } else {
                    println!("Undefined symbol {}", token_name);
                    return false;
                }

                return true;
            }

            TokenType::CT_INT => {
                self.consume();
                rv.r#type = Some(Type::new(TypeBase::Int, -1));
                if let Some(token) = self.consumed_token.clone() {
                    match token.literal.parse::<i64>() {
                        Ok(number) => {
                            rv.ct_val = Some(CtVal::Int(number));
                        },
                        Err(e) => {
                            println!("Failed to parse number: {}", e);
                            rv.ct_val = None;
                        }
                    }
                } else {
                    println!("No token consumed");
                    rv.r#type = None;
                }
                rv.is_ctval = true;
                rv.is_lval = false;

                return true;
            }

            TokenType::CT_REAL => {
                self.consume();

                rv.r#type = Some(Type::new(TypeBase::Double, -1));
                if let Some(token) = self.consumed_token.clone() {
                    match token.literal.parse::<f64>() {
                        Ok(number) => {
                            rv.ct_val = Some(CtVal::Double(number));
                        },
                        Err(e) => {
                            println!("Failed to parse number: {}", e);
                            rv.ct_val = None;
                        }
                    }
                } else {
                    println!("No token consumed");
                    rv.r#type = None;
                }
                rv.is_ctval = true;
                rv.is_lval = false;

                return true;
            }

            TokenType::CT_CHAR => {
                self.consume();

                rv.r#type = Some(Type::new(TypeBase::Char, -1));
                if let Some(token) = self.consumed_token.clone() {
                    match token.literal.parse::<char>() {
                        Ok(char_literal) => {
                            rv.ct_val = Some(CtVal::Char(char_literal));
                        },
                        Err(e) => {
                            println!("Failed to parse number: {}", e);
                            rv.ct_val = None;
                        }
                    }
                } else {
                    println!("No token consumed");
                    rv.r#type = None;
                }
                rv.is_ctval = true;
                rv.is_lval = false;

                return true;
            }
            TokenType::CT_STRING => {
                self.consume();
                
                rv.r#type = Some(Type::new(TypeBase::Char, 0));
                rv.ct_val = Some(CtVal::Str(self.consumed_token.clone().unwrap().literal));
                rv.is_ctval = true;
                rv.is_lval = false;
                return true;
            }

            TokenType::LPAR => {
                self.consume();

                if self.expr(rv) {
                    if self.get_token_type() == TokenType::RPAR {
                        self.consume();
                        return true;
                    } else {
                        self.current_token_index = start_token;
                        println!("Expected ')' to close the expression!");
                    }
                }

                return false;
            }

            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::{self, File}, io::{self, BufRead, BufReader, Write}};

    use crate::{lexer::lexer::Lexer, token::token::TokenType};

    use super::*;

    #[test]
    fn test_rules() {
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

        let t_dot = Token {
            r#type: TokenType::DOT,
            literal: String::from("."),
            line: 1,
            column: 1,
        };

        let t_eof = Token {
            r#type: TokenType::EOF,
            literal: String::from("eof"),
            line: 1,
            column: 1,
        };

        let t_ctint = Token {
            r#type: TokenType::CT_INT,
            literal: String::from("5"),
            line: 1,
            column: 1,
        };

        let t_assign = Token {
            r#type: TokenType::ASSIGN,
            literal: String::from("="),
            line: 1,
            column: 1,
        };

        let t_less = Token {
            r#type: TokenType::LESS,
            literal: String::from("<"),
            line: 1,
            column: 1,
        };

        let mut tokens: Vec<Token> = Vec::new();
        tokens.push(t_struct.clone());
        tokens.push(t_id.clone());
        tokens.push(t_lacc.clone());
        tokens.push(t_racc.clone());
        tokens.push(t_semicolon.clone());

        tokens.push(t_struct.clone());
        tokens.push(t_id1.clone());
        tokens.push(t_lacc.clone());
        tokens.push(t_racc.clone());
        tokens.push(t_semicolon.clone());
        tokens.push(t_eof.clone());
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.unit(), true, "{:?}", parser.current_token());

        // if let Ok(file) = File::open("./res/9.c") {
        //     let reader = BufReader::new(file);
        //     let mut lexer = Lexer::new(String::new());
        //
        //     for line in reader.lines() {
        //         let line = line.unwrap();
        //         lexer.set_input(line);
        //
        //         loop {
        //             let token = lexer.next_token();
        //
        //             if token.r#type == TokenType::EOF {
        //                 break;
        //             } else {
        //                 tokens.push(token.clone());
        //             }
        //         }
        //     }
        //
        //     tokens.push(Token {r#type: TokenType::EOF, literal: String::from("EOF"), line: 1, column: 1});
        //     // println!("{:?}", tokens);
        //     let mut parser = Parser::new(tokens);
        //     assert_eq!(parser.unit(), true);
        //
        // } else {
        //     eprintln!("Failed to open the file");
        // }
    }
}
