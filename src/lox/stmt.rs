use super::{expr::Expr, token::Token};

#[derive(Clone, Debug)]
pub enum Stmt<'a> {
    Expression(Box<Expr<'a>>),
    Print(Box<Expr<'a>>),
    Variable(Box<Token<'a>>, Box<Expr<'a>>),
    Block(Vec<Stmt<'a>>),
}
