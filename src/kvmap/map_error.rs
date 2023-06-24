#[derive(Debug, PartialEq)]
pub enum MapError {
    KeyNotFound(String),
}

impl MapError {
    pub fn message(&self) -> String {
        match self {
            MapError::KeyNotFound(key) => format!("Key not found: {}", key),
        }
    }
}
