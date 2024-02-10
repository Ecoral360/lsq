use std::collections::HashMap;

use crate::{ast::Expr, scheme::ast::Value as SchemeValue};
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub enum FuncError {
    WrongType(),
}

type BuiltinFuncType = HashMap<
    &'static str,
    fn(Box<SchemeValue>, Vec<Box<SchemeValue>>) -> Result<Option<Box<SchemeValue>>, FuncError>,
>;

macro_rules! car_fns {
    ($map:expr, $($name:literal),+) => {{
        $(
        $map.insert($name, |value, args| {
            let mut v = *value;
            for op in 2..$name.len() {
                let op = $name.len() - op;
                match v {
                    SchemeValue::List(ref l) | SchemeValue::Vector(ref l) => {
                        if &$name[op..op+1] == "a" {
                            v = *l.get(0).cloned().unwrap();
                        } else if &$name[op..op+1] == "d"{
                            v = SchemeValue::List(
                                l.into_iter().skip(1).cloned().collect::<Vec<_>>(),
                            );
                        } else {
                            unreachable!()
                        }
                    }
                    _ => return Err(FuncError::WrongType()),
                }
            }
            Ok(Some(Box::new(v)))
        });
        );+
    }
    };
}

pub static BUILTIN_FUNCS: Lazy<BuiltinFuncType> = Lazy::new(|| {
    let mut map: BuiltinFuncType = HashMap::new();

    car_fns!(
        map, "car", "cdr", "caar", "cadr", "cdar", "cddr", "caaar", "caadr", "cadar", "cdaar",
        "caddr", "cdadr", "cddar", "cdddr", "caaadr", "caaddr", "cadadr", "cdaadr", "cadddr",
        "cdaddr", "cddadr", "cddddr", "caaaar", "caadar", "cadaar", "cdaaar", "caddar", "cdadar",
        "cddaar", "cdddar"
    );

    map.insert("eqv?", |value, args| {
        Ok(Some(Box::new(SchemeValue::Bool(*value == *args[0]))))
    });

    map.insert("=?", |value, args| {
        Ok(Some(Box::new(SchemeValue::Bool(*value == *args[0]))))
    });

    map.insert(">=?", |value, args| {
        Ok(Some(Box::new(SchemeValue::Bool(*value >= *args[0]))))
    });

    map.insert(">?", |value, args| {
        Ok(Some(Box::new(SchemeValue::Bool(*value > *args[0]))))
    });

    map.insert("<=?", |value, args| {
        Ok(Some(Box::new(SchemeValue::Bool(*value <= *args[0]))))
    });

    map.insert("<?", |value, args| {
        Ok(Some(Box::new(SchemeValue::Bool(*value < *args[0]))))
    });

    map.insert("filter", |value, args| match *args[0].clone() {
        SchemeValue::Symbol(v) => {
            let f = *BUILTIN_FUNCS.get(v.as_str()).unwrap();
            let result = f(value.clone(), args[1..].to_vec())?.unwrap();
            if !matches!(*result, SchemeValue::Bool(false)) {
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
        SchemeValue::Bool(b) => {
            if b {
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
        _ => Err(FuncError::WrongType()),
    });

    map.insert("select", |value, args| match *args[0].clone() {
        SchemeValue::Symbol(v) => {
            let f = *BUILTIN_FUNCS.get(v.as_str()).unwrap();
            let result = f(args[1].clone(), args[2..].to_vec())?.unwrap();
            if !matches!(*result, SchemeValue::Bool(false)) {
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
        _ => Err(FuncError::WrongType()),
    });

    map.insert("map", |value, args| match *args[1].clone() {
        SchemeValue::Symbol(v) => {
            let f = *BUILTIN_FUNCS.get(v.as_str()).unwrap();
            let result = f(args[0].clone(), args[2..].to_vec())?.unwrap();
            if !matches!(*result, SchemeValue::Bool(false)) {
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
        _ => Err(FuncError::WrongType()),
    });

    map.insert("debug", |value, args| {
        println!(
            "; is '{} & args is '({})",
            value,
            args.into_iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        Ok(Some(value))
    });

    map
});
