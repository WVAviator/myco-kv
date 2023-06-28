use std::collections::HashMap;

pub struct RadixNode {
    pub children: HashMap<String, RadixNode>,
    pub key: String,
    pub value: Option<String>,
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
