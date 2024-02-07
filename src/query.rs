use std::str::FromStr;

use derive_getters::Getters;
use derive_new::new;
use once_cell::sync::Lazy;

use crate::{
    ast::{Expr, Filter, Query},
    func::BUILTIN_FUNCS,
    scheme,
    scheme_ast::Value as SchemeValue,
};

#[derive(Debug, Clone, new, Getters)]
struct QueryState {
    query: Query,
    branches: Vec<Box<SchemeValue>>,
}

pub fn handle_query(query: Query, content: String) -> Vec<Box<SchemeValue>> {
    let mut branches = scheme::SchemeParser::new().parse(&content).unwrap();

    for filter in query.filters() {
        branches = handle_filter(filter.as_ref(), &branches);
    }

    branches
}

pub fn handle_filter(filter: &Filter, branches: &[Box<SchemeValue>]) -> Vec<Box<SchemeValue>> {
    match filter {
        Filter::Identity => branches.to_vec(),
        Filter::Key(key) => branches
            .into_iter()
            .cloned()
            .map(|branch| match *branch {
                SchemeValue::List(l) => Box::new(SchemeValue::List(
                    l.into_iter()
                        .skip_while(|k| k.as_ref() != &SchemeValue::Symbol(key.clone()))
                        .collect::<Vec<_>>(),
                )),
                _ => panic!("Expected list"),
            })
            .collect::<Vec<_>>(),
        Filter::Index(index) => branches
            .into_iter()
            .cloned()
            .map(|branch| match *branch {
                SchemeValue::List(l) | SchemeValue::Vector(l) => {
                    let index = if *index < 0 {
                        l.len() as i64 + index
                    } else {
                        *index
                    } as usize;

                    if index < l.len() {
                        l[index].clone()
                    } else {
                        panic!("Index out of bounds")
                    }
                }
                _ => panic!("Expected list"),
            })
            .collect::<Vec<_>>(),
        Filter::Branch(filters) => {
            let mut final_branches = vec![];
            for filter in filters {
                final_branches.extend(handle_filter(filter.as_ref(), branches.as_ref()));
            }

            final_branches
        }
        Filter::ListIter => {
            let mut final_branches = vec![];
            for branch in branches {
                match branch.as_ref() {
                    SchemeValue::List(l) | SchemeValue::Vector(l) => {
                        final_branches.extend(l.clone())
                    }
                    _ => panic!("Expected list"),
                }
            }

            final_branches
        }
        Filter::FuncCall { func, args } => {
            let func = *BUILTIN_FUNCS.get(func.as_str()).unwrap();
            let mut final_branches = vec![];

            let args = args
                .into_iter()
                .map(|arg| match *arg.clone() {
                    Expr::Filter(f) => {
                        let result = handle_filter(&*f, branches);
                        if result.len() == 1 {
                            result[0].clone()
                        } else {
                            Box::new(SchemeValue::List(result))
                        }
                    }
                    Expr::Value(v) => Box::new(v),
                })
                .collect::<Vec<_>>();

            for branch in branches {
                let new_value = func(branch.clone(), args.clone()).unwrap();
                if let Some(value) = new_value {
                    final_branches.push(value);
                }
            }

            final_branches
        }
        Filter::Expr(_) => unimplemented!(),
    }
}
