mod accesstype;
mod radixnode;
mod recursive_map;

use crate::{errors::TransactionError, operation::value::Value};

use self::{accesstype::AccessType, radixnode::RadixNode, recursive_map::RecursiveMap};
use std::collections::HashMap;

pub struct RadixTree {
    root: RadixNode,
    map: HashMap<String, Value>,
}

impl RadixTree {
    pub fn new() -> Self {
        RadixTree {
            root: RadixNode::new(String::from("_")),
            map: HashMap::new(),
        }
    }

    pub fn serialize_subtree(&self, head: &RadixNode, depth: usize) -> RecursiveMap {
        if head.children.len() == 0 {
            return match self.map.get(&head.key) {
                Some(value) => RecursiveMap::Value(value.clone()),
                None => RecursiveMap::Value(Value::Null),
            };
        }
        let mut map: HashMap<String, RecursiveMap> = HashMap::new();
        for child in head.children.keys() {
            if depth == 1 {
                if let Some(value) = self.map.get(head.children.get(child).unwrap().key.as_str()) {
                    map.insert(child.to_string(), RecursiveMap::Value(value.clone()));
                }
                continue;
            }
            let new_depth = if depth == 0 { 0 } else { depth - 1 };
            map.insert(
                child.to_string(),
                self.serialize_subtree(head.children.get(child).unwrap(), new_depth),
            );
        }

        if let Some(value) = self.map.get(&head.key) {
            map.insert(String::from("_"), RecursiveMap::Value(value.clone()));
        }

        RecursiveMap::Map(map)
    }

    pub fn get(&self, key: &str) -> Result<String, TransactionError> {
        let access_type = AccessType::parse(key);

        match access_type {
            AccessType::Direct => match self.map.get(key) {
                Some(value) => Ok(value.to_string()),
                None => Err(TransactionError::KeyNotFound(key.to_string())),
            },
            AccessType::FullSubtree(key) => {
                let mut current = &self.root;
                let parts = key.split(".");
                for part in parts {
                    let child = current.children.get(part);
                    match child {
                        Some(child) => current = child,
                        None => return Err(TransactionError::KeyNotFound(key.to_string())),
                    }
                }

                self.serialize_subtree(current, 0)
                    .to_string()
                    .map_err(|_| TransactionError::SerializationFailure)
            }
            AccessType::PartialSubtree(key, depth) => {
                let mut current = &self.root;
                let parts = key.split(".");
                for part in parts {
                    let child = current.children.get(part);
                    match child {
                        Some(child) => current = child,
                        None => return Err(TransactionError::KeyNotFound(key.to_string())),
                    }
                }

                self.serialize_subtree(current, depth)
                    .to_string()
                    .map_err(|_| TransactionError::SerializationFailure)
            }
        }
    }

    pub fn put(&mut self, key: String, value: Value) -> Result<String, TransactionError> {
        let mut current = &mut self.root;
        let value_result = value.to_string();
        let parts: Vec<&str> = key.split(".").collect();
        for (i, part) in parts.iter().enumerate() {
            if part.starts_with("*") {
                return Err(TransactionError::InvalidKey(key));
            }

            if current.children.contains_key(*part) {
                current = current.children.get_mut(*part).unwrap();
            } else {
                let child = RadixNode::new(parts[..i + 1].join("."));
                current.children.insert(part.to_string(), child);
                current = current.children.get_mut(*part).unwrap();
            }
        }
        self.map.insert(key, value);

        Ok(value_result)
    }

    pub fn remove(
        node: &mut RadixNode,
        map: &HashMap<String, Value>,
        parts: &[&str],
    ) -> Result<bool, TransactionError> {
        if parts.is_empty() {
            if node.children.is_empty() && !map.contains_key(&node.key) {
                return Ok(true);
            }
            return Ok(false);
        }

        let part = parts[0];
        let remaining_parts = &parts[1..];

        if let Some(child) = node.children.get_mut(part) {
            if Self::remove(child, map, remaining_parts)? {
                node.children.remove(part);
            }
        } else {
            return Err(TransactionError::KeyNotFound(parts.join(".").to_string()));
        }

        if node.children.is_empty() && !map.contains_key(&node.key) {
            return Ok(true);
        }

        Ok(false)
    }

