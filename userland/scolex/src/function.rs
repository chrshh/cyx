use std::{cell::RefCell, rc::Rc};

use crate::{
    callable::{Callable, CallableObject},
    environment::Environment,
    error::RuntimeError,
    interpreter::Interpreter,
    parser::Function,
    token::Literal,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCallable {
    pub declaration: Function,
}

impl Callable for FunctionCallable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
    ) -> Result<Literal, RuntimeError> {
        let environment = Rc::new(RefCell::new(
            Environment::with_enclosing(Rc::clone(
                &interpreter.globals,
            )),
        ));

        for (param, argument) in
            self.declaration.params.iter().zip(arguments)
        {
            environment
                .borrow_mut()
                .define(param.lexeme.clone(), argument);
        }

        interpreter.execute_block_stmt(
            self.declaration.body.clone(),
            environment,
        )?;

        Ok(Literal::Null)
    }

    fn clone_box(&self) -> Box<dyn Callable> {
        Box::new(self.clone())
    }

    fn eq_box(&self, _other: &dyn Callable) -> bool {
        false
    }
}

impl CallableObject {
    pub fn new_function(declaration: Function) -> CallableObject {
        CallableObject {
            arity: declaration.params.len(),
            callable: Box::new(FunctionCallable {
                declaration: declaration.clone(),
            }),
            declaration: Box::new(declaration),
        }
    }
}
