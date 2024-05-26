use std::{fs::{self, File}, io::{self, BufRead, BufReader, Write}};

use parser::parser::Parser;
use token::token::Token;

use crate::{lexer::lexer::Lexer, token::token::TokenType};

pub mod token;
pub mod lexer;
pub mod parser;
pub mod symbols;

fn main() -> io::Result<()>{
    let mut output_file = fs::File::create("./res/tokens.txt")?;

    if let Ok(file) = File::open("./res/8.c") {
        let reader = BufReader::new(file);
        let mut lexer = Lexer::new(String::new());
        let mut tokens: Vec<Token> = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            lexer.set_input(line);

            loop {
                let token = lexer.next_token();

                if token.r#type == TokenType::EOF {
                    break;
                } else {
                    tokens.push(token.clone());
                }

                writeln!(output_file, "{:?}", token)?;
            }
        }
        tokens.push(Token {r#type: TokenType::EOF, literal: String::from("EOF"), line: lexer.get_line() , column: 1});

        let mut parser = Parser::new(tokens);
        if parser.unit() {
            print!("\x1B[2J\x1B[1;1H");
            println!("Success!");
            for element in &parser.symbols_table.table {
                println!("{:?}", element);
            }
        } else {
            println!("Didn't work :(");
            println!("{:?}", parser.symbols_table);
        }
    } else {
        eprintln!("Failed to open the file");
    }

    Ok(())
}
