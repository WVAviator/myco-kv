#[derive(Debug, Clone, PartialEq)]
pub enum RadixError {
    InvalidKey(String),
    KeyNotFound(String),
    SerializationFailure,
}
