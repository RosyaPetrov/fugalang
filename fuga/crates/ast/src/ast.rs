use crate::{expr::Expr, typ::Typ};

pub struct Program {}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Complex { real: f64, imaginary: f64 },
    String(String),
    RawString(String),
    Char(char),
    Bool(bool),
}

// name ty
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub typ: Option<Box<Typ>>,    // if typ == none => x: expr == 2
    pub value: Option<Box<Expr>>, // if type none and value none == 1
    // pub pattern: Option<Box<Pattern>>, // if pattern none and value none == 1
    pub is_public: bool,
}
