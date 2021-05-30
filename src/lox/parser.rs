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
            let declaration = self.declaration();
            if declaration.is_ok() {
                statements.push(declaration.unwrap());
            } else {
                println!("{}", declaration.unwrap_err());
            }
        }
        Ok(statements)
    }

    fn declaration(&self) -> Result<Stmt, String> {
        // declaration    → varDecl | statement;
        if self.match_token_types(vec![TokenType::Var]) {
            match self.var_declaration() {
                Ok(var_declaration) => Ok(var_declaration),
                Err(err) => {
                    self.synchronize();
                    Err(err)
                }
            }
        } else {
            match self.statement() {
                Ok(stmt) => Ok(stmt),
                Err(err) => {
                    self.synchronize();
                    Err(err)
                }
            }
        }
    }

    fn var_declaration(&self) -> Result<Stmt, String> {
        // varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
        let name = self.consume(TokenType::Identifier, "Expected variable name.")?;

        let initializer = if self.match_token_types(vec![TokenType::Equal]) {
            self.expression()?
        } else {
            Expr::NilLiteral
        };

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration.",
        )?;

        Ok(Stmt::Variable(Box::from(name), Box::from(initializer)))
    }

    fn statement(&self) -> Result<Stmt, String> {
        // statement      → exprStatement | printStatement;
        if self.match_token_types(vec![TokenType::If]) {
            Ok(self.if_statement()?)
        } else if self.match_token_types(vec![TokenType::Print]) {
            Ok(self.print_statement()?)
        } else if self.match_token_types(vec![TokenType::LeftBrace]) {
            Ok(Stmt::Block(self.block()?))
        } else {
            Ok(self.expression_statement()?)
        }
    }

    fn block(&self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression_statement(&self) -> Result<Stmt, String> {
        // exprStatement  → expression ";";
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value")?;
        Ok(Stmt::Expression(Box::from(expression)))
    }

    fn if_statement(&self) -> Result<Stmt, String> {
        // ifStatement    → "if" "(" expression ")" statement ( "else" statement )?;
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after 'if' condition.")?;

        let then_branch = self.statement()?;

        let else_branch = if self.match_token_types(vec![TokenType::Else]) {
            Option::from(self.statement()?)
        } else {
            Option::None
        };

        Ok(Stmt::If(
            Box::from(condition),
            Box::from(then_branch),
            Box::from(else_branch),
        ))
    }

    fn print_statement(&self) -> Result<Stmt, String> {
        // printStatement → "print" expression ";";
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value")?;
        Ok(Stmt::Print(Box::from(expression)))
    }

    fn expression(&self) -> Result<Expr, String> {
        // expression     → assignment ;
        Ok(self.assignment()?)
    }

    fn assignment(&self) -> Result<Expr, String> {
        // assignment     → IDENTIFIER "=" assignment | equality ;
        let expr = self.equality()?;
        if self.match_token_types(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;
            match expr {
                Expr::Variable(name) => return Ok(Expr::Assign(name, Box::from(value))),
                _ => {
                    return Err(format!(
                        "Invalid assignment: {:?} {:?} {:?}",
                        expr, equals, value
                    ))
                }
            }
        };
        Ok(expr)
    }

    fn equality(&self) -> Result<Expr, String> {
        // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
        let mut expr = self.comparison()?;

        while self.match_token_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        Ok(expr)
    }

    fn comparison(&self) -> Result<Expr, String> {
        // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
        let mut expr = self.term()?;

        while self.match_token_types(vec![
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

        while self.match_token_types(vec![TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        Ok(expr)
    }

    fn factor(&self) -> Result<Expr, String> {
        // factor         → unary ( ( "/" | "*" ) unary )* ; // instead of making it left-recursive, we make it a flat sequence of mults/divs
        let mut expr = self.urnary()?;

        while self.match_token_types(vec![TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let right = self.urnary()?;
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        Ok(expr)
    }

    fn urnary(&self) -> Result<Expr, String> {
        // unary          → ( "!" | "-" ) unary // recursive urnary
        //                | primary ;
        if self.match_token_types(vec![TokenType::Bang, TokenType::Minus]) {
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
        if self.match_token_types(vec![TokenType::True]) {
            Ok(Expr::TrueLiteral)
        } else if self.match_token_types(vec![TokenType::False]) {
            Ok(Expr::FalseLiteral)
        } else if self.match_token_types(vec![TokenType::Nil]) {
            Ok(Expr::NilLiteral)
        } else if self.match_token_types(vec![TokenType::Number]) {
            Ok(Expr::NumberLiteral(self.previous().number_literal.unwrap()))
        } else if self.match_token_types(vec![TokenType::String]) {
            Ok(Expr::StringLiteral(self.previous().string_literal.unwrap()))
        } else if self.match_token_types(vec![TokenType::Identifier]) {
            Ok(Expr::Variable(self.previous()))
        } else if self.match_token_types(vec![TokenType::LeftParen]) {
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

    fn match_token_types(&self, token_types: Vec<TokenType>) -> bool {
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
