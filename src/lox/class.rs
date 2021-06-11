use std::collections::HashMap;
use std::fmt::Display;

use super::{callable::Callable, instance::Instance, value::Value};

#[derive(Clone, PartialEq, Debug)]
pub struct Class {
    name: String,
    methods: HashMap<String, Value>,
}

impl Class {
    pub fn new(name: String, methods: HashMap<String, Value>) -> Self {
        Self { name, methods }
    }

    pub fn find_method(&self, name: &String) -> Option<&Value> {
        self.methods.get(name)
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
        _interpreter: &mut super::interpreter::Interpreter,
        _arguments: Vec<super::value::Value>,
    ) -> Result<super::value::Value, String> {
        Ok(Value::Instance(Instance::new(self.clone())))
    }
}
