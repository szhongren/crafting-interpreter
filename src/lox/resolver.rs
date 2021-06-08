use std::collections::HashMap;

use super::{expr::Expr, interpreter::Interpreter, stmt::Stmt, token::Token};

#[derive(Clone, Copy, PartialEq)]
enum FunctionType {
    Function,
    None,
}

pub struct Resolver<'a> {
    interpreter: &'a Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
        }
    }

    pub fn resolve(&mut self, statements: &Vec<Stmt>) -> Result<(), String> {
        for statement in statements {
            self.resolve_statement(statement)?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, statement: &Stmt) -> Result<(), String> {
        // println!("resolving: {}", statement);
        match statement {
            Stmt::Block(statements) => {
                self.begin_scope();
                self.resolve(statements)?;
                self.end_scope();
            }
            Stmt::Expression(expression) => {
                self.resolve_expression(expression)?;
            }
            Stmt::If(condition, then_branch, else_branch) => {
                self.resolve_expression(condition)?;
                self.resolve_statement(then_branch)?;
                if let Some(else_statement) = &**else_branch {
                    self.resolve_statement(else_statement)?;
                }
            }
            Stmt::Print(expression) => {
                self.resolve_expression(expression)?;
            }
            Stmt::While(condition, statement) => {
                self.resolve_expression(condition)?;
                self.resolve_statement(statement)?;
            }
            Stmt::ClassDeclaration(name, _) => {
                self.declare(name)?;
                self.define(name);
            }
            Stmt::VariableDeclaration(name, initializer) => {
                self.declare(name)?;
                if **initializer != Expr::NilLiteral {
                    self.resolve_expression(initializer)?;
                }
                self.define(name);
            }
            Stmt::FunctionDeclaration(name, _, _) => {
                self.declare(name)?;
                self.define(name);

                self.resolve_function(statement, &FunctionType::Function)?;
            }
            Stmt::Return(_, value) => {
                if self.current_function == FunctionType::None {
                    return Err("Can't return from top level code".to_string());
                }
                if **value != Expr::NilLiteral {
                    self.resolve_expression(value)?;
                }
            }
        }
        Ok(())
    }

    fn resolve_expression(&self, expression: &Expr) -> Result<(), String> {
        // println!("resolving: {}", expression);
        match expression {
            Expr::Assign(name, value) => {
                self.resolve_expression(value)?;
                self.resolve_local(expression, name);
            }
            Expr::Binary(left, _, right) => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            Expr::Grouping(expression) => {
                self.resolve_expression(expression)?;
            }
            Expr::Urnary(_, right) => {
                self.resolve_expression(right)?;
            }
            Expr::StringLiteral(_) => (),
            Expr::NumberLiteral(_) => (),
            Expr::NilLiteral => (),
            Expr::TrueLiteral => (),
            Expr::FalseLiteral => (),
            Expr::Variable(name) => {
                if !self.scopes.is_empty()
                    && self.scopes.last().unwrap().get(&name.lexeme) == Some(&false)
                {
                    return Err(String::from(
                        "Can't read local variable in its own initializer",
                    ));
                }
                self.resolve_local(expression, name);
            }
            Expr::Logical(left, _, right) => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            Expr::Call(callee, _, arguments) => {
                self.resolve_expression(callee)?;
                for argument in arguments {
                    self.resolve_expression(argument)?;
                }
            }
        }
        Ok(())
    }

    fn resolve_local(&self, expression: &Expr, name: &Token) {
        // go from the back
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter
                    .resolve(expression, self.scopes.len() - 1 - i);
                return;
            }
        }
    }

    fn resolve_function(
        &mut self,
        function: &Stmt,
        function_type: &FunctionType,
    ) -> Result<(), String> {
        let enclosing_function = function_type;
        self.begin_scope();
        if let Stmt::FunctionDeclaration(_, params, body) = function {
            self.current_function = *function_type;
            for param in params {
                self.declare(param)?;
                self.define(param);
            }
            self.resolve(body)?;
            self.end_scope();
            self.current_function = *enclosing_function;
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

    fn declare(&mut self, name: &Token) -> Result<(), String> {
        if self.scopes.is_empty() {
            return Ok(());
        }

        if self.scopes.last().unwrap().contains_key(&name.lexeme) {
            return Err("Variable with this name already exists in this scope.".to_string());
        }

        // means that the variable assignment exists and we know about it
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.lexeme.clone(), false);

        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        // means that the variable has been assigned a value
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.lexeme.clone(), true);
    }
}
