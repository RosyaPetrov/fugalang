use crate::declaration::Declaration;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}