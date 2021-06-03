use std::fmt::Display;

use super::{expr::Expr, token::Token};

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Box<Expr>),
    If(Box<Expr>, Box<Stmt>, Box<Option<Stmt>>),
    Print(Box<Expr>),
    While(Box<Expr>, Box<Stmt>),
    VariableDeclaration(Box<Token>, Box<Expr>),
    FunctionDeclaration(Token, Vec<Token>, Vec<Stmt>),
    Return(Token, Box<Expr>),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Block(stmts) => {
                write!(f, "(block")?;
                for stmt in stmts {
                    write!(f, " {}", stmt)?;
                }
                write!(f, ")")
            }
            Stmt::Expression(expr) => write!(f, "(expression {})", expr),
            Stmt::If(condition, then_branch, else_branch) => {
                write!(f, "(if {} then {}", condition, then_branch)?;
                if let Some(else_stmt) = else_branch.as_ref() {
                    write!(f, " else {}", else_stmt)?;
                }
                write!(f, ")")
            }
            Stmt::Print(expr) => write!(f, "(print {})", expr),
            Stmt::While(condition, body) => write!(f, "(while {} do {})", condition, body),
            Stmt::VariableDeclaration(name, expr) => write!(f, "(var {} = {})", name.lexeme, expr),
            Stmt::FunctionDeclaration(name, params, stmts) => {
                write!(f, "(fun {} = (", name)?;
                for param in params {
                    write!(f, " {}", param)?;
                }
                write!(f, ") => (")?;
                for stmt in stmts {
                    write!(f, " {}", stmt)?;
                }
                write!(f, "))")
            }
            Stmt::Return(_, value) => write!(f, "(return {})", value),
        }
    }
}
