use crate::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
    Unit,

    Bool,
    Char,

    I8,
    I16,
    I32,
    I64,
    I128,

    U8,
    U16,
    U32,
    U64,
    U128,

    F32,
    F64,

    Str,

    Generic {
        name: String,
        constraint: Expr,
    },

    // (T1, T2, T3) (X: T1, Y: T2, Z: T3)
    Tuple(Vec<Ty>),

    // (X: T1, Y: T2, Z: T3)
    Field {
        name: String,
        ty: Box<Ty>,
        value: Box<Expr>,
    },

    // Option[T]
    Named {
        name: String,
        generics: Vec<Ty>,
    },

    Reference {
        mutable: bool,
        inner: Box<Ty>,
    },
}
