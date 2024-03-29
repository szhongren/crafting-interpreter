use std::{
    array::IntoIter,
    cell::RefCell,
    collections::HashMap,
    iter::FromIterator,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use super::{
    callable::{Function, NativeFunction},
    class::Class,
    environment::Environment,
    expr::Expr,
    stmt::Stmt,
    token::Token,
    token_type::TokenType,
    value::Value,
};

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    locals: Rc<RefCell<HashMap<Expr, usize>>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let env = Rc::from(RefCell::from(Environment::new(
            HashMap::from_iter(IntoIter::new([(
                "clock".to_string(),
                Value::NativeFunction(NativeFunction::new("clock".to_string(), 0, |_, _| {
                    let start = SystemTime::now();
                    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();

                    Ok(Value::Number(since_the_epoch.as_millis() as f64))
                })),
            )])),
            None,
        )));
        Self {
            environment: env.clone(),
            globals: env,
            locals: Rc::from(RefCell::from(HashMap::new())),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), Value> {
        for stmt in stmts {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: Stmt) -> Result<(), Value> {
        match stmt.clone() {
            Stmt::Expression(expr) => {
                self.evaluate(*expr)?;
            }
            Stmt::Print(expr) => {
                println!("{}", self.evaluate(*expr)?.to_string());
            }
            Stmt::VariableDeclaration(name, initializer) => {
                let eval = self.evaluate(*initializer)?;
                self.environment.borrow_mut().define(name.lexeme, eval);
            }
            Stmt::Block(statements) => {
                let new_environment =
                    Environment::new(HashMap::new(), Some(self.environment.clone()));
                self.execute_block(statements, new_environment)?;
            }
            Stmt::If(condition, then_branch, maybe_else_branch) => {
                let eval = self.evaluate(*condition)?;
                if self.is_truthy(eval) {
                    self.execute(*then_branch)?;
                } else if let Some(else_branch) = *maybe_else_branch {
                    self.execute(else_branch)?;
                }
            }
            Stmt::While(condition, body) => {
                let mut evaluation = self.evaluate(*condition.clone())?;
                while self.is_truthy(evaluation) {
                    self.execute(*body.clone())?;
                    evaluation = self.evaluate(*condition.clone())?;
                }
            }
            Stmt::ClassDeclaration(name, superclass, methods) => {
                let mut superklass = None;
                let original_environment = self.environment.clone();
                if let Some(superclass_expr) = superclass.clone() {
                    let superclass_eval = self.evaluate(superclass_expr)?;
                    match superclass_eval {
                        Value::Class(superklass_object) => {
                            superklass = Some(Box::from(superklass_object))
                        }
                        _ => return Err(Value::Nil),
                    }
                }
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), Value::Nil);

                if let Some(superclass_expr) = superclass.clone() {
                    let superclass_eval = self.evaluate(superclass_expr)?;
                    self.environment = Rc::from(RefCell::from(Environment::new(
                        HashMap::new(),
                        Some(self.environment.clone()),
                    )));
                    self.environment
                        .borrow_mut()
                        .define("super".to_string(), superclass_eval);
                }
                let mut methods_map = HashMap::new();
                for method in methods {
                    if let Stmt::FunctionDeclaration(name, _, _) = &method {
                        methods_map.insert(
                            name.lexeme.clone(),
                            Value::Function(Function::new(
                                method.clone(),
                                self.environment.clone(),
                                name.lexeme.clone() == "init".to_string(),
                            )),
                        );
                    }
                }
                let klass = Class::new(name.lexeme.clone(), superklass, methods_map);

                if let Some(_) = superclass.clone() {
                    self.environment = original_environment;
                }

                self.environment
                    .borrow_mut()
                    .assign(name.lexeme, Value::Class(klass))?;
            }
            Stmt::FunctionDeclaration(name, _, _) => {
                let function =
                    Value::Function(Function::new(stmt, self.environment.clone(), false));
                self.environment.borrow_mut().define(name.lexeme, function);
            }
            Stmt::Return(_, value) => {
                let mut return_value = Value::Nil;
                if *value != Expr::NilLiteral {
                    return_value = self.evaluate(*value)?;
                };
                return Err(return_value);
            }
        };
        Ok(())
    }

    pub fn execute_block(
        &mut self,
        statements: Vec<Stmt>,
        new_environment: Environment,
    ) -> Result<(), Value> {
        // set current environment to newly constructed environment
        let previous = self.environment.clone();
        self.environment = Rc::from(RefCell::from(new_environment));

        for statement in statements {
            let result = self.execute(statement);
            if let Err(value) = result {
                self.environment = previous;
                return Err(value);
            }
        }

        // set to original environment with changes
        self.environment = previous;
        Ok(())
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Value, String> {
        match expr.clone() {
            Expr::Assign(name, value) => {
                let evaluated_value = self.evaluate(*value)?;
                match self.locals.borrow().get(&expr) {
                    Some(distance) => {
                        self.environment.borrow_mut().assign_at(
                            *distance,
                            name.lexeme,
                            evaluated_value.clone(),
                        );
                    }
                    None => {
                        self.globals
                            .borrow_mut()
                            .assign(name.lexeme, evaluated_value.clone())?;
                    }
                }
                Ok(evaluated_value)
            }
            Expr::Binary(left, operator, right) => self.binary(*left, operator, *right),
            Expr::Grouping(group_expr) => self.evaluate(*group_expr),
            Expr::Urnary(operator, right) => self.urnary(operator, *right),
            Expr::StringLiteral(string_literal) => {
                Ok(Value::from(string_literal.clone().to_string()))
            }
            Expr::NumberLiteral(number_literal) => Ok(Value::from(number_literal)),
            Expr::NilLiteral => Ok(Value::Nil),
            Expr::TrueLiteral => Ok(Value::Bool(true)),
            Expr::FalseLiteral => Ok(Value::Bool(false)),
            Expr::Variable(token) => Ok(self.lookup_variable(token, &expr)?),
            Expr::Logical(left, operator, right) => {
                let left_value = self.evaluate(*left)?;
                if TokenType::Or == operator.token_type && self.is_truthy(left_value.clone()) {
                    Ok(left_value)
                } else if TokenType::And == operator.token_type
                    && !self.is_truthy(left_value.clone())
                {
                    Ok(left_value)
                } else {
                    self.evaluate(*right)
                }
            }
            Expr::Call(callee, _, args) => {
                let callee = self.evaluate(*callee)?;
                let mut arguments = Vec::new();
                for arg in args {
                    arguments.push(self.evaluate(arg)?);
                }
                callee.call(self, arguments)
            }
            Expr::Get(object, name) => {
                let object = self.evaluate(*object)?;
                if let Value::Instance(instance) = object {
                    instance.get(name.lexeme)
                } else {
                    Err("Only instances have properties".to_string())
                }
            }
            Expr::Set(object, name, value) => {
                let evaluated_object = self.evaluate(*object.clone())?;
                match evaluated_object {
                    Value::Instance(mut instance) => {
                        let value = self.evaluate(*value)?;
                        instance.set(name.lexeme.clone(), value.clone());
                        if let Expr::Variable(object_name) = *object {
                            self.environment
                                .borrow_mut()
                                .assign(object_name.lexeme, Value::Instance(instance.clone()))?;
                        }
                        Ok(value)
                    }
                    _ => Err("Only instances have fields".to_string()),
                }
            }
            Expr::This(keyword) => self.lookup_variable(keyword, &expr),
            Expr::Super(_, method) => {
                let distance = *self.locals.borrow().get(&expr).unwrap();
                let superclass = self
                    .environment
                    .borrow()
                    .get_at(distance, "super".to_string())?;
                let object = self
                    .environment
                    .borrow()
                    .get_at(distance - 1, "this".to_string())?;
                if let Value::Class(klass) = superclass {
                    if let Value::Instance(instance) = object {
                        match klass.find_method(&method.lexeme) {
                            Some(klass_method) => klass_method.bind(&instance),
                            None => Err(format!("Undefined property {}", method.lexeme)),
                        }
                    } else {
                        Err("Something went wrong".to_string())
                    }
                } else {
                    Err("Something went wrong".to_string())
                }
            }
        }
    }

    fn urnary(&mut self, operator: Token, right: Expr) -> Result<Value, String> {
        let right_value = self.evaluate(right)?;
        match operator.token_type {
            TokenType::Minus => match right_value {
                Value::Number(right_number_value) => Ok(Value::from(-right_number_value)),
                _ => Err("unexpected value".to_string()),
            },
            TokenType::Bang => Ok(Value::from(!self.is_truthy(right_value))),
            _ => Err("unexpected token".to_string()),
        }
    }

    fn binary(&mut self, left: Expr, operator: Token, right: Expr) -> Result<Value, String> {
        let left_value = self.evaluate(left)?;
        let right_value = self.evaluate(right)?;
        match operator.token_type {
            TokenType::Greater => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value > right_number_value))
                }
                _ => Err("unexpected value".to_string()),
            },
            TokenType::GreaterEqual => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value >= right_number_value))
                }
                _ => Err("unexpected value".to_string()),
            },
            TokenType::Less => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value < right_number_value))
                }
                _ => Err("unexpected value".to_string()),
            },
            TokenType::LessEqual => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value <= right_number_value))
                }
                _ => Err("unexpected value".to_string()),
            },
            TokenType::Slash => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value / right_number_value))
                }
                _ => Err("unexpected value".to_string()),
            },
            TokenType::Star => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value * right_number_value))
                }
                _ => Err("unexpected value".to_string()),
            },
            TokenType::Minus => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value - right_number_value))
                }
                _ => Err("unexpected value".to_string()),
            },
            TokenType::Plus => match (left_value, right_value) {
                (Value::Number(left_number_value), Value::Number(right_number_value)) => {
                    Ok(Value::from(left_number_value + right_number_value))
                }
                (Value::String(left_string_value), Value::String(right_string_value)) => Ok(
                    Value::from([left_string_value, right_string_value].join("")),
                ),
                _ => Err("unexpected value".to_string()),
            },
            TokenType::BangEqual => Ok(Value::from(!self.is_equal(left_value, right_value))),
            TokenType::EqualEqual => Ok(Value::from(self.is_equal(left_value, right_value))),
            _ => Err("unexpected token".to_string()),
        }
    }

    fn is_truthy(&self, value: Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Bool(value) => value,
            _ => true,
        }
    }

    fn is_equal(&self, left_value: Value, right_value: Value) -> bool {
        left_value == right_value
    }

    pub fn resolve(&self, expression: &Expr, depth: usize) {
        self.locals.borrow_mut().insert(expression.clone(), depth);
    }

    fn lookup_variable(&self, name: Token, expr: &Expr) -> Result<Value, String> {
        match self.locals.borrow().get(expr) {
            Some(distance) => self.environment.borrow().get_at(*distance, name.lexeme),
            None => self.globals.borrow().get(name.lexeme),
        }
    }
}
