#![allow(unused)]

pub struct Config {
    keywords: Vec<String>,
    symbol_set: Vec<String>,
    str_qts: [char; 2],
    parens: [char; 2],
}

pub struct Positon {
    ln: isize,
    col: isize,
    idx: usize,
}

pub struct Error {
    details: char,
    pos: Positon,
}

pub enum Literal {
    Str(String),
    Int(isize),
    Float(f64),
}

pub enum Identifier {
    Lit(String),
    Keyword(String),
}

pub enum Token {
    Ident(Identifier),
    Symbol(String),
    Lit(Literal),
}

pub trait Lexer {
    fn extract_tokens(&self, config: Config) -> Result<Vec<Token>, Error>;
}

impl Lexer for String {
    // please do not look at this awful, hacky, mutable code
    fn extract_tokens(&self, config: Config) -> Result<Vec<Token>, Error> {
        fn extract_identifier() {}
        fn exract_literal() {}
        fn extract_symbol() {}

        let mut pos = Positon {
            ln: 0,
            col: -1,
            idx: 0,
        };
        let mut iter = self.chars();
        let mut result: Vec<Token> = vec![];
        while let Some(char) = iter.next() {
            pos.col += 1;
            pos.idx += 1;
            if char == '\n' {
                pos.ln += 1;
                continue;
            } else {
                match char {
                    _ => return Err(Error { details: char, pos }),
                }
            }
        }
        Ok(result)
    }
}
