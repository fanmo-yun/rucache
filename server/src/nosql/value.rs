use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Double(f64),
    Str(String)
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{}", v),
            Value::Double(v) => write!(f, "{}", v),
            Value::Str(v) => write!(f, "{}", v),
        }
    }
}
