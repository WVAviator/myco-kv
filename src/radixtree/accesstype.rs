pub enum AccessType {
    Direct,
    FullSubtree(String),
    PartialSubtree(String, usize),
}

impl AccessType {
    pub fn parse(key: &str) -> Self {
        let parts: Vec<&str> = key.split(".").collect();
        let last_part = parts.last().unwrap();

        if last_part == &"*" {
            let key = parts[0..parts.len() - 1].join(".");
            AccessType::FullSubtree(key)
        } else if last_part.chars().nth(0).unwrap() == '*' {
            let key = parts[0..parts.len() - 1].join(".");
            AccessType::PartialSubtree(key, last_part[1..].parse::<usize>().unwrap())
        } else {
            AccessType::Direct
        }
    }
}
