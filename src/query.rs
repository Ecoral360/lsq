use std::{str::FromStr, usize};

use derive_getters::Getters;
use derive_new::new;
use once_cell::sync::Lazy;

use crate::{
    ast::{Expr, Filter, Query},
    func::BUILTIN_FUNCS,
    lsq,
    scheme::ast::Value as SchemeValue,
};

#[derive(Debug, Clone, new, Getters)]
struct QueryState {
    query: Query,
    branches: Vec<Box<SchemeValue>>,
}

pub fn handle_query(query: Query, content: Vec<Box<SchemeValue>>) -> Vec<Box<SchemeValue>> {
    let mut branches = content;

    for filter in query.filters() {
        branches = handle_filter(filter.as_ref(), &branches);
    }

    branches
}

pub fn handle_query_scm(query: Query, content: SchemeValue) -> Vec<Box<SchemeValue>> {
    let mut branches = vec![Box::new(content)];

    for filter in query.filters() {
        branches = handle_filter(filter.as_ref(), &branches);
    }

    branches
}

pub fn handle_filter(filter: &Filter, branches: &[Box<SchemeValue>]) -> Vec<Box<SchemeValue>> {
    match filter {
        Filter::Identity => branches.to_vec(),
        Filter::Tail(key) => branches
            .into_iter()
            .cloned()
            .map(|branch| match *branch {
                SchemeValue::List(l) => Box::new(SchemeValue::List(
                    l.into_iter()
                        .skip_while(|k| k.as_ref() != &SchemeValue::Symbol(key.clone()))
                        .skip(1)
                        .collect::<Vec<_>>(),
                )),
                SchemeValue::Vector(l) => Box::new(SchemeValue::Vector(
                    l.into_iter()
                        .skip_while(|k| k.as_ref() != &SchemeValue::Symbol(key.clone()))
                        .skip(1)
                        .collect::<Vec<_>>(),
                )),
                _ => panic!("Expected list"),
            })
            .collect::<Vec<_>>(),

        Filter::Head(key) => branches
            .into_iter()
            .cloned()
            .map(|branch| match *branch {
                SchemeValue::List(l) => Box::new(SchemeValue::List(
                    l.into_iter()
                        .take_while(|k| k.as_ref() != &SchemeValue::Symbol(key.clone()))
                        .collect::<Vec<_>>(),
                )),
                SchemeValue::Vector(l) => Box::new(SchemeValue::Vector(
                    l.into_iter()
                        .take_while(|k| k.as_ref() != &SchemeValue::Symbol(key.clone()))
                        .collect::<Vec<_>>(),
                )),
                _ => panic!("Expected list"),
            })
            .collect::<Vec<_>>(),

        Filter::Key(key) => branches
            .into_iter()
            .cloned()
            .map(|branch| match *branch {
                SchemeValue::List(l) | SchemeValue::Vector(l) => l
                    .into_iter()
                    .skip_while(|k| k.as_ref() != &SchemeValue::Symbol(key.clone()))
                    .nth(1)
                    .unwrap()
                    .clone(),
                _ => panic!("Expected list"),
            })
            .collect::<Vec<_>>(),

        Filter::Index(i) => branches
            .into_iter()
            .cloned()
            .map(|branch| match *branch {
                SchemeValue::List(l) | SchemeValue::Vector(l) => {
                    let index = normalize_idx(*i, l.len());
                    if index < l.len() {
                        l[index].clone()
                    } else {
                        panic!("Index out of bounds")
                    }
                }
                _ => panic!("Expected list"),
            })
            .collect::<Vec<_>>(),

        Filter::Slice(start, end) => branches
            .into_iter()
            .cloned()
            .map(|branch| match *branch {
                SchemeValue::List(l) => {
                    let start = start.map(|s| normalize_idx(s, l.len()));
                    let end = end.map(|e| normalize_idx(e, l.len()));

                    Box::new(match (start, end) {
                        (None, None) => SchemeValue::List(l),
                        (Some(start), None) => SchemeValue::List(l[start..].to_vec()),
                        (None, Some(end)) => SchemeValue::List(l[..end].to_vec()),
                        (Some(start), Some(end)) => SchemeValue::List(l[start..end].to_vec()),
                    })
                }
                SchemeValue::Vector(l) => {
                    let start = start.map(|s| normalize_idx(s, l.len()));
                    let end = end.map(|e| normalize_idx(e, l.len()));

                    Box::new(match (start, end) {
                        (None, None) => SchemeValue::Vector(l),
                        (Some(start), None) => SchemeValue::Vector(l[start..].to_vec()),
                        (None, Some(end)) => SchemeValue::Vector(l[..end].to_vec()),
                        (Some(start), Some(end)) => SchemeValue::Vector(l[start..end].to_vec()),
                    })
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

            for branch in branches {
                let args = args
                    .into_iter()
                    .map(|arg| match *arg.clone() {
                        Expr::Filter(f) => {
                            let result = handle_filter(&*f, vec![branch.clone()].as_ref());
                            if result.len() == 1 {
                                result[0].clone()
                            } else {
                                Box::new(SchemeValue::List(result))
                            }
                        }
                        Expr::Value(v) => v,
                    })
                    .collect::<Vec<_>>();
                let new_value = func(branch.clone(), args.clone()).unwrap();
                if let Some(value) = new_value {
                    final_branches.push(value);
                }
            }

            final_branches
        }
        Filter::SubQuery(query) => {
            let mut final_branches = vec![];
            for branch in branches {
                let new_branches = handle_query_scm(*query.clone(), *branch.clone());
                final_branches.extend(new_branches);
            }

            final_branches
        }
        Filter::Expr(_) => unimplemented!(),
    }
}

const fn normalize_idx(index: i64, len: usize) -> usize {
    (if index < 0 { len as i64 + index } else { index } as usize)
}
