use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    Token, error::RuntimeError, interpreter::Interpreter,
    parser::Function, token::Literal,
};

pub trait Callable: std::fmt::Debug {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
    ) -> Result<Literal, RuntimeError>;

    fn clone_box(&self) -> Box<dyn Callable>;

    fn eq_box(&self, other: &dyn Callable) -> bool;
}

#[derive(Debug)]
pub struct CallableObject {
    pub callable: Box<dyn Callable>,
    pub arity: usize,
    pub declaration: Box<Function>,
}

impl Clone for CallableObject {
    fn clone(&self) -> Self {
        Self {
            callable: self.callable.clone_box(),
            arity: self.arity,
            declaration: self.declaration.clone(),
        }
    }
}

impl PartialEq for CallableObject {
    fn eq(&self, other: &Self) -> bool {
        self.callable.eq_box(&*other.callable)
    }
}

/* generic wrapper container */
impl CallableObject {
    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
    ) -> Result<Literal, RuntimeError> {
        self.callable.call(interpreter, arguments)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClockCallable;

impl Callable for ClockCallable {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Literal>,
    ) -> Result<Literal, RuntimeError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| {
                RuntimeError::new(
                    Token::default(),
                    format!("Time error: {}", e),
                )
            })?
            .as_secs_f64();

        Ok(Literal::Number(now))
    }

    fn clone_box(&self) -> Box<dyn Callable> {
        Box::new(self.clone())
    }

    fn eq_box(&self, _other: &dyn Callable) -> bool {
        true
    }
}

pub type Clock = CallableObject;

impl Clock {
    pub fn new_clock() -> CallableObject {
        CallableObject {
            callable: Box::new(ClockCallable),
            arity: 0,
            declaration: Box::new(Function::default()),
        }
    }
}
