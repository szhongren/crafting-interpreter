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
            None => Err(format!("Undefined property '{}'", name)),
        }
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