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
    Get(Box<Expr>, Token),
    Set(Box<Expr>, Token, Box<Expr>),
    This(Token),
    Super(Token, Token),
}

impl std::hash::Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Expr::Assign(a, b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Binary(a, b, c) => {
                a.hash(state);
                b.hash(state);
                c.hash(state);
            }
            Expr::Grouping(a) => {
                a.hash(state);
            }
            Expr::Urnary(a, b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::StringLiteral(a) => {
                format!("Expr::StringLiteral{}", a).hash(state);
            }
            Expr::NumberLiteral(a) => {
                format!("Expr::NumberLiteral{}", a.to_string()).hash(state);
            }
            Expr::NilLiteral => {
                "Expr::NilLiteral".hash(state);
            }
            Expr::TrueLiteral => {
                "Expr::TrueLiteral".hash(state);
            }
            Expr::FalseLiteral => {
                "Expr::FalseLiteral".hash(state);
            }
            Expr::Variable(a) => {
                a.hash(state);
            }
            Expr::Logical(a, b, c) => {
                a.hash(state);
                b.hash(state);
                c.hash(state);
            }
            Expr::Call(a, b, c) => {
                a.hash(state);
                b.hash(state);
                c.hash(state);
            }
            Expr::Get(a, b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Set(a, b, c) => {
                a.hash(state);
                b.hash(state);
                c.hash(state);
            }
            Expr::This(a) => {
                a.hash(state);
            }
            Expr::Super(a, b) => {
                a.hash(state);
                b.hash(state);
            }
        }
    }
}

impl Eq for Expr {}

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
            Expr::Get(object, name) => {
                write!(f, "(get {}.{})", object, name.lexeme)
            }
            Expr::Set(object, name, value) => {
                write!(f, "(set {}.{} = {})", object, name.lexeme, value)
            }
            Expr::This(keyword) => {
                write!(f, "(this {})", keyword)
            }
            Expr::Super(keyword, method) => {
                write!(f, "(super {} {})", keyword, method)
            }
        }
    }
}
