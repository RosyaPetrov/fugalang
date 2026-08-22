pub struct Program {}

pub enum Literal {
    Integer(i64),
    Float(f64),
    Complex(f64),
    String(String),
    RawString(String),
    Char(char),
    Bool(bool),
}
