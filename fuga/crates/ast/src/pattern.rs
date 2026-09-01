use crate::{
    ast::{Field, Literal},
    expr::Expr,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    // _ =>
    Wildcard,

    Literal(Literal),

    // Some(x) =>. Tuple.1(x)
    Constructor {
        name: String,
        args: Vec<Pattern>,
    },

    // 1: x, y, z | 2:  x: "", y: 2, z: 'c' | 3: x: T, y: T, z: T | 4: "1", "3"
    Tuple(Vec<Field>), // Field

    // x: T := expr

    // 1..100
    Range {
        start: Option<Expr>,
        end: Option<Expr>,
        inclusive: bool,
    },

    // 1 | 2 | 5
    // 1 & 2 & 5
    // !x
    Expr(Box<Expr>),
}
