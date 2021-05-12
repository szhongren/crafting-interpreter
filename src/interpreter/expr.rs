use super::token::Token;

// I could use tuple structs here instead
#[derive(Clone, Debug)]
pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
    StringLiteral(&'a str),
    NumberLiteral(f64),
    NilLiteral,
    Urnary(Token<'a>, Box<Expr<'a>>),
}

impl<'a> Expr<'a> {
    pub fn print(&self) -> String {
        match self {
            Expr::Binary(left, operator, right) => {
                Self::parenthesize(operator.lexeme, vec![(**left).clone(), (**right).clone()])
            }
            Expr::Grouping(expression) => Self::parenthesize("group", vec![(**expression).clone()]),
            Expr::NilLiteral => "nil".to_string(),
            Expr::StringLiteral(literal) => literal.to_string(),
            Expr::NumberLiteral(literal) => literal.to_string(),
            Expr::Urnary(operator, right) => {
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
