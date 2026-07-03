use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{Token, token::Literal};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Environment {
    pub enclosing: Rc<RefCell<Environment>>,
    pub values: HashMap<String, Literal>,
}

// #[derive(Debug)]
// struct EnclosingSelector<T> {
//     value: T,
// }
//
// impl<T> Deref for EnclosingSelector<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }
//
// impl<T> DerefMut for EnclosingSelector<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.value
//     }
// }

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: Rc::new(RefCell::new(Environment::default())),
            values: HashMap::new(),
        }
    }

    pub fn from(e: Environment) -> Environment {
        Environment {
            enclosing: e.enclosing,
            values: e.values,
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
            return self.enclosing.borrow().get(name);
        }

        panic!("Undefined variable '{}'", name.lexeme);
    }

    pub fn assign(&mut self, name: String, value: Literal) {
        if let std::collections::hash_map::Entry::Occupied(mut e) =
            self.values.entry(name.to_string())
        {
            Some(e.insert(value))
                .expect("Undefined variabled in environemtn.assign");
            return;
        }

        if self.enclosing != None.unwrap() {
            self.enclosing.take().assign(name, value);
            return;
        }

        panic!("Undefined variable in environment.assign");
    }
}
