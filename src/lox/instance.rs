use std::{collections::HashMap, fmt::Display};

use super::{class::Class, value::Value};

#[derive(Clone, Debug, PartialEq)]
pub struct Instance {
    klass: Class,
    fields: HashMap<String, Value>,
}

impl Instance {
    pub fn new(klass: Class) -> Self {
        Self {
            klass,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: String) -> Result<Value, String> {
        let value = self.fields.get(&name);
        match value {
            Some(result) => Ok(result.clone()),
            None => match self.klass.find_method(&name) {
                Some(method) => Ok(method.clone()),
                None => Err(format!("Undefined property '{}'", name)),
            },
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.fields.insert(name, value);
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(instance {}, {:?})",
            self.klass,
            self.fields.keys().collect::<Vec<&String>>()
        )
    }
}
