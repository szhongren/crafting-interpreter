use std::fmt::{Debug, Display};

use super::interpreter::Interpreter;

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Value>) -> Result<Value, String>;
}

#[derive(Clone)]
pub struct Function {
    name: String,
    arity: usize,
    callable: fn(&Interpreter, Vec<Value>) -> Result<Value, String>,
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(fn {} {})", self.name, self.arity)
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arity == other.arity
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(fn {} {})", self.name, self.arity)
    }
}

impl Function {
    pub fn new(
        name: String,
        arity: usize,
        callable: fn(&Interpreter, Vec<Value>) -> Result<Value, String>,
    ) -> Self {
        Self {
            name,
            arity,
            callable,
        }
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(&self, interpreter: &Interpreter, arguments: Vec<Value>) -> Result<Value, String> {
        if arguments.len() != self.arity() {
            return Err(format!(
                "Expected {} arguments but got {} arguments",
                self.arity(),
                arguments.len()
            ));
        }
        (self.callable)(interpreter, arguments)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    Callable(Function),
}

impl Value {
    pub fn call(&self, interpreter: &Interpreter, arguments: Vec<Value>) -> Result<Value, String> {
        if let Value::Callable(function) = self {
            function.call(interpreter, arguments)
        } else {
            Err(format!("Value {} is not callable", self))
        }
    }
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
                Value::Nil => String::from("nil"),
                Value::Callable(callable) => format!("{}", callable),
            }
        )
    }
}
