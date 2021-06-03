use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use super::{environment::Environment, interpreter::Interpreter, stmt::Stmt, value::Value};

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, String>;
}

#[derive(Clone)]
pub struct NativeFunction {
    name: String,
    arity: usize,
    callable: fn(&Interpreter, Vec<Value>) -> Result<Value, String>,
}

impl Display for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(fn {} {})", self.name, self.arity)
    }
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arity == other.arity
    }
}

impl Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(nativefn {} {})", self.name, self.arity)
    }
}

impl NativeFunction {
    pub fn new(
        name: String,
        arity: usize,
        callable: fn(&Interpreter, Vec<Value>) -> Result<Value, String>,
    ) -> Self {
        Self {
            name,
            arity,
            callable,
        }
    }
}

impl Callable for NativeFunction {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, String> {
        if arguments.len() != self.arity() {
            return Err(format!(
                "Expected {} arguments but got {} arguments",
                self.arity(),
                arguments.len()
            ));
        }
        (self.callable)(interpreter, arguments)
    }
}

#[derive(Clone)]
pub struct Function {
    declaration: Stmt,
}

impl Function {
    pub fn new(declaration: Stmt) -> Self {
        if let Stmt::FunctionDeclaration(_, _, _) = declaration {
            Self { declaration }
        } else {
            panic!()
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Stmt::FunctionDeclaration(name, parameters, _) = &self.declaration {
            write!(f, "(fn {}(", name.lexeme)?;
            for parameter in parameters {
                write!(f, " {}", parameter)?;
            }
            write!(f, "))")
        } else {
            panic!()
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.declaration == other.declaration
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Stmt::FunctionDeclaration(name, parameters, _) = &self.declaration {
            write!(f, "(fn {}(", name.lexeme)?;
            for parameter in parameters {
                write!(f, " {}", parameter)?;
            }
            write!(f, "))")
        } else {
            panic!()
        }
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        if let Stmt::FunctionDeclaration(_, parameters, _) = &self.declaration {
            parameters.len()
        } else {
            panic!()
        }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, String> {
        if arguments.len() != self.arity() {
            return Err(format!(
                "Expected {} arguments but got {} arguments",
                self.arity(),
                arguments.len()
            ));
        }
        let mut environment = Environment::new(HashMap::new(), Some(interpreter.globals.clone()));
        if let Stmt::FunctionDeclaration(_, parameters, body) = &self.declaration {
            for (parameter, argument) in parameters.iter().zip(arguments) {
                environment.define(parameter.lexeme.clone(), argument);
            }
            interpreter.execute_block(body.to_vec(), environment);
            Ok(Value::Nil)
        } else {
            panic!()
        }
    }
}
