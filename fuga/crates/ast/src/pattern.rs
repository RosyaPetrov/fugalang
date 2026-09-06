use crate::{expression::Expr, literal::Literal, path::Path};

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Wildcard,
    Name(String),
    Literal(Literal),
    Constructor {
        path: Path,
        args: Vec<Pattern>,
    },
    Tuple(Vec<Pattern>),
    Record {
        fields: Vec<PatternField>,
    },
    Range {
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
        inclusive: bool,
    },
    Expression(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PatternField {
    pub name: String,
    pub pattern: Pattern,
}
