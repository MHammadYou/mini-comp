pub mod lexer;
pub mod parser;

use lexer::*;
use lexer::lexer::Lexer;


fn main() {
    let mut lexer = Lexer::new("22 2.2");

    loop {
        match lexer.next_token() {
            Ok(TokenType::EOF) => break,
            Ok(token) => println!("{:?}", token),
            Err(err) => println!("{:?}", err)
        }
    }
}