pub mod lexer;
pub mod parser;

use lexer::*;


fn main() {
    let mut lexer = Lexer::new("(2.2 2 3)");

    loop {
        match lexer.next_token() {
            Ok(TokenType::EOF) => break,
            Ok(token) => println!("{:?}", token),
            Err(err) => println!("{:?}", err)
        }
    }
}