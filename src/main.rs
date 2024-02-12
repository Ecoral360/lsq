#![allow(unused)]

mod ast;
mod func;
mod lexer;
mod query;
mod scheme;
mod token;

use std::io::Read;

use anyhow::Result as AnyResult;
use clap::{Parser, ValueEnum};
use lalrpop_util::lalrpop_mod;
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::{SyntaxReference, SyntaxSet},
    util::LinesWithEndings,
};

lalrpop_mod!(pub lsq, "/src/lsq.rs");
lalrpop_mod!(pub scheme_parser, "/src/scheme_parser.rs");

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

    /// Format the output
    #[clap(short, long, default_value = "data")]
    format: FormatOptions,

    #[clap(short = 'M', long, default_value = "false")]
    monochrome: bool,

    #[clap(short, long, default_value = "false")]
    raw: bool,

    #[clap(long = "ast", default_value = "false")]
    show_query: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
enum FormatOptions {
    Data,
    Code,
    Compact,
}

fn main() -> AnyResult<()> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let args = Sq::parse();

    let lexer = lexer::Lexer::new(&args.query);
    let result_query = lsq::QueryParser::new().parse(lexer);

    if args.show_query {
        println!("{:#?}", result_query?.filters());
        return Ok(());
    }

    let content = match args.file {
        Some(ref file) => std::fs::read_to_string(file)
            .map_err(|err| anyhow::anyhow!("Could not read file {}: {}", file, err))?,
        None => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    let query = result_query?;

    let lexer = scheme::lexer::Lexer::new(&content);
    let branches = scheme_parser::SchemeParser::new().parse(lexer)?;

    let theme_set = syntect::highlighting::ThemeSet::load_defaults();

    let branches = query::handle_query(query, branches);

    let branch_str = match args.format {
        FormatOptions::Compact => branches
            .into_iter()
            .map(|b| b.compact_repr(args.raw))
            .collect::<Vec<_>>()
            .join("\n"),
        FormatOptions::Data => branches
            .into_iter()
            .map(|b| b.data_repr(args.raw))
            .collect::<Vec<_>>()
            .join("\n"),
        FormatOptions::Code => branches
            .into_iter()
            .map(|b| b.code_repr(args.raw))
            .collect::<Vec<_>>()
            .join("\n"),
    };

    if args.monochrome {
        println!("{}", branch_str);
    } else {
        let syntax = ps.find_syntax_by_extension("scm").unwrap();
        let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
        for line in LinesWithEndings::from(&branch_str) {
            let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
            let escaped = syntect::util::as_24_bit_terminal_escaped(&ranges[..], false);
            print!("{}", escaped);
        }
        println!();
    }

    Ok(())
}
