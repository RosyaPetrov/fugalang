use crate::ast::Literal;
use crate::decl::Param;
use crate::ty::Ty;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Name(String),

    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    Postfix {
        expr: Box<Expr>,
        op: PostfixOp,
    },

    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    // foo[...](arg1: N, arg2: Y)
    Call {
        name: String,
        generics: Vec<Ty>, // ? Generic
        args: Vec<Param>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg, // -
    Not, // !
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PostfixOp {
    Increment, // ++
    Decrement, // --
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /

    Eq, // ==
    Ne, // !=

    Lt, // <
    Gt, // >

    And, // &&
    Or,  // ||
}

impl BinaryOp {
    pub fn precedence(self) -> u8 {
        match self {
            BinaryOp::Or => 1,
            BinaryOp::And => 2,

            BinaryOp::Eq | BinaryOp::Ne => 3,

            BinaryOp::Lt | BinaryOp::Gt => 4,

            BinaryOp::Add | BinaryOp::Sub => 5,

            BinaryOp::Mul | BinaryOp::Div => 6,
        }
    }
}
