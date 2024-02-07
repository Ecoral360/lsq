use derive_getters::Getters;
use derive_new::new;

use crate::scheme_ast;

#[derive(Debug, new, Getters, Clone)]
pub struct Query {
    filters: Vec<Box<Filter>>,
}

#[derive(Debug, Clone)]
pub enum Filter {
    Identity,
    Key(String),
    Index(i64),
    /// x | filter1, filter2, ... | y
    Branch(Vec<Box<Filter>>),
    ListIter,
    Expr(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Expr {}
