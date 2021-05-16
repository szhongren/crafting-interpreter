use std::fmt::Display;

use super::{expr::Expr, token::Token, token_type::TokenType};

#[derive(PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self::Number(v)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Number(number_value) => number_value.to_string(),
                Value::String(string_value) => string_value.to_string(),
                Value::Bool(bool_value) => bool_value.to_string(),
                Value::Nil => "nil".to_string(),
            }
        )
    }
}

pub struct Interpreter {}

impl<'a> Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&self, expr: Expr) -> Result<Value, String> {
        self.evaluate(expr)
    }

    fn evaluate(&self, expr: Expr) -> Result<Value, String> {
        match expr {
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
        }
    }

    fn urnary(&self, operator: Token, right: Expr) -> Result<Value, String> {
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

    fn binary(&self, left: Expr, operator: Token, right: Expr) -> Result<Value, String> {
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
