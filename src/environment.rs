use super::object::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment<'a> {
    store: HashMap<String, Box<dyn Object>>,
    outer: Option<&'a Environment<'a>>,
}
impl<'a> Environment<'a> {
    pub fn get(&self, name: &String) -> Option<&Box<dyn Object>> {
        match self.store.get(name) {
            Some(obj) => Some(obj),
            _ => match self.outer {
                Some(outer) => outer.get(name),
                _ => None,
            },
        }
    }
    pub fn set(&mut self, name: String, val: Box<dyn Object>) -> Option<&Box<dyn Object>> {
        match self.store.insert(name.clone(), val) {
            Some(_) => None,
            _ => self.get(&name),
        }
    }
}

pub fn new_environment<'a>() -> Environment<'a> {
    Environment {
        store: HashMap::new(),
        outer: None,
    }
}

pub fn new_enclosed_environment<'a>(outer: &'a Environment) -> Environment<'a> {
    let mut env = new_environment();
    env.outer = Some(outer);
    env
}
