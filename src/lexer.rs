use std::{iter::Chain, iter::Peekable, str::Chars};

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
    details: char,
    pos: Position,
}

impl Error {
    fn new(reason: String, details: char, pos: Position) -> Self {
        Self {
            reason,
            details,
            pos,
        }
    }
    #[allow(unused)] // my linter does not recognize that this is being used in the other file because of the #[cfg(test)]
    pub fn fmt(&self) -> String {
        format!(
            "{} at line {}, column {}",
            self.reason, self.pos.ln, self.pos.col
        )
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
    reserved: Vec<&'a str>,
    symbols: Vec<&'a str>,
    iter: Peekable<Chain<Chars<'a>, Chars<'a>>>,
    pos: Position,
}

impl<'a> Lexer<'a> {
    fn new(chars: Chars<'a>, reserved: Vec<&'a str>, symbols: Vec<&'a str>) -> Self {
        Self {
            iter: chars.chain("\n".chars()).peekable(),
            reserved,
            symbols,
            pos: Position {
                col: -1,
                ln: 0,
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
        } else if self.symbols.contains(&char.to_string().as_str()) {
            self.symbol(char)
        } else if char.is_alphabetic() || "#_".contains(char) {
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
                '"' => self.str(char),
                '\n' => {
                    self.pos.newline();
                    self.next()
                }
                _ => Some(Err(Error::new(
                    format!("Unknown character '{}'", char),
                    char,
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
                            format!("Invalid character '{}'", char),
                            char,
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
                if char.is_alphanumeric() || "#_".contains(char.to_string().as_str()) {
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
            self.pos.idx += 1;
            self.pos.col += 1;
        }
        Some(Ok(Token::Sym(sym)))
    }
}

pub struct TokenStream<'a> {
    lexer: Lexer<'a>,
}

impl<'a> TokenStream<'a> {
    #[allow(unused)] // my linter does not recognize that this is being used in the other file because of the #[cfg(test)]
    pub fn new(s: &'a str, reserved: Vec<&'a str>, symbols: Vec<&'a str>) -> Self {
        Self {
            lexer: Lexer::new(s.chars(), reserved, symbols),
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Result<Token, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next()
    }
}
