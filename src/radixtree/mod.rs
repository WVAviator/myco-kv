use std::collections::HashMap;

use self::{accesstype::AccessType, radixnode::RadixNode};

mod accesstype;
mod radixnode;

pub struct RadixTree {
    root: RadixNode,
}

impl RadixTree {
    pub fn new() -> Self {
        RadixTree {
            root: RadixNode::new(String::from("_")),
        }
    }

    pub fn get(&self, key: &str) -> HashMap<String, String> {
        let access_type = AccessType::parse(key);
        let mut results: HashMap<String, String> = HashMap::new();

        match access_type {
            AccessType::Direct => {
                let mut current = &self.root;
                let parts = key.split(".");
                for part in parts {
                    let child = current.children.get(part);
                    match child {
                        Some(child) => current = child,
                        None => return HashMap::new(),
                    }
                }

                if let Some(value) = &current.value {
                    results.insert(key.to_string(), value.to_string());
                }
                results
            }
            AccessType::FullSubtree(key) => {
                let mut current = &self.root;
                let parts = key.split(".");
                for part in parts {
                    let child = current.children.get(part);
                    match child {
                        Some(child) => current = child,
                        None => return HashMap::new(),
                    }
                }
                let mut queue = Vec::new();
                queue.push(current);

                while queue.len() > 0 {
                    let node = queue.pop().unwrap();
                    if let Some(value) = &node.value {
                        results.insert(node.key.to_string(), value.to_string());
                    }
                    for child in node.children.values() {
                        queue.push(child);
                    }
                }
                results
            }
            AccessType::PartialSubtree(key, depth) => {
                let mut current = &self.root;
                let parts = key.split(".");
                for part in parts {
                    let child = current.children.get(part);
                    match child {
                        Some(child) => current = child,
                        None => return HashMap::new(),
                    }
                }
                let mut queue = Vec::new();
                queue.push((current, 0));

                while queue.len() > 0 {
                    let (node, current_depth) = queue.pop().unwrap();
                    if let Some(value) = &node.value {
                        results.insert(node.key.to_string(), value.to_string());
                    }
                    if current_depth >= depth {
                        continue;
                    }
                    for child in node.children.values() {
                        queue.push((child, current_depth + 1));
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

    pub fn delete(&mut self, key: String) -> Option<String> {
        let mut current = &mut self.root;
        let parts = key.split(".");
        for part in parts {
            if current.children.contains_key(part) {
                current = current.children.get_mut(part).unwrap();
            } else {
                return None;
            }
        }

        let value = current.value.clone().unwrap();
        current.value = None;

        Some(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puts_and_gets_single_value() {
        let mut radix = RadixTree::new();
        radix.put("key".to_string(), "value".to_string());

        let expected = HashMap::from([("key".to_string(), "value".to_string())]);
        assert_eq!(radix.get("key"), expected);
    }

    #[test]
    fn puts_and_gets_nested_single_value() {
        let mut radix = RadixTree::new();
        radix.put("key.abc.def".to_string(), "value".to_string());

        let expected = HashMap::from_iter([("key.abc.def".to_string(), "value".to_string())]);

        assert_eq!(radix.get("key.abc.def"), expected);
    }

    #[test]
    fn puts_and_gets_multiple_values() {
        let mut radix = RadixTree::new();
        radix.put("key.a".to_string(), "value1".to_string());
        radix.put("key.b".to_string(), "value2".to_string());
        radix.put("key.c".to_string(), "value3".to_string());

        let expected = HashMap::from([
            ("key.a".to_string(), "value1".to_string()),
            ("key.b".to_string(), "value2".to_string()),
            ("key.c".to_string(), "value3".to_string()),
        ]);

        let actual = radix.get("key.*");

        assert_eq!(actual, expected);
    }

    #[test]
    fn puts_and_gets_nested_subtree() {
        let mut radix = RadixTree::new();
        radix.put("key.a".to_string(), "value1".to_string());
        radix.put("key.b".to_string(), "value2".to_string());
        radix.put("key.b.a".to_string(), "value3".to_string());

        let expected = HashMap::from([
            ("key.a".to_string(), "value1".to_string()),
            ("key.b".to_string(), "value2".to_string()),
            ("key.b.a".to_string(), "value3".to_string()),
        ]);

        let actual = radix.get("key.*");

        assert_eq!(actual, expected);
    }

    #[test]
    fn puts_and_gets_partial_subtree() {
        let mut radix = RadixTree::new();
        radix.put("key.a".to_string(), "value1".to_string());
        radix.put("key.b".to_string(), "value2".to_string());
        radix.put("key.b.a".to_string(), "value3".to_string());

        let expected = HashMap::from([
            ("key.a".to_string(), "value1".to_string()),
            ("key.b".to_string(), "value2".to_string()),
        ]);

        let actual = radix.get("key.*1");

        assert_eq!(actual, expected);
    }
}
