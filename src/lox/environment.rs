use std::collections::HashMap;

use super::expr::Expr;

pub struct Environment<'a> {
    values: HashMap<&'a str, Expr<'a>>,
}

impl<'a> Environment<'a> {
    pub fn define(&mut self, name: &'a str, value: Expr<'a>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Expr, String> {
        let maybe_value = self.values.get(name);
        match maybe_value {
            Some(expr) => Ok(expr.clone()),
            None => Err(format!("Undefined variable: {}", name)),
        }
    }
}
