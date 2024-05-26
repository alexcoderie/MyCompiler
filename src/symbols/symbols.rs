#[derive(Debug, Clone, PartialEq)]
pub enum TypeBase {
    Int,
    Double,
    Char,
    Struct,
    Void,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub type_base: TypeBase,
    pub s: Option<Box<Symbol>>,
    pub n_elements: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Class {
    Var,
    Func,
    ExtFunc,
    Struct,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Memory {
    Global,
    Arg,
    Local,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub class: Class,
    pub memory: Option<Memory>,
    pub r#type: Option<Type>,
    pub depth: i32,
    pub args: Option<SymbolTable>,
    pub members: Option<SymbolTable>,
}

impl Symbol {
    pub fn new(name: String, 
               class: Class, 
               memory: Option<Memory>, 
               r#type: Option<Type>, 
               depth: i32, 
               args: Option<SymbolTable>, 
               members: Option<SymbolTable>
               ) -> Symbol {
        let mut symbol = Symbol {
            name,
            class,
            memory,
            r#type,
            depth,
            args,
            members,
        };

        symbol
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTable {
    pub table: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut symbol_table = SymbolTable {
            table: Vec::new(),
        };

        symbol_table
    }

    pub fn add_symbol(&mut self, symbol: Symbol) -> &mut Symbol {
        self.table.push(symbol);

        self.table.last_mut().unwrap()
    }

    pub fn find_symbol(&mut self, name: &str) -> Option<&Symbol> {
        self.table.iter().rev().find(|symbol| symbol.name == name)
    }

    pub fn find_symbol_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        self.table.iter_mut().rev().find(|symbol| symbol.name == name)
    }

    pub fn find_symbol_index(&mut self, name: &str) -> Option<usize> {
        self.table.iter().position(|symbol| symbol.name == name)
    }

    pub fn delete_symbol_after(&mut self, target_symbol: &Symbol) {
        if let Some(index) = self.find_symbol_index(&target_symbol.name) {
            self.table.truncate(index + 1);
        }
    }
}
