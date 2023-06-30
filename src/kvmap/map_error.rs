#[derive(Debug, PartialEq)]
pub enum MapError {
    KeyNotFound(String),
    InvalidKey(String),
}

impl MapError {
    pub fn message(&self) -> String {
        match self {
            MapError::KeyNotFound(key) => format!("Key not found: {}", key),
            MapError::InvalidKey(key) => format!("Invalid key: {}", key),
        }
    }
}
