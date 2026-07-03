use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{Token, token::Literal};

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

    pub fn from(e: Environment) -> Environment {
        Environment {
            enclosing: e.enclosing,
            values: e.values,
        }
    }

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

    pub fn get(&self, name: &Token) -> Literal {
        if let Some(v) = self.values.get(&name.lexeme) {
            return v.clone();
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }
        panic!("Undefined variable: '{}'", name.lexeme);
    }

    pub fn assign(&mut self, name: String, value: Literal) {
        if let std::collections::hash_map::Entry::Occupied(mut e) =
            self.values.entry(name.clone())
        {
            e.insert(value);
            return;
        }

        if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value);
            return;
        }

        panic!("Undefined variable: '{}'", name);
    }
}
