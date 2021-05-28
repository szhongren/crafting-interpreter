use std::collections::HashMap;

use super::{interpreter::Value, token::Token};

#[derive(Clone, Debug)]
pub struct Environment<'a> {
    pub enclosing: Option<Box<Environment<'a>>>,
    values: HashMap<&'a str, Value>,
}

impl<'a> Environment<'a> {
    pub fn new(values: HashMap<&'a str, Value>, enclosing: Option<Box<Environment<'a>>>) -> Self {
        Self { values, enclosing }
    }

    pub fn define(&mut self, name: &'a str, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        println!("getting {} from {:?}", name, self);
        match self.values.get(name) {
            Some(value) => Ok(value.clone()),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.get(name),
                None => Err(format!("me(Undefined variable: {}", name)),
            },
        }
    }

    pub fn assign(&mut self, name: Token<'a>, value: Value) -> Result<(), String> {
        println!("setting {} in {:?} to {:?}", name.lexeme, self, value);
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
