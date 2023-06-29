pub enum AccessType {
    Direct,
    Subtree(String),
}

impl AccessType {
    pub fn parse(key: &str) -> Self {
        let parts: Vec<&str> = key.split(".").collect();
        let last_part = parts.last().unwrap();
        if last_part == &"*" {
            let key = parts[0..parts.len() - 1].join(".");
            AccessType::Subtree(key)
        } else {
            AccessType::Direct
        }
    }
}
