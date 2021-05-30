use super::{expr::Expr, token::Token};

#[derive(Clone, Debug)]
pub enum Stmt<'a> {
    Block(Vec<Stmt<'a>>),
    Expression(Box<Expr<'a>>),
    If(Box<Expr<'a>>, Box<Stmt<'a>>, Box<Option<Stmt<'a>>>),
    Print(Box<Expr<'a>>),
    While(Box<Expr<'a>>, Box<Stmt<'a>>),
    VariableDeclaration(Box<Token<'a>>, Box<Expr<'a>>),
}
