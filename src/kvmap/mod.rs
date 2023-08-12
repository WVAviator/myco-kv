use crate::atomicheap::AtomicHeap;
use crate::errors::TransactionError;
use crate::operation::expiration::Expiration;
use crate::operation::{value::Value, Operation};
use crate::radixtree::RadixTree;
use crate::wal::WriteAheadLog;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

pub struct KVMap {
    radix_tree: RadixTree,
    wal: Arc<Mutex<WriteAheadLog>>,
    exp_heap: AtomicHeap<Expiration>,
}

impl KVMap {
    pub fn new(wal: Arc<Mutex<WriteAheadLog>>) -> Self {
        KVMap {
            radix_tree: RadixTree::new(),
            wal,
            exp_heap: AtomicHeap::new(),
        }
    }

    pub fn restore(&mut self) -> Result<(), TransactionError> {
        let line_iter = self
            .wal
            .lock()
            .unwrap()
            .read_all_lines()
            .map_err(|error| match error {
                err => TransactionError::RestoreError(err.message()),
            })?;

        for line in line_iter {
            let operation = Operation::parse(line.unwrap()).map_err(|error| match error {
                err => TransactionError::RestoreError(err.message()),
            })?;

            let result: Result<(), TransactionError> = match operation {
                Operation::Get(_) => Ok(()),
                Operation::Put(key, value) => {
                    if let Err(error) = self.put(key, value) {
                        return Err(TransactionError::RestoreError(error.message()));
                    }
                    Ok(())
                }
                Operation::Delete(key) => {
                    if let Err(error) = self.delete(&key) {
                        return Err(TransactionError::RestoreError(error.message()));
                    }
                    Ok(())
                }
                Operation::ExpireAt(expiration) => {
                    if let Err(error) = self.expire_at(expiration) {
                        return Err(TransactionError::RestoreError(error.message()));
                    }
                    Ok(())
                }
                Operation::Time => Ok(()),
                Operation::Purge => Ok(()),
            };

            result.map_err(|error| match error {
                err => TransactionError::RestoreError(err.message()),
            })?;
        }

        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Result<String, TransactionError> {
        let result = self.radix_tree.get(key);

        result.map_err(|_| TransactionError::KeyNotFound(key.to_string()))
    }

    pub fn put(&mut self, key: String, value: Value) -> Result<String, TransactionError> {
        let result = self.radix_tree.put(key.to_string(), value);
        result.map_err(|_| TransactionError::InvalidKey(key))
    }

    pub fn delete(&mut self, key: &str) -> Result<String, TransactionError> {
        self.exp_heap.invalidate(&key);
        let result = self.radix_tree.delete(key.to_string());
        result.map_err(|_| TransactionError::KeyNotFound(key.to_string()))
    }

    pub fn purge(&mut self) -> Result<String, TransactionError> {
        let result = self.radix_tree.purge();
        result
            .map_err(|_| TransactionError::OperationFailure("Unable to purge data.".to_string()))?;
        self.exp_heap.clear();
        Ok(String::from("OK"))
    }

    pub fn expire_at(&mut self, expiration: Expiration) -> Result<String, TransactionError> {
        let result = self.radix_tree.get(&expiration.key);
        result.map_err(|_| TransactionError::KeyNotFound(expiration.key.clone()))?;

        self.exp_heap.push(expiration.key.clone(), expiration);

        Ok(String::from("OK"))
    }

    pub fn process_expirations(&mut self) -> Result<(), TransactionError> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        while let Some(expiration) = self.exp_heap.peek() {
            if expiration.timestamp > now {
                break;
            }

            let key = expiration.key.clone();
            self.delete(&key)?;
            self.exp_heap.pop();
        }

        Ok(())
    }

    pub fn validate(&self, operation: &Operation) -> Result<(), TransactionError> {
        match operation {
            Operation::Get(key) => {
                if let Err(_) = self.radix_tree.get(&key) {
                    return Err(TransactionError::KeyNotFound(key.to_string()));
                }
                Ok(())
            }
            Operation::Put(key, _value) => {
                for part in key.split(".") {
                    if part == "*" || part == "_" {
                        return Err(TransactionError::InvalidKey(key.to_string()));
                    }
                }
                Ok(())
            }
            Operation::Delete(key) => {
                if let Err(_) = self.radix_tree.get(&key) {
                    return Err(TransactionError::KeyNotFound(key.to_string()));
                }
                Ok(())
            }
            Operation::ExpireAt(expiration) => {
                if expiration.timestamp
                    <= SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64
                {
                    return Err(TransactionError::InvalidExpiration(expiration.timestamp));
                }

                Ok(())
            }
            Operation::Time => Ok(()),
            Operation::Purge => Ok(()),
        }
    }

    /// Process an operation and return a result.
    ///
    /// # Errors
    /// Returns a `TransactionError` if the key does not exist in the map.
    ///
    pub fn process_operation(&mut self, operation: Operation) -> Result<String, TransactionError> {
        self.process_expirations()?;
        self.validate(&operation)?;

        {
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
            Operation::ExpireAt(expiration) => self.expire_at(expiration),
            Operation::Time => Ok(SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .to_string()),
            Operation::Purge => self.purge(),
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
            Err(TransactionError::KeyNotFound("key".to_string()))
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
            Err(TransactionError::KeyNotFound("key".to_string()))
        );
    }

    #[test]
    fn test_process_operation_get_key_not_found() {
        let wal_mutex = Arc::new(Mutex::new(WriteAheadLog::new("log.test.txt").unwrap()));
        let mut map = super::KVMap::new(wal_mutex.clone());

        let operation = super::Operation::Get("key".to_string());
        assert_eq!(
            map.process_operation(operation),
            Err(super::TransactionError::KeyNotFound("key".to_string()))
        );
    }

    #[test]
    fn test_process_operation_delete_key_not_found() {
        let wal_mutex = Arc::new(Mutex::new(WriteAheadLog::new("log.test.txt").unwrap()));
        let mut map = super::KVMap::new(wal_mutex.clone());

        let operation = super::Operation::Delete("key".to_string());
        assert_eq!(
            map.process_operation(operation),
            Err(super::TransactionError::KeyNotFound("key".to_string()))
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
