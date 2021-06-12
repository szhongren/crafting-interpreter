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
        match self.find_method(&"init".to_string()) {
            Some(initializer_value) => match initializer_value {
                Value::Function(function) => function.arity(),
                _ => 0,
            },
            None => 0,
        }
    }

    fn call(
        &self,
        _interpreter: &mut super::interpreter::Interpreter,
        _arguments: Vec<super::value::Value>,
    ) -> Result<super::value::Value, String> {
        let instance = Instance::new(self.clone());
        let initializer = self.find_method(&"init".to_string());
        if let Some(initializer_value) = initializer {
            initializer_value.bind(&instance)?;
        }
        Ok(Value::Instance(instance))
    }
}
