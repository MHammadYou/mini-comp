extern crate clap;

pub mod lexer;
pub mod parser;

use lexer::*;
use lexer::lexer::Lexer;

use std::fs::File;
use clap::{App, SubCommand};

fn main() -> std::io::Result<()> {

    let matches = App::new("MiniComp")
        .subcommand(SubCommand::with_name("debug").args_from_usage(
            "
            --show...       'show certain steps in compiling process'
            <INPUT>         'File to load'
            "
        ))
        .get_matches();

    match matches.subcommand() {
        ("debug", Some(sub_matches)) => {
            let filename = sub_matches.value_of("INPUT").unwrap();
            let text = std::fs::read_to_string(filename)?;
            let lexer = Lexer::new(&text);
        },
        _ => {}
    }
    // TODO: will get back here

    let mut lexer = Lexer::new(&*data);

    loop {
        match lexer.next_token() {
            Ok(TokenType::EOF) => break,
            Ok(token) => println!("{:?}", token),
            Err(err) => println!("{:?}", err)
        }
    }
}