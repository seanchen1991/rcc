use std::error::Error;
use std::fmt;

/// The type variants our compiler recognizes, e.g. "int", "char", etc.
#[derive(Debug, Clone)]
pub enum Type {
    /// The `int` type
    Int,
    /// The `char` type
    Char,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Type::*;
        let s = match self {
            Int => "int".to_string(),
            Char => "char".to_string(),
        };

        write!(f, "{}", s)
    }
}

/// The token variants that the compiler supports
#[derive(Debug, Clone)]
pub enum Token {
    /// Opening brace
    LBrace,
    /// Closing brace
    RBrace,
    /// Opening parenthesis
    LParen,
    /// Closing parenthesis
    RParen,
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
        use self::Token::*;
        let s = match self {
            LBrace => "{".to_string(),
            RBrace => "}".to_string(),
            LParen => "(".to_string(),
            RParen => ")".to_string(),
            Semicolon => ";".to_string(),
            Return => "return".to_string(),
            Type(ty) => ty.to_string(),
            Identifier(ident) => ident.to_string(),
            Integer(int) => int.to_string(),
        };

        write!(f, "{}", s)
    }
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
