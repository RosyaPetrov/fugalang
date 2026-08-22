use ast::expr::Expr::{self, Literal};
use crate::ParseError;
use crate::Parser;
use token::TokenKind;

impl<'a> Parser<'a> {
    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        match self.peek_kind() {
            TokenKind::True => {
                self.bump();

                Ok(Expr::Literal(ast::Literal::Bool(true)))
            }

            TokenKind::False => {
                self.bump();

                Ok(Expr::Literal(ast::Literal::Bool(false)))
            }

            _ => Err(self.error("expected expression")),
        }
    }
}