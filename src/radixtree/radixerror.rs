#[derive(Debug)]
pub enum RadixError {
    InvalidKey(String),
    KeyNotFound(String),
    SerializationFailure,
}
