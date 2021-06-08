use std::fmt::Display;

use super::{callable::Callable, instance::Instance, value::Value};

#[derive(Clone, PartialEq, Debug)]
pub struct Class {
    name: String,
}

impl Class {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(class {})", self.name)
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        interpreter: &mut super::interpreter::Interpreter,
        arguments: Vec<super::value::Value>,
    ) -> Result<super::value::Value, String> {
        Ok(Value::Instance(Instance::new(self.clone())))
    }
}
