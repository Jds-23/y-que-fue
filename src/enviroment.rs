use std::collections::HashMap;

use crate::literal::Literal;

pub struct Enviroment {
    storage: HashMap<String, Literal>,
    pub enviroment: Option<Box<Self>>,
}

impl Enviroment {
    pub fn new(enviroment: Option<Box<Self>>) -> Self {
        Enviroment {
            storage: HashMap::new(),
            enviroment,
        }
    }

    pub fn get(&self, k: &String) -> Option<&Literal> {
        match self.storage.get(k) {
            None => match &self.enviroment {
                None => None,
                Some(enviroment) => enviroment.get(k),
            },
            literal => literal,
        }
    }

    pub fn declare(&mut self, k: String, val: Literal) {
        self.storage.insert(k, val);
    }

    pub fn assign(&mut self, k: String, val: Literal) -> bool {
        if self.storage.contains_key(&k) {
            self.storage.insert(k, val);
            return true;
        }
        match &mut self.enviroment {
            Some(enviroment) => enviroment.assign(k, val),
            None => false,
        }
    }
}
