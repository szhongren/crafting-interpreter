use std::{array::IntoIter, cell::RefCell, collections::HashMap, iter::FromIterator, rc::Rc};

use super::{
    environment::Environment,
    expr::Expr,
    stmt::Stmt,
    token::Token,
    token_type::TokenType,
    value::{Function, Value},
};

pub struct Interpreter {
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let env = Rc::from(RefCell::from(Environment::new(
            HashMap::from_iter(IntoIter::new([(
                "clock".to_string(),
                Value::Callable(Function::new(0)),
            )])),
            Option::None,
        )));
        Self {
            environment: Rc::clone(&env),
            globals: env,
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(*expr)?;
            }
            Stmt::Print(expr) => {
                println!("{}", self.evaluate(*expr)?.to_string());
            }
            Stmt::VariableDeclaration(token, expr) => {
                let eval = self.evaluate(*expr)?;
                self.environment.borrow_mut().define(token.lexeme, eval);
            }
            Stmt::Block(statements) => {
                self.execute_block(statements);
            }
            Stmt::If(condition, then_branch, maybe_else_branch) => {
                let eval = self.evaluate(*condition)?;
                if self.is_truthy(eval) {
                    self.execute(*then_branch)?;
                } else if let Some(else_branch) = *maybe_else_branch {
                    self.execute(else_branch)?;
                }
            }
            Stmt::While(condition, body) => {
                let mut evaluation = self.evaluate(*condition.clone())?;
                while self.is_truthy(evaluation) {
                    self.execute(*body.clone())?;
                    evaluation = self.evaluate(*condition.clone())?;
                }
            }
        };
        Ok(())
    }

    fn execute_block(&mut self, statements: Vec<Stmt>) {
        // set current environment to newly constructed environment
        let previous = self.environment.clone();
        let new_environment =
            Environment::new(HashMap::new(), Option::from(self.environment.clone()));
        self.environment = Rc::from(RefCell::from(new_environment));

        for statement in statements {
            if self.execute(statement).is_err() {
                break;
            }
        }

        // set to original environment with changes
        self.environment = previous;
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Assign(name, value) => {
                let evaluated_value = self.evaluate(*value)?;
                self.environment
                    .borrow_mut()
                    .assign(name, evaluated_value.clone())?;
                Ok(evaluated_value)
            }
            Expr::Binary(left, operator, right) => self.binary(*left, operator, *right),
            Expr::Grouping(group_expr) => self.evaluate(*group_expr),
            Expr::Urnary(operator, right) => self.urnary(operator, *right),
            Expr::StringLiteral(string_literal) => {
                Ok(Value::from(string_literal.clone().to_string()))
            }
            Expr::NumberLiteral(number_literal) => Ok(Value::from(number_literal)),
            Expr::NilLiteral => Ok(Value::Nil),
            Expr::TrueLiteral => Ok(Value::Bool(true)),
            Expr::FalseLiteral => Ok(Value::Bool(false)),
            Expr::Variable(token) => Ok(self.environment.borrow().get(token.lexeme)?),
            Expr::Logical(left, operator, right) => {
                let left_value = self.evaluate(*left)?;
                if TokenType::Or == operator.token_type && self.is_truthy(left_value.clone()) {
                    Ok(left_value)
                } else if TokenType::And == operator.token_type
                    && !self.is_truthy(left_value.clone())
                {
                    Ok(left_value)
                } else {
                    self.evaluate(*right)
                }
            }
            Expr::Call(callee, _, args) => {
                let callee = self.evaluate(*callee)?;
                let mut arguments = Vec::new();
                for arg in args {
                    arguments.push(self.evaluate(arg)?);
                }
                callee.call(self, arguments)
            }
        }
    }

    fn urnary(&mut self, operator: Token, right: Expr) -> Result<Value, String> {
        let right_value = self.evaluate(right)?;
        match operator.token_type {
            TokenType::Minus => match right_value {
                Value::Number(right_number_value) => Ok(Value::from(-right_number_value)),
                _ => Err("unexpected value".to_string()),
            },
            TokenType::Bang => Ok(Value::from(!self.is_truthy(right_value))),
            _ => Err("unexpected token".to_string()),
        }
    }

    fn binary(&mut self, left: Expr, operator: Token, right: Expr) -> Result<Value, String> {
        let left_value = self.evaluate(left)?;
        let right_value = self.evaluate(right)?;
        match operator.token_type {
            TokenType::Greater => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value > right_number_value))
                }
                _ => Err("unexpected value".to_string()),
            },
            TokenType::GreaterEqual => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value >= right_number_value))
                }
                _ => Err("unexpected value".to_string()),
            },
            TokenType::Less => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value < right_number_value))
                }
                _ => Err("unexpected value".to_string()),
            },
            TokenType::LessEqual => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value <= right_number_value))
                }
                _ => Err("unexpected value".to_string()),
            },
            TokenType::Slash => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value / right_number_value))
                }
                _ => Err("unexpected value".to_string()),
            },
            TokenType::Star => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value * right_number_value))
                }
                _ => Err("unexpected value".to_string()),
            },
            TokenType::Minus => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value - right_number_value))
                }
                _ => Err("unexpected value".to_string()),
            },
            TokenType::Plus => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value + right_number_value))
                }
                (Value::String(left_string_value), Value::String(right_string_value)) => Ok(
                    Value::from([left_string_value, right_string_value].join("")),
                ),
                _ => Err("unexpected value".to_string()),
            },
            TokenType::BangEqual => Ok(Value::from(!self.is_equal(left_value, right_value))),
            TokenType::EqualEqual => Ok(Value::from(self.is_equal(left_value, right_value))),
            _ => Err("unexpected token".to_string()),
        }
    }

    fn is_truthy(&self, value: Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Bool(value) => value,
            _ => true,
        }
    }

    fn is_equal(&self, left_value: Value, right_value: Value) -> bool {
        left_value == right_value
    }
}
