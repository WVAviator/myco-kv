use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
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
