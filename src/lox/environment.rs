use std::collections::HashMap;

use super::{token::Token, value::Value};

#[derive(Clone, Debug)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new(values: HashMap<String, Value>, enclosing: Option<Box<Environment>>) -> Self {
        Self { values, enclosing }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> Result<Value, String> {
        // println!("getting {} from {:?}", name, self);
        match self.values.get(&name) {
            Some(value) => Ok(value.clone()),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.get(name),
                None => Err(format!("Undefined variable: {}", name)),
            },
        }
    }

    pub fn assign(&mut self, name: Token, value: Value) -> Result<(), String> {
        // println!("setting {} in {:?} to {:?}", name.lexeme, self, value);
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            return Ok(());
        }

        if self.enclosing.is_some() {
            self.enclosing.as_mut().unwrap().assign(name, value)?;
            return Ok(());
        };

        Err(format!("Undefined variable: '{}'", name.lexeme))
    }
}
