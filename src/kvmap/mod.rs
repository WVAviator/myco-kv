use crate::operation::Operation;
use std::collections::HashMap;

use self::map_error::MapError;

mod map_error;

pub struct KVMap {
    pub map: HashMap<String, String>,
}

impl KVMap {
    pub fn new() -> Self {
        KVMap {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    pub fn put(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }

    pub fn process_operation(&mut self, operation: Operation) -> Result<String, MapError> {
        match operation {
            Operation::Get(key) => self
                .get(&key)
                .ok_or(MapError::KeyNotFound(key))
                .map(|value| value.to_string()),
            Operation::Put(key, value) => {
                self.put(key, value);
                Ok("".to_string())
            }
            Operation::Delete(key) => self
                .delete(&key)
                .ok_or(MapError::KeyNotFound(key))
                .map(|value| value.to_string()),
        }
    }
}