    pub fn delete(&mut self, key: String) -> Result<String, TransactionError> {
        if !self.map.contains_key(&key) {
            return Err(TransactionError::KeyNotFound(key.clone()));
        }
        let value = self.map.remove(&key).unwrap();

        let parts: Vec<&str> = key.split(".").collect();

        Self::remove(&mut self.root, &self.map, &parts)?;

        Ok(value.to_string())
    }

    pub fn purge(&mut self) -> Result<(), TransactionError> {
        self.map = HashMap::new();
        self.root = RadixNode::new("_".to_string());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn puts_and_gets_single_value() {
        let mut radix = RadixTree::new();
        radix
            .put("key".to_string(), Value::String("value".to_string()))
            .unwrap();

        assert_eq!(radix.get("key").unwrap(), "\"value\"".to_string());
    }

    #[test]
    fn puts_and_gets_nested_single_value() {
        let mut radix = RadixTree::new();
        radix
            .put(
                "key.abc.def".to_string(),
                Value::String("value".to_string()),
            )
            .unwrap();

        assert_eq!(radix.get("key.abc.def").unwrap(), "\"value\"".to_string());
    }

    #[test]
    fn puts_and_gets_multiple_values() {
        let mut radix = RadixTree::new();
        radix
            .put("key.a".to_string(), Value::String("value1".to_string()))
            .unwrap();
        radix
            .put("key.b".to_string(), Value::String("value2".to_string()))
            .unwrap();
        radix
            .put("key.c".to_string(), Value::String("value3".to_string()))
            .unwrap();

        let expected = json!(
            {
                "a": "value1",
                "b": "value2",
                "c": "value3"
            }
        );

        let actual = radix.get("key.*").unwrap();
        let actual = serde_json::from_str::<serde_json::Value>(&actual).unwrap();

        assert_json_eq!(actual, expected);
    }

    #[test]
    fn puts_and_gets_nested_subtree() {
        let mut radix = RadixTree::new();
        radix
            .put("key.a".to_string(), Value::String("value1".to_string()))
            .unwrap();
        radix
            .put("key.b".to_string(), Value::String("value2".to_string()))
            .unwrap();
        radix
            .put("key.b.a".to_string(), Value::String("value3".to_string()))
            .unwrap();

        let expected = json!(
            {
                "a": "value1",
                "b": {
                    "_": "value2",
                    "a": "value3"
                }
            }
        );

        let actual = radix.get("key.*").unwrap();
        let actual = serde_json::from_str::<serde_json::Value>(&actual).unwrap();

        assert_json_eq!(actual, expected);
    }

    #[test]
    fn puts_and_gets_partial_subtree() {
        let mut radix = RadixTree::new();
        radix
            .put("key.a".to_string(), Value::String("value1".to_string()))
            .unwrap();
        radix
            .put("key.b".to_string(), Value::String("value2".to_string()))
            .unwrap();
        radix
            .put("key.b.a".to_string(), Value::String("value3".to_string()))
            .unwrap();

        let expected = json!(
            {
                "a": "value1",
                "b": "value2"
            }
        );

        let actual = radix.get("key.*1").unwrap();
        let actual = serde_json::from_str::<serde_json::Value>(&actual).unwrap();

        assert_json_eq!(actual, expected);
    }

    #[test]
    fn deletes_empty_subtrees() {
        let mut radix = RadixTree::new();
        radix
            .put("a.b.c".to_string(), Value::String("value".to_string()))
            .unwrap();
        radix
            .put("a.b.c.d.z".to_string(), Value::String("value".to_string()))
            .unwrap();
        radix
            .put(
                "a.b.c.d.e.f".to_string(),
                Value::String("value".to_string()),
            )
            .unwrap();
        radix.delete("a.b.c.d.e.f".to_string()).unwrap();
        radix.delete("a.b.c".to_string()).unwrap();

        let expected = json!(
            {
                "b": {
                    "c": {
                        "d": {
                            "z": "value"
                        }
                    }
                }
            }
        );

        let actual = radix.get("a.*").unwrap();
        let actual = serde_json::from_str::<serde_json::Value>(&actual).unwrap();

        assert_json_eq!(actual, expected);
    }

    #[test]
    fn deletes_not_found_key_in_subtree_path() {
        let mut radix = RadixTree::new();
        radix
            .put("a.b.c".to_string(), Value::String("value".to_string()))
            .unwrap();
        radix.delete("a.b.c".to_string()).unwrap();

        assert_eq!(
            radix.delete("a.b".to_string()),
            Err(TransactionError::KeyNotFound("a.b".to_string()))
        );
    }
}
