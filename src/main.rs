use std::{fs::{self, File}, io::{self, BufRead, BufReader, Write}};

use crate::{lexer::lexer::Lexer, token::token::TokenType};

pub mod token;
pub mod lexer;
pub mod parser;

fn main() -> io::Result<()>{
    let mut output_file = fs::File::create("./res/tokens.txt")?;

    if let Ok(file) = File::open("./res/9.c") {
        let reader = BufReader::new(file);
        let mut lexer = Lexer::new(String::new());

        for line in reader.lines() {
            let line = line.unwrap();
            lexer.set_input(line);

            loop {
                let token = lexer.next_token();

                if token.r#type == TokenType::EOF {
                    break;
                }

                writeln!(output_file, "{:?}", token)?;

            }
        }
    } else {
        eprintln!("Failed to open the file");
    }

    Ok(())
}
