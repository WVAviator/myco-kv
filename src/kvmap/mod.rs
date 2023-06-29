use crate::operation::Operation;
use crate::radixtree::RadixTree;
use std::collections::HashMap;

use self::map_error::MapError;

mod map_error;

pub struct KVMap {
    pub radix_tree: RadixTree,
}

impl KVMap {
    pub fn new() -> Self {
        KVMap {
            radix_tree: RadixTree::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let values: HashMap<String, String> = self.radix_tree.get(key);
        if values.len() > 0 {
            serde_json::to_string(&values).ok()
        } else {
            None
        }
    }

    pub fn put(&mut self, key: String, value: String) {
        self.radix_tree.put(key, value);
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.radix_tree.delete(key.to_string())
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
    /// assert_eq!(result, Ok(r#"{"key":"value"}"#.to_string()));
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
                .map(|value| value.to_string()),
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
    use serde_json::map;

    #[test]
    fn test_put_and_get() {
        let mut map = super::KVMap::new();
        map.put("key".to_string(), "value".to_string());

        let expected = r#"{"key":"value"}"#.to_string();

        assert_eq!(map.get("key"), Some(expected));
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

        let expected = r#"{"key":"value"}"#.to_string();

        assert_eq!(map.process_operation(operation), Ok(expected));
    }

    #[test]
    fn test_process_operation_put() {
        let mut map = super::KVMap::new();

        let operation = super::Operation::Put("key".to_string(), "value".to_string());
        assert_eq!(map.process_operation(operation), Ok("OK".to_string()));

        let expected = r#"{"key":"value"}"#.to_string();

        assert_eq!(map.get("key"), Some(expected));
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

    #[test]
    fn test_process_operation_get_multiple() {
        let mut map = super::KVMap::new();

        map.put("key.abc".to_string(), "value1".to_string());
        map.put("key.def".to_string(), "value2".to_string());

        let operation = super::Operation::Get("key.*".to_string());

        let expected = r#"{"key.abc":"value1","key.def":"value2"}"#.to_string();
        let actual = map.process_operation(operation).unwrap();

        let expected_map: map::Map<String, serde_json::Value> =
            serde_json::from_str(&expected).unwrap();
        let actual_map: map::Map<String, serde_json::Value> =
            serde_json::from_str(&actual).unwrap();

        assert_eq!(expected_map, actual_map);
    }
}
