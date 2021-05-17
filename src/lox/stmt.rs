use super::expr::Expr;

#[derive(Clone, Debug)]
pub enum Stmt<'a> {
    Expression(Box<Expr<'a>>),
    Print(Box<Expr<'a>>),
}
