use std::fmt::{Debug, Display};

use super::{
    callable::{Callable, Function, NativeFunction},
    class::Class,
    interpreter::Interpreter,
};

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    NativeFunction(NativeFunction),
    Function(Function),
    Class(Class),
}

impl Value {
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, String> {
        if let Value::NativeFunction(function) = self {
            function.call(interpreter, arguments)
        } else if let Value::Function(function) = self {
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
                Value::NativeFunction(callable) => format!("{}", callable),
                Value::Function(callable) => format!("{}", callable),
                Value::Class(klass) => format!("{}", klass),
            }
        )
    }
}
