use std::fmt;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("|")]
    Pipe,

    #[regex(r"[a-zA-Z_!\$%\*\/:<=>\?@^~][a-zA-Z0-9_!\$%\*\+\-\.\/:<=>\?@^~]*", |lex| lex.slice().to_string())]
    // #[regex(r"['`]\|(\\\||[^\|])\|", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"\.[a-zA-Z_!\$%\*\/:<=>\?@^~][a-zA-Z0-9_!\$%\*\+\-\/:<=>\?@^~]*", |lex| lex.slice()[1..].to_string())]
    #[regex(r"\.\|(\\\||[^\|])*\|", |lex| {
        let s = &lex.slice()[2..];
        s[..s.len() - 1].to_string()
    })]
    KeyIdent(String),

    #[token(".")]
    Dot,

    #[token(",")]
    Comma,

    #[token(".()")]
    DotParen,

    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    Int(i64),

    #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().unwrap())]
    Float(f64),

    #[regex(r#""(\\"|[^"])*""#, |lex| lex.slice().to_string())]
    String(String),

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let to_string = match self {
            Token::OpenParen => "OpenParen<(>".to_owned(),
            Token::CloseParen => "CloseParen<)>".to_owned(),
            Token::Pipe => "Pipe<|>".to_owned(),
            Token::Ident(s) => format!("Ident({})", s),
            Token::KeyIdent(s) => format!("KeyIdent({})", s),
            Token::Dot => "Dot<.>".to_owned(),
            Token::Comma => "Comma<,>".to_owned(),
            Token::DotParen => "DotParen<.()>".to_owned(),
            Token::Int(i) => format!("Int({})", i),
            Token::Float(f) => format!("Float({})", f),
            Token::String(s) => format!("String({})", s),
            Token::Error => "Error".to_owned(),
        };
        write!(f, "{}", to_string)
    }
}
