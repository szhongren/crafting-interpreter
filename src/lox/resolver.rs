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

    pub fn resolve(&mut self, statements: &Vec<Stmt>) -> Result<(), String> {
        for statement in statements {
            self.resolve_statement(statement)?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, statement: &Stmt) -> Result<(), String> {
        match statement {
            Stmt::Block(statements) => {
                self.begin_scope();
                self.resolve(statements)?;
                self.end_scope();
            }
            Stmt::Expression(_) => todo!(),
            Stmt::If(_, _, _) => todo!(),
            Stmt::Print(_) => todo!(),
            Stmt::While(_, _) => todo!(),
            Stmt::VariableDeclaration(name, initializer) => {
                self.declare(&name);
                if **initializer != Expr::NilLiteral {
                    self.resolve_expression(&initializer)?;
                }
                self.define(&name);
            }
            Stmt::FunctionDeclaration(name, _, _) => {
                self.declare(name);
                self.define(name);

                self.resolve_function(statement);
            }
            Stmt::Return(_, _) => todo!(),
        }
        Ok(())
    }

    fn resolve_expression(&self, expression: &Expr) -> Result<(), String> {
        match expression {
            Expr::Assign(name, value) => {
                self.resolve_expression(value)?;
                self.resolve_local(expression, name);
            }
            Expr::Binary(_, _, _) => todo!(),
            Expr::Grouping(_) => todo!(),
            Expr::Urnary(_, _) => todo!(),
            Expr::StringLiteral(_) => todo!(),
            Expr::NumberLiteral(_) => todo!(),
            Expr::NilLiteral => todo!(),
            Expr::TrueLiteral => todo!(),
            Expr::FalseLiteral => todo!(),
            Expr::Variable(name) => {
                if !self.scopes.is_empty()
                    && self.scopes.last().unwrap().get(&name.lexeme).unwrap() == &false
                {
                    return Err(String::from(
                        "Can't read local variable in its own initializer",
                    ));
                }
                self.resolve_local(expression, name);
            }
            Expr::Logical(_, _, _) => todo!(),
            Expr::Call(_, _, _) => todo!(),
        }
        Ok(())
    }

    fn resolve_local(&self, expression: &Expr, name: &Token) {
        // go from the back
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter
                    .resolve(expression, self.scopes.len() - 1 - i);
            }
        }
    }

    fn resolve_function(&mut self, function: &Stmt) -> Result<(), String> {
        self.begin_scope();
        if let Stmt::FunctionDeclaration(_, params, body) = function {
            for param in params {
                self.declare(param);
                self.define(param);
            }
            self.resolve(body)?;
            self.end_scope();
            Ok(())
        } else {
            Err(format!("Unexpected statement {}", function))
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.lexeme.clone(), false);
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.lexeme.clone(), true);
    }
}
