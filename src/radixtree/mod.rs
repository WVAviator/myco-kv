mod accesstype;
mod radixnode;
use self::{accesstype::AccessType, radixnode::RadixNode};
use std::collections::HashMap;

struct RadixTree {
    root: RadixNode,
}

impl RadixTree {
    pub fn new() -> Self {
        RadixTree {
            root: RadixNode::new(String::from("_")),
        }
    }

    pub fn get(&self, key: &str) -> Vec<HashMap<String, String>> {
        let access_type = AccessType::parse(key);

        match access_type {
            AccessType::Direct => {
                let mut current = &self.root;
                let parts = key.split(".");
                for part in parts {
                    let child = current.children.get(part);
                    match child {
                        Some(child) => current = child,
                        None => return Vec::new(),
                    }
                }
                let mut results = Vec::new();
                if let Some(value) = &current.value {
                    let mut result = HashMap::new();
                    result.insert(key.to_string(), value.to_string());
                    results.push(result);
                }
                results
            }
            AccessType::Subtree(key) => {
                let mut current = &self.root;
                let parts = key.split(".");
                for part in parts {
                    let child = current.children.get(part);
                    match child {
                        Some(child) => current = child,
                        None => return Vec::new(),
                    }
                }
                let mut queue = Vec::new();
                queue.push(current);

                let mut results = Vec::new();

                while queue.len() > 0 {
                    let node = queue.pop().unwrap();
                    if let Some(value) = &node.value {
                        let mut result = HashMap::new();
                        result.insert(node.key.to_string(), value.to_string());
                        results.push(result);
                    }
                    for child in node.children.values() {
                        queue.push(child);
                    }
                }
                results
            }
        }
    }

    pub fn put(&mut self, key: String, value: String) {
        let mut current = &mut self.root;
        let parts: Vec<&str> = key.split(".").collect();
        for (i, part) in parts.iter().enumerate() {
            if current.children.contains_key(*part) {
                current = current.children.get_mut(*part).unwrap();
            } else {
                let child = RadixNode::new(parts[..i + 1].join("."));
                current.children.insert(part.to_string(), child);
                current = current.children.get_mut(*part).unwrap();
            }
        }
        current.value = Some(value);
    }

    pub fn delete(&mut self, key: String) {
        let mut current = &mut self.root;
        let parts = key.split(".");
        for part in parts {
            if current.children.contains_key(part) {
                current = current.children.get_mut(part).unwrap();
            } else {
                panic!("Key not found.")
            }
        }

        current.value = None;
    }
}

#[cfg(test)]
mod test {
    use crate::assert_vec_hashmap_eq;

    use super::*;

    #[test]
    fn puts_and_gets_single_value() {
        let mut radix = RadixTree::new();
        radix.put("key".to_string(), "value".to_string());

        let expected = vec![HashMap::from_iter(vec![(
            "key".to_string(),
            "value".to_string(),
        )])];
        assert_eq!(radix.get("key"), expected);
    }

    #[test]
    fn puts_and_gets_nested_single_value() {
        let mut radix = RadixTree::new();
        radix.put("key.abc.def".to_string(), "value".to_string());

        let expected = vec![HashMap::from_iter(vec![(
            "key.abc.def".to_string(),
            "value".to_string(),
        )])];

        assert_eq!(radix.get("key.abc.def"), expected);
    }

    #[test]
    fn puts_and_gets_multiple_values() {
        let mut radix = RadixTree::new();
        radix.put("key.a".to_string(), "value1".to_string());
        radix.put("key.b".to_string(), "value2".to_string());
        radix.put("key.c".to_string(), "value3".to_string());

        let expected: Vec<HashMap<String, String>> = vec![
            HashMap::from_iter(vec![("key.a".to_string(), "value1".to_string())]),
            HashMap::from_iter(vec![("key.b".to_string(), "value2".to_string())]),
            HashMap::from_iter(vec![("key.c".to_string(), "value3".to_string())]),
        ];

        let actual = radix.get("key.*");

        assert_vec_hashmap_eq!(actual, expected);
    }

    #[test]
    fn puts_and_gets_nested_subtree() {
        let mut radix = RadixTree::new();
        radix.put("key.a".to_string(), "value1".to_string());
        radix.put("key.b".to_string(), "value2".to_string());
        radix.put("key.b.a".to_string(), "value3".to_string());

        let expected: Vec<HashMap<String, String>> = vec![
            HashMap::from_iter(vec![("key.a".to_string(), "value1".to_string())]),
            HashMap::from_iter(vec![("key.b".to_string(), "value2".to_string())]),
            HashMap::from_iter(vec![("key.b.a".to_string(), "value3".to_string())]),
        ];

        let actual = radix.get("key.*");

        assert_vec_hashmap_eq!(actual, expected);
    }
}
