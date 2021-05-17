use std::cell::{Cell, RefCell};

use super::{stmt::Stmt, token_type::TokenType};

use super::{expr::Expr, token::Token};

pub struct Parser<'a> {
    tokens: RefCell<Vec<Token<'a>>>,
    current: Cell<usize>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens: RefCell::new(tokens),
            current: Cell::new(0),
        }
    }

    pub fn parse(&self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&self) -> Result<Stmt, String> {
        if self.match_token_type(vec![TokenType::Print]) {
            Ok(self.print_statement()?)
        } else {
            Ok(self.expression_statement()?)
        }
    }

    fn expression_statement(&self) -> Result<Stmt, String> {
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value")?;
        Ok(Stmt::Expression(Box::from(expression)))
    }

    fn print_statement(&self) -> Result<Stmt, String> {
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value")?;
        Ok(Stmt::Print(Box::from(expression)))
    }

    fn expression(&self) -> Result<Expr, String> {
        // expression     → equality ;
        Ok(self.equality()?)
    }

    fn equality(&self) -> Result<Expr, String> {
        // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
        let mut expr = self.comparison()?;

        while self.match_token_type(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        Ok(expr)
    }

    fn comparison(&self) -> Result<Expr, String> {
        // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
        let mut expr = self.term()?;

        while self.match_token_type(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        Ok(expr)
    }

    fn term(&self) -> Result<Expr, String> {
        // term           → factor ( ( "-" | "+" ) factor )* ;
        let mut expr = self.factor()?;

        while self.match_token_type(vec![TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        Ok(expr)
    }

    fn factor(&self) -> Result<Expr, String> {
        // factor         → unary ( ( "/" | "*" ) unary )* ; // instead of making it left-recursive, we make it a flat sequence of mults/divs
        let mut expr = self.urnary()?;

        while self.match_token_type(vec![TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let right = self.urnary()?;
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        Ok(expr)
    }

    fn urnary(&self) -> Result<Expr, String> {
        // unary          → ( "!" | "-" ) unary // recursive urnary
        //                | primary ;
        if self.match_token_type(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.urnary()?;
            Ok(Expr::Urnary(operator, Box::from(right)))
        } else {
            self.primary()
        }
    }

    fn primary(&self) -> Result<Expr, String> {
        // primary        → NUMBER | STRING | "true" | "false" | "nil"
        //                | "(" expression ")" ;
        if self.match_token_type(vec![TokenType::True]) {
            Ok(Expr::TrueLiteral)
        } else if self.match_token_type(vec![TokenType::False]) {
            Ok(Expr::FalseLiteral)
        } else if self.match_token_type(vec![TokenType::Nil]) {
            Ok(Expr::NilLiteral)
        } else if self.match_token_type(vec![TokenType::Number]) {
            Ok(Expr::NumberLiteral(self.previous().number_literal.unwrap()))
        } else if self.match_token_type(vec![TokenType::String]) {
            Ok(Expr::StringLiteral(self.previous().string_literal.unwrap()))
        } else if self.match_token_type(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "expected ')' after expression.")?;
            Ok(Expr::Grouping(Box::new(expr)))
        } else {
            let errored_token = self.peek();
            Err(format!(
                "Line {}: Found an unexpected token \"{}\" with type {:?}",
                errored_token.line, errored_token.lexeme, errored_token.token_type,
            ))
        }
    }

    fn consume(&self, token_type: TokenType, message: &str) -> Result<Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            let errored_token = self.peek();
            Err(format!(
                "Line {}: Found an unexpected token \"{}\" with type {:?}, {}",
                errored_token.line, errored_token.lexeme, errored_token.token_type, message
            ))
        }
    }

    fn match_token_type(&self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&self) -> Token {
        if !self.is_at_end() {
            self.current.set(self.current.get() + 1);
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        // returns current token we haven't consumed yet
        self.tokens.to_owned().into_inner()[self.current.get()]
    }

    fn previous(&self) -> Token {
        // returns previous token we just consumed
        self.tokens.to_owned().into_inner()[self.current.get() - 1]
    }

    fn synchronize(&self) {
        // discards tokens until we find a statement boundary
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => (),
            }

            self.advance();
        }
    }
}
