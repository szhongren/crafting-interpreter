use super::token::Token;

#[derive(Clone, Debug)]
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

impl Expr {
    pub fn print(&self) -> String {
        match self {
            Expr::Assign(name, value) => Self::parenthesize(&name.lexeme, vec![(**value).clone()]),
            Expr::Binary(left, operator, right) => {
                Self::parenthesize(&operator.lexeme, vec![(**left).clone(), (**right).clone()])
            }
            Expr::Grouping(expression) => Self::parenthesize("group", vec![(**expression).clone()]),
            Expr::StringLiteral(literal) => literal.into(),
            Expr::NumberLiteral(literal) => literal.to_string(),
            Expr::TrueLiteral => String::from("true"),
            Expr::FalseLiteral => String::from("false"),
            Expr::NilLiteral => String::from("nil"),
            Expr::Urnary(operator, right) => {
                Self::parenthesize(&operator.lexeme, vec![(**right).clone()])
            }
            Expr::Variable(name) => Self::parenthesize(&name.lexeme, vec![]),
            Expr::Logical(left, operator, right) => {
                Self::parenthesize(&operator.lexeme, vec![(**left).clone(), (**right).clone()])
            }
            Expr::Call(callee, _, arguments) => {
                let exprs = vec![(**callee).clone()];
                Self::parenthesize("\\", [exprs, arguments.to_vec()].concat())
            }
        }
    }

    fn parenthesize(name: &str, exprs: Vec<Expr>) -> String {
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
