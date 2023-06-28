use std::collections::HashMap;

struct RadixNode {
    children: HashMap<String, RadixNode>,
    key: String,
    value: Option<String>,
}

impl RadixNode {
    pub fn new(key: String) -> Self {
        RadixNode {
            children: HashMap::new(),
            key,
            value: None,
        }
    }
}

struct RadixTree {
    root: RadixNode,
}

impl RadixTree {
    pub fn new() -> Self {
        RadixTree {
            root: RadixNode::new(String::from("_")),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        let mut current = &self.root;
        let parts = key.split(".");
        for part in parts {
            let child = current.children.get(part);
            match child {
                Some(child) => current = child,
                None => return None,
            }
        }
        current.value.as_ref()
    }

    pub fn get_all(&self, key: &str) -> Vec<&String> {
        let mut current = &self.root;
        let parts = key.split(".");
        let mut results = Vec::new();
        for part in parts {
            if part == "*" {
                break;
            }
            let child = current.children.get(part);
            match child {
                Some(child) => current = child,
                None => return results,
            }
        }

        let mut queue = Vec::new();
        queue.push(current);

        while queue.len() > 0 {
            let node = queue.pop().unwrap();
            if node.value.is_some() {
                results.push(node.value.as_ref().unwrap());
            }
            for child in node.children.values() {
                queue.push(child);
            }
        }

        results
    }

    pub fn put(&mut self, key: String, value: String) {
        let mut current = &mut self.root;
        let parts = key.split(".");
        for part in parts {
            if current.children.contains_key(part) {
                current = current.children.get_mut(part).unwrap();
            } else {
                let child = RadixNode::new(part.to_string());
                current.children.insert(part.to_string(), child);
                current = current.children.get_mut(part).unwrap();
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
    use super::*;

    #[test]
    fn puts_and_gets_single_value() {
        let mut radix = RadixTree::new();
        radix.put("key".to_string(), "value".to_string());

        assert_eq!(radix.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn puts_and_gets_nested_single_value() {
        let mut radix = RadixTree::new();
        radix.put("key.abc.def".to_string(), "value".to_string());

        assert_eq!(radix.get("key.abc.def"), Some(&"value".to_string()));
    }

    #[test]
    fn puts_and_gets_multiple_values() {
        let mut radix = RadixTree::new();
        radix.put("key.a".to_string(), "value1".to_string());
        radix.put("key.b".to_string(), "value2".to_string());
        radix.put("key.c".to_string(), "value3".to_string());

        assert_eq!(
            radix.get_all("key.*").sort(),
            vec![
                &"value1".to_string(),
                &"value2".to_string(),
                &"value3".to_string()
            ]
            .sort()
        );
    }
}
