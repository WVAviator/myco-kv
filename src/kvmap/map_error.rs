#[derive(Debug, PartialEq)]
pub enum MapError {
    KeyNotFound(String),
    InvalidKey(String),
    EmptyValue(String),
}

impl MapError {
    pub fn message(&self) -> String {
        match self {
            MapError::KeyNotFound(key) => format!("Key not found: {}", key),
            MapError::InvalidKey(key) => format!("Invalid key: {}", key),
            MapError::EmptyValue(key) => format!("No value found at: {}", key),
        }
    }
}
