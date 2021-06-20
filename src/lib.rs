use std::env;
use std::error::Error;
use std::fs;

pub mod lexer;

/// Stores configuration information when the compiler is invoked
#[derive(Debug)]
pub struct Config {
    /// The C file to be compiled
    pub filename: String,
}

impl Config {
    /// Populates a `Config` from the CLI arguments to the compiler
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No file was given"),
        };

        Ok(Config { filename })
    }
}

/// Runs the compiler with a Config
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let tokens = lexer::lex(contents)?;

    println!("{:?}", tokens);

    Ok(())
}
