pub mod lexer;

use lexer::*;

fn main() {
    let mut lexer = Lexer::new("()()()");

    loop {
        match lexer.next_token() {
            Ok(TokenType::EOF) => break,
            Ok(token) => println!("{:?}", token),
            Err(err) => println!("{:?}", err)
        }
    }
}