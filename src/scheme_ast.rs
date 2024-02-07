use core::fmt;

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
    Char(char),
    List(Vec<Box<Value>>),
    Vector(Vec<Box<Value>>),
    Nil,
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
            Value::Char(c) => format!("#\\{}", c),
            Value::List(l) => format!(
                "({})",
                l.iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Value::Vector(v) => format!(
                "[{}]",
                v.iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Value::Nil => format!("nil"),
        };
        write!(f, "{}", to_string)
    }
}
