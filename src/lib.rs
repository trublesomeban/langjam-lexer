mod lexer;

#[cfg(test)]
use lexer::*;
#[test]
fn lex_int() {
    let mut iter = TokenStream::new("3", vec![], vec![]);
    let token = iter.next().unwrap().unwrap();
    println!("{:?}", token);
    assert_eq!(token, Token::Lit(Literal::Int, "3".to_string()));
}

#[test]
fn lex_float() {
    let mut iter = TokenStream::new("3.14", vec![], vec![]);
    let token = iter.next().unwrap().unwrap();
    println!("{:?}", token);
    assert_eq!(token, Token::Lit(Literal::Float, "3.14".to_string()));
}

#[test]
fn lex_string() {
    let mut iter = TokenStream::new("\"hello!\"", vec![], vec![]);
    let token = iter.next().unwrap().unwrap();
    println!("{:?}", token);
    assert_eq!(token, Token::Lit(Literal::Str, "hello!".to_string()));
}

#[test]
fn lex_ident() {
    let mut iter = TokenStream::new("hello", vec![], vec![]);
    let token = iter.next().unwrap().unwrap();
    println!("{:?}", token);
    assert_eq!(token, Token::Ident(Identifier::Normal, "hello".to_string()));
}

#[test]
fn lex_braces() -> Result<(), ()> {
    let iter = TokenStream::new("(1, 2) [3, 4] {5, 6}", vec![], vec![]);
    let mut result: Vec<Token> = vec![];
    for token in iter {
        result.push(token?);
    }
    println!("{:?}", result);
    assert_eq!(
        result,
        vec![
            Token::Brace(BraceType::Paren, BraceSide::Left),
            Token::Lit(Literal::Int, "1".to_string()),
            Token::Sep(Separator::Comma),
            Token::Lit(Literal::Int, "2".to_string()),
            Token::Brace(BraceType::Paren, BraceSide::Right),
            Token::Brace(BraceType::Bracket, BraceSide::Left),
            Token::Lit(Literal::Int, "3".to_string()),
            Token::Sep(Separator::Comma),
            Token::Lit(Literal::Int, "4".to_string()),
            Token::Brace(BraceType::Bracket, BraceSide::Right),
            Token::Brace(BraceType::Curly, BraceSide::Left),
            Token::Lit(Literal::Int, "5".to_string()),
            Token::Sep(Separator::Comma),
            Token::Lit(Literal::Int, "6".to_string()),
            Token::Brace(BraceType::Curly, BraceSide::Right),
        ]
    );
    Ok(())
}

#[test]
fn lex_multi() -> Result<(), ()> {
    let iter = TokenStream::new("let x = (3.14, 0.86)", vec!["let"], vec!["="]);
    let mut result: Vec<Token> = vec![];
    for token in iter {
        result.push(token?);
    }
    println!("{:?}", result);
    assert_eq!(
        result,
        vec![
            Token::Ident(Identifier::Reserved, "let".to_string()),
            Token::Ident(Identifier::Normal, "x".to_string()),
            Token::Sym("=".to_string()),
            Token::Brace(BraceType::Paren, BraceSide::Left),
            Token::Lit(Literal::Float, "3.14".to_string()),
            Token::Sep(Separator::Comma),
            Token::Lit(Literal::Float, "0.86".to_string()),
            Token::Brace(BraceType::Paren, BraceSide::Right),
        ]
    );
    Ok(())
}

#[test]
fn lex_fn() -> Result<(), ()> {
    let iter = TokenStream::new(
        "fn sum(x: i32, y: i32) = 
		x + y",
        vec!["fn"],
        vec!["+", "=", ":"],
    );
    let mut result: Vec<Token> = vec![];
    for token in iter {
        result.push(token?);
    }
    println!("{:?}", result);
    assert_eq!(
        result,
        vec![
            Token::Ident(Identifier::Reserved, "fn".to_string()),
            Token::Ident(Identifier::Normal, "sum".to_string()),
            Token::Brace(BraceType::Paren, BraceSide::Left),
            Token::Ident(Identifier::Normal, "x".to_string()),
            Token::Sym(":".to_string()),
            Token::Ident(Identifier::Normal, "i32".to_string()),
            Token::Sep(Separator::Comma),
            Token::Ident(Identifier::Normal, "y".to_string()),
            Token::Sym(":".to_string()),
            Token::Ident(Identifier::Normal, "i32".to_string()),
            Token::Brace(BraceType::Paren, BraceSide::Right),
            Token::Sym("=".to_string()),
            Token::Ident(Identifier::Normal, "x".to_string()),
            Token::Sym("+".to_string()),
            Token::Ident(Identifier::Normal, "y".to_string()),
        ]
    );
    Ok(())
}
