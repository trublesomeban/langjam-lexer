#[derive(Debug, PartialEq, Clone)]
pub enum Identifier {
    Reserved(String),
    Normal(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BraceSide {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BraceType {
    Paren(BraceSide),
    Bracket(BraceSide),
    Curly(BraceSide),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Separator {
    Comma,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int(String),
    Float(String),
    Str(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    EOF,
    Sym(String),
    Lit(Literal),
    Ident(Identifier),
    Brace(BraceType),
    Sep(Separator),
    Comment(String),
}