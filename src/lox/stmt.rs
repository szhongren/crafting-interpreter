use super::{expr::Expr, token::Token};

#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Box<Expr>),
    If(Box<Expr>, Box<Stmt>, Box<Option<Stmt>>),
    Print(Box<Expr>),
    While(Box<Expr>, Box<Stmt>),
    VariableDeclaration(Box<Token>, Box<Expr>),
}
