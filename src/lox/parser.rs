use std::cell::{Cell, RefCell};

use super::{stmt::Stmt, token_type::TokenType};

use super::{expr::Expr, token::Literal, token::Token};

pub struct Parser {
    tokens: RefCell<Vec<Token>>,
    current: Cell<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
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

        Ok(Stmt::VariableDeclaration(
            Box::from(name),
            Box::from(initializer),
        ))
    }

    fn statement(&self) -> Result<Stmt, String> {
        // statement      → exprStatement
        //                | forStatement
        //                | ifStatement
        //                | printStatement
        //                | whileStatement
        //                | block;
        if self.match_token_types(vec![TokenType::If]) {
            Ok(self.if_statement()?)
        } else if self.match_token_types(vec![TokenType::For]) {
            Ok(self.for_statement()?)
        } else if self.match_token_types(vec![TokenType::Print]) {
            Ok(self.print_statement()?)
        } else if self.match_token_types(vec![TokenType::While]) {
            Ok(self.while_statement()?)
        } else if self.match_token_types(vec![TokenType::LeftBrace]) {
            Ok(Stmt::Block(self.block()?))
        } else {
            Ok(self.expression_statement()?)
        }
    }

    fn block(&self) -> Result<Vec<Stmt>, String> {
        // block          → "{" declaration* "}";
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

    fn for_statement(&self) -> Result<Stmt, String> {
        // forStatement   → "for"
        //                  "(" (varDecl | exprStatement | ";")
        //                  expression? ";"
        //                  expression? ")"
        //                  statement;
        self.consume(TokenType::LeftParen, "Expected '(' after 'for'")?;
        let initializer = if self.match_token_types(vec![TokenType::Semicolon]) {
            Option::None
        } else if self.match_token_types(vec![TokenType::Var]) {
            Option::from(self.var_declaration()?)
        } else {
            Option::from(self.expression_statement()?)
        };

        let condition = if !self.check(TokenType::Semicolon) {
            Option::from(self.expression()?)
        } else {
            Option::None
        };
        self.consume(TokenType::Semicolon, "Expected ';' after loop condition")?;

        let increment = if !self.check(TokenType::RightParen) {
            Option::from(self.expression()?)
        } else {
            Option::None
        };
        self.consume(TokenType::RightParen, "Expected ')' after for clauses")?;

        let mut body = self.statement()?;

        if let Some(statement) = increment {
            body = Stmt::Block(vec![body, Stmt::Expression(Box::from(statement))]);
        }

        body = Stmt::While(
            if let None = condition {
                Box::from(Expr::TrueLiteral)
            } else {
                Box::from(condition.unwrap())
            },
            Box::from(body),
        );

        if let Some(statement) = initializer {
            body = Stmt::Block(vec![statement, body]);
        }

        Ok(body)
    }

    fn print_statement(&self) -> Result<Stmt, String> {
        // printStatement → "print" expression ";";
        let expression = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value")?;
        Ok(Stmt::Print(Box::from(expression)))
    }

    fn while_statement(&self) -> Result<Stmt, String> {
        // whileStatement → "while" "(" expression ")" statement;
        self.consume(TokenType::LeftParen, "Expected '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after 'while'")?;
        let body = self.statement()?;
        Ok(Stmt::While(Box::from(condition), Box::from(body)))
    }

    fn expression(&self) -> Result<Expr, String> {
        // expression     → assignment ;
        self.assignment()
    }

    fn assignment(&self) -> Result<Expr, String> {
        // assignment     → IDENTIFIER "=" assignment | logic_or ;
        let expr = self.or()?;
        if self.match_token_types(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;
            match expr {
                Expr::Variable(name) => return Ok(Expr::Assign(name, Box::from(value))),
                _ => {
                    return Err(format!(
                        "Invalid assignment: {} {:?} {}",
                        expr, equals, value
                    ))
                }
            }
        };
        Ok(expr)
    }

    fn or(&self) -> Result<Expr, String> {
        // logic_or       → logic_and ( "or" logic_and )*;
        let mut expr = self.and()?;

        while self.match_token_types(vec![TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(Box::from(expr), operator, Box::from(right));
        }

        Ok(expr)
    }

    fn and(&self) -> Result<Expr, String> {
        // logic_and      → equality ( "and" equality )*;
        let mut expr = self.equality()?;

        while self.match_token_types(vec![TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Box::from(expr), operator, Box::from(right));
        }

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
        //                | call ;
        if self.match_token_types(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.urnary()?;
            Ok(Expr::Urnary(operator, Box::from(right)))
        } else {
            self.call()
        }
    }

    fn call(&self) -> Result<Expr, String> {
        // call           → primary ( "(" arguments? ")" )* ;
        let mut expr = self.primary()?;
        loop {
            if self.match_token_types(vec![TokenType::LeftParen]) {
                let mut arguments = Vec::new();

                if !self.check(TokenType::RightParen) {
                    while {
                        if arguments.len() >= 255 {
                            return Err(format!(
                                "Can't have more than 255 arguments: {:?}",
                                self.peek()
                            ));
                        }
                        arguments.push(self.expression()?);
                        self.match_token_types(vec![TokenType::Comma])
                    } {}
                }

                let paren = self.consume(TokenType::RightParen, "Expected ')' after arguments")?;

                expr = Expr::Call(Box::from(expr), paren, arguments);
            } else {
                break;
            }
        }
        Ok(expr)
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
            if let Literal::Number(number_literal) = self.previous().literal.unwrap() {
                Ok(Expr::NumberLiteral(number_literal))
            } else {
                Err("Expected number literal".to_string())
            }
        } else if self.match_token_types(vec![TokenType::String]) {
            if let Literal::String(string_literal) = self.previous().literal.unwrap() {
                Ok(Expr::StringLiteral(string_literal))
            } else {
                Err("Expected string literal".to_string())
            }
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
        self.tokens.to_owned().into_inner()[self.current.get()].clone()
    }

    fn previous(&self) -> Token {
        // returns previous token we just consumed
        self.tokens.to_owned().into_inner()[self.current.get() - 1].clone()
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
