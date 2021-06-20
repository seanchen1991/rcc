use std::env;
use std::error::Error;
use std::fmt;
use std::fs;

/// Stores configuration information when the compiler is invoked
#[derive(Debug)]
pub struct Config {
    /// The C file to be compiled
    pub filename: String,
}

/// The type variants our compiler recognizes, e.g. "int", "char", etc.
#[derive(Debug)]
pub enum Type {
    /// The `int` type
    Int,
    /// The `char` type
    Char,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Type::Int => "int".to_string(),
            Type::Char => "char".to_string(),
        };

        write!(f, "{}", s)
    }
}

/// The token variants that the compiler supports
#[derive(Debug)]
pub enum Token {
    /// Opening brace
    OpenBrace,
    /// Closing brace
    CloseBrace,
    /// Opening parenthesis
    OpenParen,
    /// Closing parenthesis
    CloseParen,
    /// A semicolon
    Semicolon,
    /// A return keyword
    Return,
    /// A type identifier
    Type(Type),
    /// An identifier `[a-zA-Z]\w*`
    Identifier(&'static str),
    /// An integer literal `[0-9]+`
    Integer(i64),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Token::OpenBrace => "{".to_string(),
            Token::CloseBrace => "}".to_string(),
            Token::OpenParen => "(".to_string(),
            Token::CloseParen => ")".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Return => "return".to_string(),
            Token::Type(ty) => ty.to_string(),
            Token::Identifier(ident) => ident.to_string(),
            Token::Integer(int) => int.to_string(),
        };

        write!(f, "{}", s)
    }
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

    let tokens = lex(contents)?;

    println!("{:?}", tokens);

    Ok(())
}

/// Lexes the contents of a C file
pub fn lex(expr: String) -> Result<Vec<String>, Box<dyn Error>> {
    let tokens = expr
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("{", " { ")
        .replace("}", " } ")
        .replace(";", " ; ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    Ok(tokens)
}
