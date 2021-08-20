use std::{fmt, iter::Chain, iter::Peekable, str::Chars};

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Reserved(String),
    Normal(String),
}

#[derive(Debug, PartialEq)]
pub enum BraceSide {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum BraceType {
    Paren(BraceSide),
    Bracket(BraceSide),
    Curly(BraceSide),
}

#[derive(Debug, PartialEq)]
pub enum Separator {
    Comma,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(String),
    Float(String),
    Str(String),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    Sym(String),
    Lit(Literal),
    Ident(Identifier),
    Brace(BraceType),
    Sep(Separator),
}

#[derive(Debug)]
pub struct Error {
    reason: String,
    pos: Position,
}

impl Error {
    fn new(reason: String, pos: Position) -> Self {
        Self { reason, pos }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}, at line: {}, column: {}.",
            self.reason, self.pos.ln, self.pos.col
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    col: isize,
    ln: isize,
    idx: isize,
}

impl Position {
    fn newline(&mut self) {
        self.col = 0;
        self.ln += 1
    }
    fn advance(&mut self) {
        self.col += 1;
        self.idx += 1;
    }
}

struct Lexer<'a> {
    reserved: &'a [&'a str],
    symbols: &'a [&'a str],
    strings: &'a [char],
    comments: &'a [char],
    iter: Peekable<Chain<Chars<'a>, Chars<'a>>>,
    pos: Position,
}

impl<'a> Lexer<'a> {
    fn new(
        chars: Chars<'a>,
        reserved: &'a [&'a str],
        symbols: &'a [&'a str],
        strings: &'a [char],
        comments: &'a [char],
    ) -> Self {
        Self {
            iter: chars.chain("\n".chars()).peekable(),
            reserved,
            symbols,
            strings,
            comments,
            pos: Position {
                col: 0,
                ln: 1,
                idx: 0,
            },
        }
    }

    fn next(&mut self) -> Option<Result<Token, Error>> {
        if let Some(char) = self.iter.next() {
            self.pos.advance();
            self.lex(char)
        } else {
            None
        }
    }

    fn lex(&mut self, char: char) -> Option<Result<Token, Error>> {
        if self.iter.peek().is_none() {
            Some(Ok(Token::EOF))
        } else if " \t".contains(char) {
            self.next()
        } else if self.comments.contains(&char) {
            while let Some(char) = self.iter.next() {
                if char == '\n' {
                    break;
                }
            }
            self.lex(char)
        } else if self.strings.contains(&char) {
            self.str(char)
        } else if self.symbols.contains(&char.to_string().as_str()) {
            self.symbol(char)
        } else if char.is_alphabetic() || char == '_' {
            self.ident(char)
        } else if char.is_numeric() {
            self.num(char)
        } else {
            match char {
                '(' => Some(Ok(Token::Brace(BraceType::Paren(BraceSide::Left)))),
                ')' => Some(Ok(Token::Brace(BraceType::Paren(BraceSide::Right)))),
                '[' => Some(Ok(Token::Brace(BraceType::Bracket(BraceSide::Left)))),
                ']' => Some(Ok(Token::Brace(BraceType::Bracket(BraceSide::Right)))),
                '{' => Some(Ok(Token::Brace(BraceType::Curly(BraceSide::Left)))),
                '}' => Some(Ok(Token::Brace(BraceType::Curly(BraceSide::Right)))),
                ',' => Some(Ok(Token::Sep(Separator::Comma))),
                '\n' => {
                    self.pos.newline();
                    self.next()
                }
                _ => Some(Err(Error::new(
                    format!("Unknown character: '{}'", char),
                    self.pos,
                ))),
            }
        }
    }

    fn num(&mut self, char: char) -> Option<Result<Token, Error>> {
        let mut num = String::new();
        let mut float = false;
        num.push(char);
        while match self.iter.peek() {
            Some(char) => {
                if char.is_numeric() || *char == '.' {
                    true
                } else {
                    false
                }
            }
            _ => false,
        } {
            if let Some(char) = self.iter.next() {
                self.pos.advance();
                if char == '.' {
                    if float {
                        return Some(Err(Error::new(
                            format!(
                                "Invalid character '{}': expected whitespace or seperator",
                                char
                            ),
                            self.pos,
                        )));
                    }
                    num.push(char);
                    float = true
                } else {
                    num.push(char)
                }
            }
        }

        Some(Ok(Token::Lit(if float {
            Literal::Float(num)
        } else {
            Literal::Int(num)
        })))
    }

    fn str(&mut self, _: char) -> Option<Result<Token, Error>> {
        let mut str = String::new();
        while let Some(char) = self.iter.next() {
            self.pos.idx += 1;
            self.pos.col += 1;
            if char == '"' {
                break;
            }
            str.push(char)
        }
        Some(Ok(Token::Lit(Literal::Str(str))))
    }

    fn ident(&mut self, char: char) -> Option<Result<Token, Error>> {
        let mut ident = String::new();
        ident.push(char);
        while match self.iter.peek() {
            Some(char) => {
                if char.is_alphanumeric() || *char == '_' {
                    true
                } else {
                    false
                }
            }
            _ => false,
        } {
            if let Some(char) = self.iter.next() {
                self.pos.idx += 1;
                self.pos.col += 1;
                ident.push(char)
            }
        }
        Some(Ok(Token::Ident(
            if self.reserved.contains(&ident.as_str()) {
                Identifier::Reserved(ident)
            } else {
                Identifier::Normal(ident)
            },
        )))
    }

    fn symbol(&mut self, char: char) -> Option<Result<Token, Error>> {
        let mut sym = String::new();
        let mut multi = false;
        sym.push(char);
        for sy in self.symbols.to_owned() {
            if sy.len() > 1 {
                if let Some(char) = self.iter.peek() {
                    if sy.chars().collect::<Vec<char>>()[1] == *char {
                        multi = true;
                        sym.push(*char);
                        break;
                    }
                }
            }
        }
        if multi {
            self.iter.next();
            self.pos.advance();
        }
        Some(Ok(Token::Sym(sym)))
    }
}

pub struct TokenStream<'a> {
    lexer: Lexer<'a>,
}

impl<'a> TokenStream<'a> {
    #[allow(unused)]
    pub fn new(
        s: &'a str,
        reserved: &'a [&'a str],
        symbols: &'a [&'a str],
        strings: &'a [char],
        comments: &'a [char],
    ) -> Self {
        Self {
            lexer: Lexer::new(s.trim().chars(), reserved, symbols, strings, comments),
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Result<Token, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next()
    }
}
