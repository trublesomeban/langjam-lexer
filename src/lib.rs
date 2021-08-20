mod lexer;

#[cfg(test)]
use lexer::*;

#[test]
fn lex_int() {
    let mut iter = TokenStream::new("3", &[], &[]);
    let token = iter.next().unwrap().unwrap();
    assert_eq!(token, Token::Lit(Literal::Int("3".to_string())));
}

#[test]
fn lex_float() {
    let mut iter = TokenStream::new("3.14", &[], &[]);
    let token = iter.next().unwrap().unwrap();
    assert_eq!(token, Token::Lit(Literal::Float("3.14".to_string())));
}

#[test]
fn lex_string() {
    let mut iter = TokenStream::new("\"hello!\"", &[], &[]);
    let token = iter.next().unwrap().unwrap();
    assert_eq!(token, Token::Lit(Literal::Str("hello!".to_string())));
}

#[test]
fn lex_ident() {
    let mut iter = TokenStream::new("hello", &[], &[]);
    let token = iter.next().unwrap().unwrap();
    assert_eq!(token, Token::Ident(Identifier::Normal("hello".to_string())));
}

#[test]
fn lex_braces() -> Result<(), lexer::Error> {
    let iter = TokenStream::new("[1, 2]", &[], &[]);
    let mut result: Vec<Token> = vec![];
    for token in iter {
        result.push(token?);
    }
    let expected = &[
        Token::Brace(BraceType::Bracket(BraceSide::Left)),
        Token::Lit(Literal::Int("1".to_string())),
        Token::Sep(Separator::Comma),
        Token::Lit(Literal::Int("2".to_string())),
        Token::Brace(BraceType::Bracket(BraceSide::Right)),
        Token::EOF,
    ];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn lex_multi() -> Result<(), lexer::Error> {
    let iter = TokenStream::new("let x = (3.14, 0.86)", &["let"], &["="]);
    let mut result: Vec<Token> = vec![];
    for token in iter {
        result.push(token?);
    }
    let expected = &[
        Token::Ident(Identifier::Reserved("let".to_string())),
        Token::Ident(Identifier::Normal("x".to_string())),
        Token::Sym("=".to_string()),
        Token::Brace(BraceType::Paren(BraceSide::Left)),
        Token::Lit(Literal::Float("3.14".to_string())),
        Token::Sep(Separator::Comma),
        Token::Lit(Literal::Float("0.86".to_string())),
        Token::Brace(BraceType::Paren(BraceSide::Right)),
        Token::EOF,
    ];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn lex_fn() -> Result<(), lexer::Error> {
    let iter = TokenStream::new(
        "fn sum(x: i32, y: i32) = 
		x + y",
        &["fn"],
        &["+", "=", ":"],
    );
    let mut result: Vec<Token> = vec![];
    for token in iter {
        result.push(token?);
    }
    let expected = &[
        Token::Ident(Identifier::Reserved("fn".to_string())),
        Token::Ident(Identifier::Normal("sum".to_string())),
        Token::Brace(BraceType::Paren(BraceSide::Left)),
        Token::Ident(Identifier::Normal("x".to_string())),
        Token::Sym(":".to_string()),
        Token::Ident(Identifier::Normal("i32".to_string())),
        Token::Sep(Separator::Comma),
        Token::Ident(Identifier::Normal("y".to_string())),
        Token::Sym(":".to_string()),
        Token::Ident(Identifier::Normal("i32".to_string())),
        Token::Brace(BraceType::Paren(BraceSide::Right)),
        Token::Sym("=".to_string()),
        Token::Ident(Identifier::Normal("x".to_string())),
        Token::Sym("+".to_string()),
        Token::Ident(Identifier::Normal("y".to_string())),
        Token::EOF,
    ];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn lex_proc() -> Result<(), lexer::Error> {
    let iter = TokenStream::new(
        "proc sum(arr: []i32)  -> i32 {
		let res: i32
		for el in arr {
			res += el 
		}
                ret res
	}",
        &["proc", "let", "for", "in", "ret"],
        &["-", "+", ":", "->", "+="],
    );
    let mut result: Vec<Token> = vec![];
    for token in iter {
        result.push(token?);
    }
    let expected = &[
        Token::Ident(Identifier::Reserved("proc".to_string())),
        Token::Ident(Identifier::Normal("sum".to_string())),
        Token::Brace(BraceType::Paren(BraceSide::Left)),
        Token::Ident(Identifier::Normal("arr".to_string())),
        Token::Sym(":".to_string()),
        Token::Brace(BraceType::Bracket(BraceSide::Left)),
        Token::Brace(BraceType::Bracket(BraceSide::Right)),
        Token::Ident(Identifier::Normal("i32".to_string())),
        Token::Brace(BraceType::Paren(BraceSide::Right)),
        Token::Sym("->".to_string()),
        Token::Ident(Identifier::Normal("i32".to_string())),
        Token::Brace(BraceType::Curly(BraceSide::Left)),
        Token::Ident(Identifier::Reserved("let".to_string())),
        Token::Ident(Identifier::Normal("res".to_string())),
        Token::Sym(":".to_string()),
        Token::Ident(Identifier::Normal("i32".to_string())),
        Token::Ident(Identifier::Reserved("for".to_string())),
        Token::Ident(Identifier::Normal("el".to_string())),
        Token::Ident(Identifier::Reserved("in".to_string())),
        Token::Ident(Identifier::Normal("arr".to_string())),
        Token::Brace(BraceType::Curly(BraceSide::Left)),
        Token::Ident(Identifier::Normal("res".to_string())),
        Token::Sym("+=".to_string()),
        Token::Ident(Identifier::Normal("el".to_string())),
        Token::Brace(BraceType::Curly(BraceSide::Right)),
        Token::Ident(Identifier::Reserved("ret".to_string())),
        Token::Ident(Identifier::Normal("res".to_string())),
        Token::Brace(BraceType::Curly(BraceSide::Right)),
        Token::EOF,
    ];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn invalid_char_error() {
    let mut iter = TokenStream::new("3.14.", &[], &[]);
    match iter.next().unwrap() {
        Err(_e) => {
            //     eprintln!("{}", _e.fmt());
        }
        Ok(_) => panic!("Error did not happen"),
    }
}

#[test]
fn unknown_char_error() {
    let mut iter = TokenStream::new("$", &[], &[]);
    match iter.next().unwrap() {
        Err(_e) => {
            //     eprintln!("{}", _e.fmt());
        }
        Ok(_) => panic!("Error did not happen"),
    }
}
#[test]
fn many_tokens() -> Result<(), lexer::Error> {
    let iter = TokenStream::new("foo bar baz xyz aaaaaa bbbbb cccc dddd eeee -> ffffff gggggg + jjjjjjjj + hjjjjj ++++++++++ ****************** ===== &:[]", &[], &["-", "->", "+", "++","=", "==", "*", "&", ":"]);
    for token in iter {
        token?;
    }
    Ok(())
}