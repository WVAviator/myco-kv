use crate::operation::{value::Value, Operation};
use crate::radixtree::RadixTree;
use crate::wal::WriteAheadLog;
use std::sync::{Arc, Mutex};

use self::map_error::MapError;

mod map_error;

pub struct KVMap {
    pub radix_tree: RadixTree,
    pub wal: Arc<Mutex<WriteAheadLog>>,
}

impl KVMap {
    pub fn new(wal: Arc<Mutex<WriteAheadLog>>) -> Self {
        KVMap {
            radix_tree: RadixTree::new(),
            wal,
        }
    }

    pub fn restore(&mut self) -> Result<(), MapError> {
        let line_iter = self
            .wal
            .lock()
            .unwrap()
            .read_all_lines()
            .map_err(|error| match error {
                err => MapError::RestoreError(err.message()),
            })?;

        for line in line_iter {
            let operation = Operation::parse(line.unwrap()).map_err(|error| match error {
                err => MapError::RestoreError(err.message()),
            })?;

            let result: Result<(), MapError> = match operation {
                Operation::Get(_) => Ok(()),
                Operation::Put(key, value) => {
                    if let Err(error) = self.put(key, value) {
                        return Err(MapError::RestoreError(error.message()));
                    }
                    Ok(())
                }
                Operation::Delete(key) => {
                    if let Err(error) = self.delete(&key) {
                        return Err(MapError::RestoreError(error.message()));
                    }
                    Ok(())
                }
            };

            result.map_err(|error| match error {
                err => MapError::RestoreError(err.message()),
            })?;
        }

        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Result<String, MapError> {
        let result = self.radix_tree.get(key);

        result.map_err(|_| MapError::KeyNotFound(key.to_string()))
    }

    pub fn put(&mut self, key: String, value: Value) -> Result<String, MapError> {
        let result = self.radix_tree.put(key.to_string(), value);
        result.map_err(|_| MapError::InvalidKey(key))
    }

    pub fn delete(&mut self, key: &str) -> Result<String, MapError> {
        let result = self.radix_tree.delete(key.to_string());
        result.map_err(|_| MapError::KeyNotFound(key.to_string()))
    }

    /// Process an operation and return a result.
    ///
    /// # Errors
    /// Returns a `MapError` if the key does not exist in the map.
    ///
    pub fn process_operation(&mut self, operation: Operation) -> Result<String, MapError> {
        {
            // TODO: Validate operation before writing to WAL.
            self.wal
                .lock()
                .unwrap()
                .write(&operation)
                .expect("Could not write to database file.");
        }

        match operation {
            Operation::Get(key) => self.get(&key),
            Operation::Put(key, value) => self.put(key.to_string(), value),
            Operation::Delete(key) => self.delete(&key),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    #[test]
    fn test_put_and_get() {
        let wal_mutex = Arc::new(Mutex::new(WriteAheadLog::new("log.test.txt").unwrap()));
        let mut map = super::KVMap::new(wal_mutex.clone());
        map.put("key".to_string(), Value::String("value".to_string()))
            .unwrap();

        assert_eq!(map.get("key"), Ok("\"value\"".to_string()));

        wal_mutex.lock().unwrap().clear().unwrap();
    }

    #[test]
    fn test_delete() {
        let wal_mutex = Arc::new(Mutex::new(WriteAheadLog::new("log.test.txt").unwrap()));
        let mut map = super::KVMap::new(wal_mutex.clone());
        map.put("key".to_string(), Value::String("value".to_string()))
            .unwrap();

        assert_eq!(map.delete("key"), Ok("\"value\"".to_string()));
        assert_eq!(
            map.get("key"),
            Err(MapError::KeyNotFound("key".to_string()))
        );
    }

    #[test]
    fn test_process_operation_get() {
        let wal_mutex = Arc::new(Mutex::new(WriteAheadLog::new("log.test.txt").unwrap()));
        let mut map = super::KVMap::new(wal_mutex.clone());
        map.put("key".to_string(), Value::String("value".to_string()))
            .unwrap();

        let operation = super::Operation::Get("key".to_string());

        assert_eq!(
            map.process_operation(operation),
            Ok("\"value\"".to_string())
        );
    }

    #[test]
    fn test_process_operation_put() {
        let wal_mutex = Arc::new(Mutex::new(WriteAheadLog::new("log.test.txt").unwrap()));
        let mut map = super::KVMap::new(wal_mutex.clone());

        let operation =
            super::Operation::Put("key".to_string(), Value::String("value".to_string()));
        assert_eq!(
            map.process_operation(operation),
            Ok("\"value\"".to_string())
        );

        assert_eq!(map.get("key"), Ok("\"value\"".to_string()));
    }

    #[test]
    fn test_process_operation_delete() {
        let wal_mutex = Arc::new(Mutex::new(WriteAheadLog::new("log.test.txt").unwrap()));
        let mut map = super::KVMap::new(wal_mutex.clone());
        map.put("key".to_string(), Value::String("value".to_string()))
            .unwrap();

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
        let wal_mutex = Arc::new(Mutex::new(WriteAheadLog::new("log.test.txt").unwrap()));
        let mut map = super::KVMap::new(wal_mutex.clone());

        let operation = super::Operation::Get("key".to_string());
        assert_eq!(
            map.process_operation(operation),
            Err(super::MapError::KeyNotFound("key".to_string()))
        );
    }

    #[test]
    fn test_process_operation_delete_key_not_found() {
        let wal_mutex = Arc::new(Mutex::new(WriteAheadLog::new("log.test.txt").unwrap()));
        let mut map = super::KVMap::new(wal_mutex.clone());

        let operation = super::Operation::Delete("key".to_string());
        assert_eq!(
            map.process_operation(operation),
            Err(super::MapError::KeyNotFound("key".to_string()))
        );
    }

    #[test]
    fn test_process_operation_get_multiple() {
        let wal_mutex = Arc::new(Mutex::new(WriteAheadLog::new("log.test.txt").unwrap()));
        let mut map = super::KVMap::new(wal_mutex.clone());

        map.put("key.abc".to_string(), Value::String("value1".to_string()))
            .unwrap();
        map.put("key.def".to_string(), Value::String("value2".to_string()))
            .unwrap();

        let operation = super::Operation::Get("key.*".to_string());

        let expected = json!(
            {
                "abc": "value1",
                "def": "value2"
            }
        );
        let actual = map.process_operation(operation).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&actual).unwrap();

        assert_json_eq!(expected, actual);
    }

    #[test]
    fn test_process_multiple_value_types() {
        let wal_mutex = Arc::new(Mutex::new(WriteAheadLog::new("log.test.txt").unwrap()));
        let mut map = super::KVMap::new(wal_mutex.clone());

        map.put("key.abc".to_string(), Value::String("value1".to_string()))
            .unwrap();
        map.put("key.def".to_string(), Value::Integer(123)).unwrap();
        map.put("key.ghi".to_string(), Value::Boolean(true))
            .unwrap();
        map.put("key.jkl".to_string(), Value::Null).unwrap();
        map.put("key.mno".to_string(), Value::Float(1.23)).unwrap();

        let operation = super::Operation::Get("key.*".to_string());
        let expected = json!(
            {
                "abc": "value1",
                "def": 123,
                "ghi": true,
                "jkl": null,
                "mno": 1.23
            }
        );

        let actual = map.process_operation(operation).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&actual).unwrap();

        assert_json_eq!(expected, actual);
    }
}
