use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{Token, error::RuntimeError, token::Literal};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    // pub fn from(e: Environment) -> Environment {
    //     Environment {
    //         enclosing: e.enclosing,
    //         values: e.values,
    //     }
    // }

    pub fn with_enclosing(
        enclosing: Rc<RefCell<Environment>>,
    ) -> Environment {
        Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Literal, RuntimeError> {
        if let Some(v) = self.values.get(&name.lexeme) {
            return Ok(v.clone());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }

        Err(RuntimeError::new(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn assign(
        &mut self,
        name: &Token,
        value: Literal,
    ) -> Result<(), RuntimeError> {
        if let std::collections::hash_map::Entry::Occupied(mut e) =
            self.values.entry(name.lexeme.clone())
        {
            e.insert(value);
            return Ok(());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }

        Err(RuntimeError::new(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme),
        ))
    }
}
