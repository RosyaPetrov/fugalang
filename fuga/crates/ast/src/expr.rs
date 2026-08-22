use crate::ast::Literal;

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

    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg, // -
    Not, // !
}

#[derive(Debug, Clone, Copy)]
pub enum PostfixOp {
    Increment, // ++
    Decrement, // --
}

#[derive(Debug, Clone, Copy)]
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