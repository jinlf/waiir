use super::object::*;
use std::cell::*;
use std::collections::HashMap;
use std::rc::*;

#[derive(Debug)]
pub struct Environment {
    store: HashMap<String, Box<dyn Object>>,
    outer: RefCell<Weak<RefCell<Environment>>>,
}
impl Environment {
    pub fn get(&self, name: &String) -> Option<Box<dyn Object>> {
        println!("env.get: {}", name);
        match self.store.get(name) {
            Some(obj) => {
                let v: Box<dyn Object> = obj.duplicate();
                Some(v)
            }
            _ => match self.outer.borrow().upgrade() {
                Some(outer) => outer.borrow().get(name),
                _ => None,
            },
        }
    }
    pub fn set(&mut self, name: String, val: Box<dyn Object>) -> Option<Box<dyn Object>> {
        println!("env.set: {}, {:?}", name, val);
        match self.store.insert(name.clone(), val) {
            Some(_) => None,
            _ => self.get(&name),
        }
    }
}

pub fn new_environment() -> Environment {
    Environment {
        store: HashMap::new(),
        outer: RefCell::new(Weak::new()),
    }
}

pub fn new_enclosed_environment(outer: &Rc<RefCell<Environment>>) -> Environment {
    let env = new_environment();
    let rc = Rc::clone(outer);
    *env.outer.borrow_mut() = Rc::downgrade(&rc);
    env
}
