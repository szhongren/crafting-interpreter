use super::token::Token;

enum LiteralType {
    StringLiteral,
    NumberLiteral,
}

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
        literal_type: LiteralType,
        string_literal: &'a str,
        number_literal: f64,
    },
    Urnary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
}
