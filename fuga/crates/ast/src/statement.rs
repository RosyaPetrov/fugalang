use crate::{declaration::Variable, expression::Expr, pattern::Pattern};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Variable(Variable),
    Expression(Expr),
    If {
        condition: Expr,
        then_branch: Block,
        else_branch: Option<Box<Statement>>,
    },
    Switch {
        expression: Expr,
        cases: Vec<SwitchCase>,
        default: Option<Block>,
    },
    For {
        condition: Option<Expr>,
        body: Block,
    },
    Continue,
    Break,
    Defer(DeferStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase {
    pub pattern: Option<Pattern>,
    pub expression: Option<Expr>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeferStatement {
    pub statement: Option<Box<Statement>>,
    pub expression: Option<Expr>,
}
