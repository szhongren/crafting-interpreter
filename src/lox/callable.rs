use std::fmt::{Debug, Display};

use super::{interpreter::Interpreter, value::Value};

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
