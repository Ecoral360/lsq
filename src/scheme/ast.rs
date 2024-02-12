use core::fmt;

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Symbol(String),
    Quote(Box<Value>),
    Quasiquote(Box<Value>),
    Unquote(Box<Value>),
    UnquoteSplicing(Box<Value>),
    Char(char),
    List(Vec<Box<Value>>),
    Vector(Vec<Box<Value>>),
    Nil,
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::Int(a), Value::Float(b)) => (*a as f64).partial_cmp(b),
            (Value::Float(a), Value::Int(b)) => a.partial_cmp(&(*b as f64)),
            _ => None,
        }
    }
}

impl Value {
    pub fn is_iterable(&self) -> bool {
        match self {
            Value::List(_) | Value::Vector(_) => true,
            Value::Quote(v)
            | Value::Quasiquote(v)
            | Value::Unquote(v)
            | Value::UnquoteSplicing(v) => v.is_iterable(),
            _ => false,
        }
    }

    pub fn iter_values(&self) -> Option<impl Iterator<Item = &Value>> {
        match self {
            Value::List(l) | Value::Vector(l) => Some(l.iter().map(|v| &**v)),
            Value::Quote(v)
            | Value::Quasiquote(v)
            | Value::Unquote(v)
            | Value::UnquoteSplicing(v) => v.iter_values(),
            _ => None,
        }
    }

    pub fn compact_repr(&self, raw: bool) -> String {
        self.rec_repr_inner(0)
    }

    pub fn data_repr(&self, raw: bool) -> String {
        self.rec_repr_inner(0)
    }

    pub fn code_repr(&self, raw: bool) -> String {
        self.rec_repr_inner(0)
    }

    fn rec_repr_inner(&self, depth: usize) -> String {
        let repr = if self.is_iterable() {
            let mut lst_repr = String::from(if matches!(self, Value::List(_)) {
                "("
            } else {
                "#("
            });

            // We can unwrap here because we know it's an iterable (checked above)
            let l = self.iter_values().unwrap();
            for (i, v) in l.enumerate() {
                if v.is_iterable() {
                    lst_repr.push_str(&v.rec_repr_inner(depth + 1).trim_start());
                    lst_repr.push('\n');
                    lst_repr.push_str(&" ".repeat(depth));
                } else {
                    lst_repr.push_str(&v.rec_repr_inner(depth + 1).trim_start());
                }
                lst_repr.push(' ');
            }
            lst_repr = lst_repr.trim_end().to_string();
            lst_repr.push(')');
            lst_repr
        } else {
            self.to_string()
        };

        format!("{}{}", " ".repeat(depth), repr)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let to_string = match self {
            Value::Int(i) => format!("{}", i),
            Value::Float(f) => format!("{}", f),
            Value::String(s) => format!("\"{}\"", s),
            Value::Bool(b) => format!("{}", b),
            Value::Symbol(s) => format!("{}", s),
            Value::Quote(v) => format!("'{}", v),
            Value::Quasiquote(v) => format!("`{}", v),
            Value::Unquote(v) => format!(",{}", v),
            Value::UnquoteSplicing(v) => format!(",@{}", v),
            Value::Char(c) => format!("#\\{}", c),
            Value::List(l) => format!(
                "({})",
                l.iter()
                    .map(|v| match **v {
                        _ => format!("{}", v),
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Value::Vector(v) => format!(
                "#({})",
                v.iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Value::Nil => format!("()"),
        };
        write!(f, "{}", to_string)
    }
}
