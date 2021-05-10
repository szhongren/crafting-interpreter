use std::rc::Rc;

use super::token::Token;

#[derive(Clone, Debug, Copy)]
pub enum LiteralType {
    StringLiteral,
    NumberLiteral,
    NilLiteral,
}

// I could use tuple structs here instead
#[derive(Clone, Debug)]
pub enum Expr<'a> {
    Binary {
        left: Rc<Expr<'a>>,
        operator: Token<'a>,
        right: Rc<Expr<'a>>,
    },
    Grouping {
        expression: Rc<Expr<'a>>,
    },
    Literal {
        literal_type: LiteralType,
        string_literal: Option<&'a str>,
        number_literal: Option<f64>,
    },
    Urnary {
        operator: Token<'a>,
        right: Rc<Expr<'a>>,
    },
}

impl<'a> Expr<'a> {
    pub fn print(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => Self::parenthesize(operator.lexeme, vec![(**left).clone(), (**right).clone()]),
            Expr::Grouping { expression } => {
                Self::parenthesize("group", vec![(**expression).clone()])
            }
            Expr::Literal {
                literal_type,
                string_literal,
                number_literal,
            } => match literal_type {
                LiteralType::NilLiteral => "nil".to_string(),
                LiteralType::NumberLiteral => number_literal.unwrap().to_string(),
                LiteralType::StringLiteral => string_literal.unwrap().to_string(),
            },
            Expr::Urnary { operator, right } => {
                Self::parenthesize(operator.lexeme, vec![(**right).clone()])
            }
        }
    }

    fn parenthesize(name: &'a str, exprs: Vec<Expr>) -> String {
        let expr_string = exprs
            .iter()
            .map(|expr| expr.print())
            .collect::<Vec<String>>()
            .join(" ");
        format!("({} {})", name.to_string(), expr_string)
    }
}

// return expr.accept(this);
// ----------------------------------------------------------------------------------------------------
// <R> R accept(Visitor<R> visitor) {
//     return visitor.visitBinaryExpr(this);
// }
// ----------------------------------------------------------------------------------------------------
// @Override
// public String visitBinaryExpr(Expr.Binary expr) {
// return parenthesize(expr.operator.lexeme,
//                     expr.left, expr.right);
// }
