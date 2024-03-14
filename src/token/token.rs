#[derive(Debug, PartialEq)]
pub enum TokenType {
    //identifiers
    ID,
    //keywords
    BREAK,
    CHAR,
    DOUBLE,
    ELSE,
    FOR,
    IF,
    INT,
    RETURN,
    STRUCT,
    VOID,
    WHILE,
    //constants
    CT_INT,
    CT_REAL,
    CT_CHAR,
    CT_STRING,
    //delimiters
    COMMA,
    SEMICOLON,
    LPAR,
    RPAR,
    LBRACKET,
    RBRACKET,
    LACC,
    RACC,
    //operators,
    ADD,
    SUB,
    MUL,
    DIV,
    DOT,
    AND,
    OR,
    NOT,
    ASSIGN,
    EQUAL,
    NOTEQ,
    LESS,
    LESSEQ,
    GREATER,
    GREATEREQ,
    ILLEGAL,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
    pub line: i32,
    pub column: i32,

}

pub fn lookup_identifier(identifier: &str) -> TokenType {
    match identifier {
        "break" => TokenType::BREAK,
        "char" => TokenType::CHAR,
        "double" => TokenType::DOUBLE,
        "else" => TokenType::ELSE,
        "for" => TokenType::FOR,
        "if" => TokenType::IF,
        "int" => TokenType::INT,
        "return" => TokenType::RETURN,
        "struct" => TokenType::STRUCT,
        "void" => TokenType::VOID,
        "while" => TokenType::WHILE,
        _ => TokenType::ID,
    }
}
