use std::collections::HashMap;

use super::{interpreter::Value, token::Token};

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

    pub fn assign(&mut self, name: Token<'a>, value: Value) -> Result<(), String> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            return Ok(());
        }

        Err(format!("Undefined variable: '{}'", name.lexeme))
    }
}
