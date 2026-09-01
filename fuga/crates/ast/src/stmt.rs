use crate::{expr::Expr, pattern::Pattern, typ::Typ};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Var(Var),

    // if expr {} else if expe {}
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    Switch {
        expr: Box<Expr>,
        cases: Vec<Stmt>,
        defualt: Option<Box<Stmt>>,
    },

    // case
    Case {
        pattern: Option<Pattern>,
        expr: Option<Box<Expr>>,
        body: Box<Stmt>,
    },

    For {
        condition: Option<Box<Expr>>,
        body: Vec<Box<Stmt>>,
    },

    Continue,
    Break,

    Defer(Option<Box<Stmt>>, Option<Box<Expr>>),
}

// let x: T := expr
// const x: T := expr
// mut x: T := expr
#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub name: String,
    pub typ: Option<Box<Typ>>,
    pub value: Option<Box<Expr>>,
    pub is_mutable: bool,
    pub is_comptime: bool,
}
