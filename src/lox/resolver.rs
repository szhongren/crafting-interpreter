use std::collections::HashMap;

use super::{expr::Expr, interpreter::Interpreter, stmt::Stmt, token::Token};

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }

    pub fn resolve(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        for statement in statements {
            self.resolve_statement(statement)?;
        }
        Ok(())
    }

    pub fn resolve_statement(&mut self, statement: Stmt) -> Result<(), String> {
        match statement {
            Stmt::Block(statements) => {
                self.begin_scope()?;
                self.resolve(statements)?;
                self.end_scope()?;
            }
            Stmt::Expression(_) => todo!(),
            Stmt::If(_, _, _) => todo!(),
            Stmt::Print(_) => todo!(),
            Stmt::While(_, _) => todo!(),
            Stmt::VariableDeclaration(name, initializer) => {
                self.declare(*name.clone());
                if *initializer != Expr::NilLiteral {
                    self.resolve_expression(*initializer)?;
                }
                self.define(*name);
            }
            Stmt::FunctionDeclaration(_, _, _) => todo!(),
            Stmt::Return(_, _) => todo!(),
        }
        Ok(())
    }

    pub fn resolve_expression(&self, expression: Expr) -> Result<(), String> {
        match expression {
            Expr::Assign(_, _) => (),
            Expr::Binary(_, _, _) => todo!(),
            Expr::Grouping(_) => todo!(),
            Expr::Urnary(_, _) => todo!(),
            Expr::StringLiteral(_) => todo!(),
            Expr::NumberLiteral(_) => todo!(),
            Expr::NilLiteral => todo!(),
            Expr::TrueLiteral => todo!(),
            Expr::FalseLiteral => todo!(),
            Expr::Variable(_) => todo!(),
            Expr::Logical(_, _, _) => todo!(),
            Expr::Call(_, _, _) => todo!(),
        }
        Ok(())
    }

    pub fn begin_scope(&mut self) -> Result<(), String> {
        self.scopes.push(HashMap::new());
        Ok(())
    }

    pub fn end_scope(&mut self) -> Result<(), String> {
        self.scopes.pop();
        Ok(())
    }

    fn declare(&mut self, name: Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes.last_mut().unwrap().insert(name.lexeme, false);
    }

    fn define(&mut self, name: Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes.last_mut().unwrap().insert(name.lexeme, true);
    }
}
