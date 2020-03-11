use super::object::*;
use std::cell::*;
use std::collections::HashMap;
use std::rc::*;

#[derive(Debug)]
pub struct Environment {
    store: Rc<RefCell<HashMap<String, Box<dyn Object>>>>,
}
impl Environment {
    pub fn get(&self, name: &String) -> Option<Box<dyn Object>> {
        match self.store.borrow().get(name) {
            Some(v) => Some(v.duplicate()),
            _ => None,
        }
    }
    pub fn set(&self, name: String, val: Box<dyn Object>) -> Option<Box<dyn Object>> {
        match self
            .store
            .borrow_mut()
            .insert(name.clone(), val.duplicate())
        {
            Some(_) => None,
            _ => self.get(&name),
        }
    }
}

pub fn new_environment() -> Environment {
    Environment {
        store: Rc::new(RefCell::new(HashMap::new())),
    }
}
