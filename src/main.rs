#![allow(unused)]

mod ast;
mod func;
mod lexer;
mod query;
mod scheme_ast;
mod token;

use std::io::Read;

use clap::Parser;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub lsq, "/src/lsq.rs");
lalrpop_mod!(pub scheme, "/src/scheme.rs");

#[derive(Debug, Parser)]
#[clap(
    name = "lsq",
    version = "1.0",
    author = "Mathis Laroche",
    about = "Like jq but for Lisp"
)]
struct Sq {
    /// The query to execute
    query: String,

    /// The file to read from (if not provided, will read from stdin)
    file: Option<String>,

    #[clap(short, long, default_value = "false")]
    compact: bool,
    #[clap(short, long, default_value = "false")]
    raw: bool,
    #[clap(short = 'C', long, default_value = "true")]
    colorize: bool,

    #[clap(long = "ast", default_value = "false")]
    show_query: bool,

    #[clap(short = 'V', long, default_value = "false")]
    version: bool,
}

fn main() {
    let args = Sq::parse();

    if args.version {
        println!("lsq 0.1.0");
        return;
    }

    let lexer = lexer::Lexer::new(&args.query);
    let result_query = lsq::QueryParser::new().parse(lexer);

    if args.show_query {
        println!("{:?}", result_query.unwrap().filters());
        return;
    }

    let content = match args.file {
        Some(file) => std::fs::read_to_string(file).unwrap(),
        None => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer).unwrap();
            buffer
        }
    };

    let query = result_query.unwrap();
    let branches = query::handle_query(query, content);
    for branch in branches {
        println!("{}", branch);
    }
}
