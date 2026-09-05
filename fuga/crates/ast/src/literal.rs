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