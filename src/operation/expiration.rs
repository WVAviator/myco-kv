use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub struct Expiration {
    pub key: String,
    pub timestamp: i64,
}

impl Expiration {
    pub fn new(key: String, timestamp: i64) -> Self {
        Expiration { key, timestamp }
    }
}

impl Ord for Expiration {
    fn cmp(&self, other: &Self) -> Ordering {
        other.timestamp.cmp(&self.timestamp)
    }
}
