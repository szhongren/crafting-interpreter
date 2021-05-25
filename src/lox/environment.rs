use std::collections::HashMap;

use super::interpreter::Value;

pub struct Environment<'a> {
    values: HashMap<&'a str, Value>,
}

impl<'a> Environment<'a> {
    pub fn new(values: HashMap<&'a str, Value>) -> Self {
        Self { values }
    }

    pub fn define(&mut self, name: &'a str, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        match self.values.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("Undefined variable: {}", name)),
        }
    }
}
