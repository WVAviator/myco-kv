use crate::eventbroker::event::Event;
use crate::eventbroker::EventBroker;
use crate::operation::Operation;
use crate::radixtree::RadixTree;

use self::map_error::MapError;

mod map_error;

pub struct KVMap {
    pub radix_tree: RadixTree,
    event_broker: EventBroker,
}

impl KVMap {
    pub fn new(event_broker: EventBroker) -> Self {
        KVMap {
            radix_tree: RadixTree::new(),
            event_broker,
        }
    }

    pub fn get(&self, key: &str) -> Result<String, MapError> {
        let result = self.radix_tree.get(key);

        match result {
            Ok(value) => {
                let json = serde_json::to_string(&value).unwrap();

                let event = Event::Get {
                    key: key.to_string(),
                    result: json.to_string(),
                };
                Ok(json)
            }
            Err(_) => Err(MapError::KeyNotFound(key.to_string())),
        }
    }

    pub fn put(&mut self, key: String, value: String) -> Result<(), MapError> {
        let result = self.radix_tree.put(key.to_string(), value.to_string());
        match result {
            Ok(_) => {
                let event = Event::Put {
                    key: key.to_string(),
                    value: value.to_string(),
                };
                self.event_broker.publish(&event);
                Ok(())
            }
            Err(_) => Err(MapError::InvalidKey(key)),
        }
    }

    pub fn delete(&mut self, key: &str) -> Result<String, MapError> {
        let result = self.radix_tree.delete(key.to_string());

        match result {
            Ok(value) => {
                let event = Event::Delete {
                    key: key.to_string(),
                };
                self.event_broker.publish(&event);
                Ok(value)
            }
            Err(_) => Err(MapError::KeyNotFound(key.to_string())),
        }
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
            Operation::Get(key) => self.get(&key),
            Operation::Put(key, value) => {
                self.put(key.to_string(), value.to_string())?;
                Ok("OK".to_string())
            }
            Operation::Delete(key) => self.delete(&key),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::eventbroker::EventBroker;
    use serde_json::map;

    #[test]
    fn test_put_and_get() {
        let mut map = super::KVMap::new(EventBroker::new());
        map.put("key".to_string(), "value".to_string());

        let expected = r#"{"key":"value"}"#.to_string();

        assert_eq!(map.get("key"), Ok(expected));
    }

    #[test]
    fn test_delete() {
        let mut map = super::KVMap::new(EventBroker::new());
        map.put("key".to_string(), "value".to_string());

        assert_eq!(map.delete("key"), Ok("value".to_string()));
        assert_eq!(
            map.get("key"),
            Err(MapError::KeyNotFound("key".to_string()))
        );
    }

    #[test]
    fn test_process_operation_get() {
        let mut map = super::KVMap::new(EventBroker::new());
        map.put("key".to_string(), "value".to_string());

        let operation = super::Operation::Get("key".to_string());

        let expected = r#"{"key":"value"}"#.to_string();

        assert_eq!(map.process_operation(operation), Ok(expected));
    }

    #[test]
    fn test_process_operation_put() {
        let mut map = super::KVMap::new(EventBroker::new());

        let operation = super::Operation::Put("key".to_string(), "value".to_string());
        assert_eq!(map.process_operation(operation), Ok("OK".to_string()));

        let expected = r#"{"key":"value"}"#.to_string();

        assert_eq!(map.get("key"), Ok(expected));
    }

    #[test]
    fn test_process_operation_delete() {
        let mut map = super::KVMap::new(EventBroker::new());
        map.put("key".to_string(), "value".to_string()).unwrap();

        let operation = super::Operation::Delete("key".to_string());
        assert_eq!(
            map.process_operation(operation),
            Ok("\"value\"".to_string())
        );
        assert_eq!(
            map.get("key"),
            Err(MapError::KeyNotFound("key".to_string()))
        );
    }

    #[test]
    fn test_process_operation_get_key_not_found() {
        let mut map = super::KVMap::new(EventBroker::new());

        let operation = super::Operation::Get("key".to_string());
        assert_eq!(
            map.process_operation(operation),
            Err(super::MapError::KeyNotFound("key".to_string()))
        );
    }

    #[test]
    fn test_process_operation_delete_key_not_found() {
        let mut map = super::KVMap::new(EventBroker::new());

        let operation = super::Operation::Delete("key".to_string());
        assert_eq!(
            map.process_operation(operation),
            Err(super::MapError::KeyNotFound("key".to_string()))
        );
    }

    #[test]
    fn test_process_operation_get_multiple() {
        let mut map = super::KVMap::new(EventBroker::new());

        map.put("key.abc".to_string(), "value1".to_string())
            .unwrap();
        map.put("key.def".to_string(), "value2".to_string())
            .unwrap();

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
