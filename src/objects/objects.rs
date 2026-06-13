use core::fmt;
use std::{collections::HashMap, iter::Map, ptr::hash};


pub struct Env {
    store: HashMap<String, Object>
}


impl Env {
    pub fn new() -> Env {
        return Env{
            store: HashMap::new(),
        }
    }

    pub fn get(&self, key: String) -> Option<Object> {
        match self.store.get(&key) {
            Some(v) => return Some(v.clone()),
            None => return None
        }
    }

    pub fn set(&mut self, key: String, val: &Object) {
        self.store.insert(key, val.clone());
    }
}

#[derive(Clone)]
pub enum Object {
    Int(i32),
    Bool(bool),
    Null
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Object::Int(v) => write!(f, "{v}"),
            Object::Bool(v) => write!(f, "{v}"),
            Object::Null => write!(f, "null")
        }
    }
}


