use std::fmt;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("(")]
    OpenParen,

    #[token("#(")]
    HashOpenParen,

    #[token(")")]
    CloseParen,

    #[regex(r"[a-zA-Z_!\$%\*\/:<=>\?@^~#](:?[#a-zA-Z0-9_!\$%\*\+\-\.\/<=>\?@^~])*", |lex| lex.slice().to_string())]
    #[regex(r"\|(\\\||[^\|])\|", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"[a-zA-Z_!\$%\*\/<=>\?@^~#](:?[#a-zA-Z0-9_!\$%\*\+\-\.\/<=>\?@^~])*:", |lex| {
        let s = &lex.slice()[..];
        s[..s.len() - 1].to_string()
    })]
    #[regex(r"#:[a-zA-Z_!\$%\*\/:<=>\?@^~#][#a-zA-Z0-9_!\$%\*\+\-\.\/:<=>\?@^~]*", |lex| {
        lex.slice()[2..].to_string()
    })]
    #[regex(r":[a-zA-Z_!\$%\*\/:<=>\?@^~#][#a-zA-Z0-9_!\$%\*\+\-\.\/:<=>\?@^~]*", |lex| {
        lex.slice()[1..].to_string()
    })]
    // #[regex(r"['`]\|(\\\||[^\|])\|", |lex| lex.slice().to_string())]
    KeywordIdent(String),

    #[token("()")]
    Nil,

    #[regex(r"[-+]?[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    Int(i64),

    #[regex(r"[-+]?[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().unwrap())]
    Float(f64),

    #[regex(r#""(\\"|[^"])*""#, |lex| {
        let s = &lex.slice()[1..];
        s[..s.len() - 1].to_string()
    })]
    String(String),

    #[regex(r"#\\(.|newline|space|tab)", |lex| {
        let s = lex.slice();
        match s {
            "newline" => '\n',
            "space" => ' ',
            "tab" => '\t',
            _ => s.chars().next().unwrap()
        }
    })]
    Char(char),

    #[token("#t", |_| true)]
    #[token("#f", |_| false)]
    Bool(bool),

    #[token("'")]
    Quote,

    #[token("`")]
    Quasiquote,

    #[token(",")]
    Unquote,

    #[regex(r";[^\n]*", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let to_string = match self {
            Token::OpenParen => "OpenParen<(>".to_owned(),
            Token::HashOpenParen => "HashOpenParen<#(>".to_owned(),
            Token::CloseParen => "CloseParen<)>".to_owned(),
            Token::Ident(s) => format!("Ident({})", s),
            Token::KeywordIdent(s) => format!("Keyword({})", s),
            Token::Quote => "Quote<'>".to_owned(),
            Token::Quasiquote => "Quasiquote<`>".to_owned(),
            Token::Unquote => "Comma<,>".to_owned(),
            Token::Int(i) => format!("Int({})", i),
            Token::Float(f) => format!("Float({})", f),
            Token::String(s) => format!("String({})", s),
            Token::Char(c) => format!("Char({})", c),
            Token::Bool(b) => format!("Bool({})", b),
            Token::Nil => "Nil<()>".to_owned(),
            Token::Error => "Error".to_owned(),
        };
        write!(f, "{}", to_string)
    }
}
