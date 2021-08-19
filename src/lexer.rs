#![allow(unused)]

use std::str::Chars;

enum Brace {
    Paren,
    Square,
    Curly,
    Angle,
}

enum Literal {
    Int,
    Float,
    Str,
}

enum Token {
    EOF,
    Sym(String),
    Lit(Literal, String),
    Keyword(String),
    Identifier(String),
    List(Brace, Vec<Token>),
}

struct Position {
    col: isize,
    ln: isize,
    idx: isize,
}

struct Lexer<'a> {
    ch: Chars<'a>,
    pos: Position,
    next: Token,
}

impl<'a> Lexer<'a> {
    pub fn next(&mut self) -> Option<Result<Token, ()>> {
        if let Some(char) = self.ch.next() {
            self.lex(char)
        } else {
            None
        }
    }

    fn lex(&mut self, char: char) -> Option<Result<Token, ()>> {
        if " \t".contains(char) {
            let char = match self.ch.next() {
                Some(char) => char,
                None => return None,
            };
            self.lex(char)
        } else if char.is_alphabetic() {
            self.ident()
        } else if char.is_numeric() {
            self.lit()
        } else {
            Some(Ok(Token::EOF))
        }
    }

    fn lit(&mut self) -> Option<Result<Token, ()>> {
        let mut lit = String::new();
        while let Some(char) = self.ch.next() {
            if char.is_numeric() {
                lit.push(char)
            } else {
                break;
            }
        }
        Some(Ok(Token::EOF))
    }

    fn ident(&mut self) -> Option<Result<Token, ()>> {
        let mut ident = String::new();
        while let Some(char) = self.ch.next() {
            if char.is_alphabetic() {
                ident.push(char)
            } else {
                break;
            }
        }
        Some(Ok(Token::EOF))
    }
}
