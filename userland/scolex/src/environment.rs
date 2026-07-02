use std::{collections::HashMap, rc::Rc};

use crate::{Token, token::Literal};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Environment {
    pub enclosing: Rc<Environment>,
    pub values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: Environment::default().into(),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Literal {
        if self.values.contains_key(&name.lexeme) {
            return self
                .values
                .get(&name.lexeme)
                .cloned()
                .expect("Undefined variable");
        }

        if self.enclosing != None.unwrap() {
            return self.enclosing.get(name);
        }

        panic!("Undefined variable '{}'", name.lexeme);
    }

    pub fn assign(&mut self, name: String, value: Literal) {
        if self.values.contains_key(&name) {
            self.values
                .insert(name, value)
                .expect("Undefined variabled in environemtn.assign");
            return;
        }

        if self.enclosing != None.unwrap() {
            self.enclosing.assign(name, value);
            return;
        }

        panic!("Undefined variable in environment.assign");
    }
}
