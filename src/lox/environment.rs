use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::value::Value;

#[derive(Clone, Debug)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new(
        values: HashMap<String, Value>,
        enclosing: Option<Rc<RefCell<Environment>>>,
    ) -> Self {
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
                Some(enclosing) => enclosing.borrow().get(name),
                None => Err(format!("Undefined variable: {}", name)),
            },
        }
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<(), String> {
        // println!("setting {} in {:?} to {:?}", name.lexeme, self, value);
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return Ok(());
        }

        if self.enclosing.is_some() {
            self.enclosing
                .as_deref()
                .unwrap()
                .borrow_mut()
                .assign(name, value)?;
            return Ok(());
        };

        Err(format!("Undefined variable: '{}'", name))
    }

    pub fn get_at(&self, distance: usize, name: String) -> Result<Value, String> {
        match self.ancestor(distance).values.get(&name) {
            Some(value) => Ok(value.clone()),
            None => Err("Failed to get value".to_string()),
        }
    }

    pub fn assign_at(&self, distance: usize, name: String, value: Value) {
        self.ancestor(distance)
            .values
            .insert(name.clone(), value.clone());
    }

    fn ancestor(&self, distance: usize) -> Environment {
        let mut environment = (*self).clone();
        for _ in 0..distance {
            environment = environment.enclosing.unwrap().clone().borrow().clone();
        }
        environment
    }
}
