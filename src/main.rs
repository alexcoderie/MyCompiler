use crate::lexer::lexer::Lexer;

pub mod token;
pub mod lexer;

fn main() {
    let input = "xffxffx";

    let mut lexer = Lexer::new(input.to_string());

    let token = lexer.next_token();
    println!("{:?}", token);
}
