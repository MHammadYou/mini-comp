extern crate clap;

pub mod lexer;
pub mod parser;

use lexer::lexer::Lexer;

use clap::{App, SubCommand};

fn main() -> std::io::Result<()> {

    let matches = App::new("MiniComp")
        .subcommand(SubCommand::with_name("debug").args_from_usage(
            "
            --show=[TOKENS]...       'show certain steps in compiling process, valid values are 'tokens', 'ast''
            <INPUT>         'File to load'
            "
        ))
        .get_matches();

    match matches.subcommand() {
        Some(("debug", sub_matches)) => {
            let filename = sub_matches.value_of("INPUT").unwrap();
            let text = std::fs::read_to_string(filename)?;
            let lexer = Lexer::new(&text);

            let shows = sub_matches.values_of("show").unwrap_or_default().collect::<Vec<&str>>();
            if shows.contains(&"tokens") {
                let mut lexer = lexer.clone();
                // loop {
                //         match lexer.next_token() {
                //             Ok(TokenType::EOF) => break,
                //             Ok(token) => println!("{:?}", token),
                //             Err(err) => println!("{:?}", err)
                //         }
                //     }

                let tokens = lexer.get_tokens();
                for token in tokens {
                    println!("{:?}", token);
                }
            }
        },
        _ => {}
    }

    Ok(())
}