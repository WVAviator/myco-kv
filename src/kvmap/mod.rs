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

    /// Process an operation and return a result.
    ///
    /// # Examples
    ///
    /// ```
    /// use myco_kv::kvmap::KVMap;
    /// use myco_kv::operation::Operation;
    ///
    /// let mut map = KVMap::new();
    /// map.put("key".to_string(), "value".to_string());
    ///
    /// let operation = Operation::Get("key".to_string());
    /// let result = map.process_operation(operation);
    ///
    /// assert_eq!(result, Ok("\"value\"".to_string()));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `MapError` if the key does not exist in the map.
    ///
    pub fn process_operation(&mut self, operation: Operation) -> Result<String, MapError> {
        match operation {
            Operation::Get(key) => self
                .get(&key)
                .ok_or(MapError::KeyNotFound(key))
                .map(|value| format!("\"{}\"", value).to_string()),
            Operation::Put(key, value) => {
                self.put(key, value);
                Ok("OK".to_string())
            }
            Operation::Delete(key) => self
                .delete(&key)
                .ok_or(MapError::KeyNotFound(key))
                .map(|value| format!("\"{}\"", value).to_string()),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_get() {
        let mut map = super::KVMap::new();
        map.put("key".to_string(), "value".to_string());

        assert_eq!(map.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_put() {
        let mut map = super::KVMap::new();
        map.put("key".to_string(), "value".to_string());

        assert_eq!(map.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_delete() {
        let mut map = super::KVMap::new();
        map.put("key".to_string(), "value".to_string());

        assert_eq!(map.delete("key"), Some("value".to_string()));
        assert_eq!(map.get("key"), None);
    }

    #[test]
    fn test_process_operation_get() {
        let mut map = super::KVMap::new();
        map.put("key".to_string(), "value".to_string());

        let operation = super::Operation::Get("key".to_string());
        assert_eq!(
            map.process_operation(operation),
            Ok("\"value\"".to_string())
        );
    }

    #[test]
    fn test_process_operation_put() {
        let mut map = super::KVMap::new();

        let operation = super::Operation::Put("key".to_string(), "value".to_string());
        assert_eq!(map.process_operation(operation), Ok("OK".to_string()));
        assert_eq!(map.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_process_operation_delete() {
        let mut map = super::KVMap::new();
        map.put("key".to_string(), "value".to_string());

        let operation = super::Operation::Delete("key".to_string());
        assert_eq!(
            map.process_operation(operation),
            Ok("\"value\"".to_string())
        );
        assert_eq!(map.get("key"), None);
    }

    #[test]
    fn test_process_operation_get_key_not_found() {
        let mut map = super::KVMap::new();

        let operation = super::Operation::Get("key".to_string());
        assert_eq!(
            map.process_operation(operation),
            Err(super::MapError::KeyNotFound("key".to_string()))
        );
    }

    #[test]
    fn test_process_operation_delete_key_not_found() {
        let mut map = super::KVMap::new();

        let operation = super::Operation::Delete("key".to_string());
        assert_eq!(
            map.process_operation(operation),
            Err(super::MapError::KeyNotFound("key".to_string()))
        );
    }
}
