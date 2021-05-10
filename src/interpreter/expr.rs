use super::token::Token;

enum Expr<'a> {
    Binary {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Grouping {
        expression: Box<Expr<'a>>,
    },
    Literal {
        string_literal: &'a str,
        number_literal: f64,
    },
    Urnary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
}
