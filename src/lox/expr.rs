use std::fmt::Display;

use super::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Urnary(Token, Box<Expr>),
    StringLiteral(String),
    NumberLiteral(f64),
    NilLiteral,
    TrueLiteral,
    FalseLiteral,
    Variable(Token),
    Logical(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Assign(name, value) => write!(f, "(assign {} = {})", name.lexeme, value),
            Expr::Binary(left, operator, right) => {
                write!(f, "(binary {} {} {})", left, operator.lexeme, right)
            }
            Expr::Grouping(expression) => write!(f, "(grouping {})", expression),
            Expr::Urnary(operator, right) => write!(f, "(urnary {} {})", operator, right),
            Expr::StringLiteral(literal) => write!(f, "(literal {})", literal),
            Expr::NumberLiteral(literal) => write!(f, "(literal {})", literal),
            Expr::NilLiteral => write!(f, "(literal nil)"),
            Expr::TrueLiteral => write!(f, "(literal true)"),
            Expr::FalseLiteral => write!(f, "(literal false)"),
            Expr::Variable(name) => write!(f, "(variable {})", name.lexeme),
            Expr::Logical(left, operator, right) => {
                write!(f, "(binary {} {} {})", left, operator, right)
            }
            Expr::Call(callee, _, arguments) => {
                write!(f, "(call {}", callee)?;
                for argument in arguments {
                    write!(f, " {}", argument)?;
                }
                write!(f, ")")
            }
        }
    }
}
