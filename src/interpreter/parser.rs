use super::token_type::TokenType;

use super::{expr::Expr, token::Token};

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, current: 0 }
    }

    // fn expression(&'a mut self) -> Expr<'a> {
    //     // expression     → equality ;
    //     self.equality()
    // }

    // fn equality(&'a mut self) -> Expr<'a> {
    //     // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    //     let mut expr = self.equality();

    //     while self.match_token_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
    //         let operator = self.previous();
    //         let right = self.equality();
    //         expr = Expr::Binary {
    //             left: Box::from(expr),
    //             operator,
    //             right: Box::from(right),
    //         };
    //     }

    //     expr.to_owned()
    // }

    // fn comparison(&mut self) -> Expr {
    //     // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    //     let mut expr = self.term();

    //     while self.match_token_type(vec![
    //         TokenType::Greater,
    //         TokenType::GreaterEqual,
    //         TokenType::Less,
    //         TokenType::LessEqual,
    //     ]) {
    //         let operator = self.previous();
    //         let right = self.term();
    //         expr = Expr::Binary {
    //             left: Box::from(expr),
    //             operator,
    //             right: Box::from(right),
    //         };
    //     }

    //     expr
    // }

    // fn term(&mut self) -> Expr {
    //     // term           → factor ( ( "-" | "+" ) factor )* ;
    //     let mut expr = self.term();

    //     while self.match_token_type(vec![
    //         TokenType::Greater,
    //         TokenType::GreaterEqual,
    //         TokenType::Less,
    //         TokenType::LessEqual,
    //     ]) {
    //         let operator = self.previous();
    //         let right = self.term();
    //         expr = Expr::Binary {
    //             left: Box::from(expr),
    //             operator,
    //             right: Box::from(right),
    //         };
    //     }

    //     expr
    // }

    // fn facter(&mut self) -> Expr {
    //     // factor         → unary ( ( "/" | "*" ) unary )* ; // instead of making it left-recursive, we make it a flat sequence of mults/divs
    //     let mut expr = self.term();

    //     while self.match_token_type(vec![
    //         TokenType::Greater,
    //         TokenType::GreaterEqual,
    //         TokenType::Less,
    //         TokenType::LessEqual,
    //     ]) {
    //         let operator = self.previous();
    //         let right = self.term();
    //         expr = Expr::Binary {
    //             left: Box::from(expr),
    //             operator,
    //             right: Box::from(right),
    //         };
    //     }

    //     expr
    // }

    // unary          → ( "!" | "-" ) unary // recursive urnary
    //                | primary ;
    // primary        → NUMBER | STRING | "true" | "false" | "nil"
    //                | "(" expression ")" ;

    fn match_token_type(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        match self.is_at_end() {
            true => false,
            false => self.peek().token_type == token_type,
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        // returns current token we haven't consumed yet
        self.tokens[self.current]
    }

    fn previous(&mut self) -> Token {
        // returns previous token we just consumed
        self.tokens[self.current - 1]
    }
}
